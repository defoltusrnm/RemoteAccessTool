using System.Buffers;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.Drawing;
using System.Net;
using System.Net.Mime;
using System.Net.Security;
using System.Net.Sockets;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading.Channels;
using Microsoft.Extensions.Options;
using RemoteAccessTool.Application.Remote.Interfaces;
using RemoteAccessTool.Domain.Common;

namespace RemoteAccessTool.Infrastructure.Remote.Primitives;

public class Remote : IRemote
{
    private static uint GlobalId;

    private Task? _serverWorker;
    private readonly Channel<byte[]> _outboundChannel;
    private readonly Channel<ScreenEvent> _screenChannel;
    private readonly Channel<AudioEvent> _audioChannel;
    private readonly ConcurrentDictionary<uint, TaskCompletionSource<(IMemoryOwner<byte>, int)>> _taskCompletionSources;
    private readonly RemoteOptions _remoteOptions;

    public Remote(IOptions<RemoteOptions> remoteOptions)
    {
        _outboundChannel = Channel.CreateUnbounded<byte[]>();
        _screenChannel = Channel.CreateUnbounded<ScreenEvent>();
        _audioChannel = Channel.CreateUnbounded<AudioEvent>();
        _taskCompletionSources = new ConcurrentDictionary<uint, TaskCompletionSource<(IMemoryOwner<byte>, int)>>();
        _remoteOptions = remoteOptions.Value;
    }

    public async Task<Result<Empty, Err>> LoginAsync(LoginRequest loginRequest,
        CancellationToken cancellation = default)
    {
        return await Enqueue(
            Command.Login,
            [loginRequest.Login, loginRequest.Password],
            s => s switch
            {
                [Event.Authenticated] => Result.Ok(),
                [Event.UnAuthenticated] => Err.Throw<Empty>("UNAUTH", "Unauthenticated"),
                _ => Err.Throw<Empty>("INTERNAL", "Unknown error"),
            },
            cancellation
        );
    }

    public async IAsyncEnumerable<ScreenEvent> ReceiveScreenAsync(
        [EnumeratorCancellation] CancellationToken cancellation = default
    )
    {
        while (await _screenChannel.Reader.WaitToReadAsync(cancellation))
        {
            yield return await _screenChannel.Reader.ReadAsync(cancellation);
        }
    }

    private async Task<T> Enqueue<T>(
        byte command,
        string[] args, Func<ReadOnlySpan<byte>, T> matFunc,
        CancellationToken cancellation = default
    )
    {
        _serverWorker ??= Task
            .Run(() => StartServer(cancellation), cancellation)
            .ContinueWith(_ => _serverWorker = null, CancellationToken.None);

        var id = Interlocked.Increment(ref GlobalId);
        var tsc = new TaskCompletionSource<(IMemoryOwner<byte>, int)>();

        _taskCompletionSources.TryAdd(id, tsc);

        await _outboundChannel.Writer.WriteAsync(Frame.CreateFrameFromStrings(command, id, args), cancellation);

        var (response, len) = await tsc.Task;
        using (response)
        {
            return matFunc(response.Memory.Span[..len]);
        }
    }

    private async Task StartServer(CancellationToken cancellation = default)
    {
        var socket = new Socket(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);
        await socket.ConnectAsync(_remoteOptions.Host, cancellation);

        var socketStream = new NetworkStream(socket, ownsSocket: true);
        var sslStream = new SslStream(socketStream);

        var sslOptions = new SslClientAuthenticationOptions();
        sslOptions.RemoteCertificateValidationCallback += (_, _, _, _) => true;
        await sslStream.AuthenticateAsClientAsync(sslOptions, cancellation);

        using var exceptionStopping = new CancellationTokenSource();
        using var cts = CancellationTokenSource.CreateLinkedTokenSource(cancellation, exceptionStopping.Token);

        await Task.WhenAny(
            WriteToServer(sslStream, cts.Token),
            ReadFromServer(sslStream, cts.Token)
        );
        await exceptionStopping.CancelAsync();
    }

    private async Task WriteToServer(SslStream sslStream, CancellationToken cancellation = default)
    {
        try
        {
            while (await _outboundChannel.Reader.WaitToReadAsync(cancellation))
            {
                var frame = await _outboundChannel.Reader.ReadAsync(cancellation);
                await sslStream.WriteAsync(new ReadOnlyMemory<byte>(frame), cancellation);
            }
        }
        catch (Exception e)
        {
        }
    }

    private async Task ReadFromServer(SslStream sslStream, CancellationToken cancellation = default)
    {
        try
        {
            var eventBuffer = new Memory<byte>(new byte[1]);
            while (sslStream.CanRead)
            {
                await sslStream.ReadExactlyAsync(eventBuffer, cancellation);
                if (Event.IsResponse(eventBuffer))
                {
                    await sslStream.ReadNumberAsync(cancellation)
                        .MapOk(x => _taskCompletionSources.TryGetValue(x, out var val)
                            ? Result<TaskCompletionSource<(IMemoryOwner<byte>, int)>, Err>.Ok(val)
                            : Err.Throw<TaskCompletionSource<(IMemoryOwner<byte>, int)>>("INTERNAL",
                                $"Cannot get task for command {x}"))
                        .WhenOk(x => { x.TrySetResult((new ManagedMemoryOwner(eventBuffer), 1)); });
                }

                switch (eventBuffer.Span[0])
                {
                    case Event.Screen:
                        await sslStream
                            .ReadScreenAsync(cancellation)
                            .WhenOkAsync(async x => await _screenChannel.Writer.WriteAsync(x, cancellation));
                        break;
                    case Event.Audio:
                        var sampleLength = await sslStream.ReadNumberAsync(cancellation: cancellation);

                        _ = sampleLength;
                        
                        break;
                }
            }
        }
        catch (Exception e)
        {
            foreach (var taskCompletionSource in _taskCompletionSources)
            {
                taskCompletionSource.Value.TrySetException(e);
            }
        }
    }
}

file class ManagedMemoryOwner : IMemoryOwner<byte>
{
    public ManagedMemoryOwner(Memory<byte> memory)
    {
        Memory = memory;
    }

    public Memory<byte> Memory { get; }

    public void Dispose()
    {
    }
}

file static class Command
{
    public const byte Login = 1;
}

file static class Event
{
    public const byte Authenticated = 1;
    public const byte UnAuthenticated = 2;
    public const byte Screen = 3;
    public const byte Audio = 4;

    public static bool IsResponse(Memory<byte> @event) => @event.Span[0] switch
    {
        Authenticated => true,
        UnAuthenticated => true,
        _ => false
    };
}

public class RemoteOptions
{
    public required IPEndPoint Host { get; init; }
}

file static class Exts
{
    public static async Task<Result<uint, Err>> ReadNumberAsync(
        this Stream stream,
        CancellationToken cancellation = default
    )
    {
        var endian = new Memory<byte>(new byte[1]);
        var buffer = new Memory<byte>(new byte[4]);

        await stream.ReadExactlyAsync(endian, cancellation);
        await stream.ReadExactlyAsync(buffer, cancellation);

        return Frame.ToUInt(endian, buffer);
    }

    public static Task<Result<Package, Err>> ReadBytesAsync(
        this Stream stream,
        CancellationToken cancellation = default
    ) => stream.ReadNumberAsync(cancellation: cancellation).MapOkAsync(async x =>
    {
        var memoryOwner = MemoryPool<byte>.Shared.Rent((int)x);
        var memory = memoryOwner.Memory[..(int)x];

        var totalRead = 0;
        while (totalRead < x)
        {
            int bytesRead = await stream.ReadAsync(memory.Slice(totalRead, (int)x - totalRead), cancellation);
            if (bytesRead == 0)
            {
                return Err.Throw<Package>("INTERNAL", "Unexpected end of stream");
            }

            totalRead += bytesRead;
        }

        return Result<Package, Err>.Ok(new Package(memoryOwner, (int)x));
    });

    public static async Task<Result<ScreenEvent, Err>> ReadScreenAsync(
        this SslStream sslStream,
        CancellationToken cancellation
    )
    {
        var screenId = await sslStream.ReadNumberAsync(cancellation);
        var width = await sslStream.ReadNumberAsync(cancellation);
        var height = await sslStream.ReadNumberAsync(cancellation);
        var image = await sslStream.ReadBytesAsync(cancellation);

        var screenRes = Result.Union(screenId, image, (u, tuple) => (
            Id: u,
            tuple.Buffer,
            tuple.Size
        ));

        var rectRes = Result.Union(width, height, (u, u1) => (
            Widht: u,
            Height: u1
        ));

        var complete = Result.Union(screenRes, rectRes, (x, y) => (
            Screen: x,
            Rect: y
        ));
        return complete.MapOk(x => new ScreenEvent(
            x.Screen.Id,
            x.Rect.Widht,
            x.Rect.Height,
            x.Screen.Buffer,
            x.Screen.Size
        ));
    }
}

file record struct Package(IMemoryOwner<byte> Buffer, int Size);

file static class Frame
{
    public static byte[] CreateFrameFromStrings(byte command, uint id, params ReadOnlySpan<string> args)
    {
        var list = new List<byte>();
        foreach (var arg in args)
        {
            list.AddRange(FromString(arg));
        }

        return [command, ..FromUInt(id), ..list];
    }

    public static byte[] FromUInt(uint value)
    {
        var bytes = BitConverter.GetBytes(value);
        var endian = (byte)(BitConverter.IsLittleEndian ? 1 : 0);

        return [endian, ..bytes];
    }

    public static byte[] FromString(string value)
    {
        var len = BitConverter.GetBytes((uint)value.Length);
        var encoded = Encoding.UTF8.GetBytes(value);
        var endian = (byte)(BitConverter.IsLittleEndian ? 1 : 0);

        return [endian, ..len, ..encoded];
    }

    public static Result<uint, Err> ToUInt(Memory<byte> endianBuffer, Memory<byte> numberBuffer)
    {
        Func<Memory<byte>, Result<uint, Err>> func = endianBuffer.Span[0] switch
        {
            0 => buffer =>
            {
                var span = buffer.Span;
                if (BitConverter.IsLittleEndian)
                {
                    span.Reverse();
                }

                return Result<uint, Err>.Ok(BitConverter.ToUInt32(span));
            },
            1 => buffer =>
            {
                var span = buffer.Span;
                if (!BitConverter.IsLittleEndian)
                {
                    span.Reverse();
                }

                return Result<uint, Err>.Ok(BitConverter.ToUInt32(span));
            },
            _ => _ => Err.Throw<uint>("PARSE", "Cannot parse uint")
        };

        return func(numberBuffer);
    }
}