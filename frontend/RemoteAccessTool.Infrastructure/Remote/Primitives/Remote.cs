using System.Buffers;
using System.Collections.Concurrent;
using System.Net;
using System.Net.Security;
using System.Net.Sockets;
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
    private readonly ConcurrentDictionary<uint, TaskCompletionSource<(IMemoryOwner<byte>, int)>> _taskCompletionSources;
    private readonly RemoteOptions _remoteOptions;

    public Remote(IOptions<RemoteOptions> remoteOptions)
    {
        _outboundChannel = Channel.CreateUnbounded<byte[]>();
        _taskCompletionSources = new ConcurrentDictionary<uint, TaskCompletionSource<(IMemoryOwner<byte>, int)>>();
        _remoteOptions = remoteOptions.Value;
    }

    public async Task<Result<Empty, Err>> LoginAsync(LoginRequest loginRequest, CancellationToken cancellation = default)
    {
        return await Enqueue(
            Command.Login,
            [loginRequest.Login, loginRequest.Password],
            s => s is "LOGIN_OK" ? Result.Ok() : Err.Throw<Empty>("PROTO", s.ToString()),
            cancellation
        );
    }

    private async Task<T> Enqueue<T>(
        byte command,
        string[] args, Func<ReadOnlySpan<char>, T> matFunc,
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
            return matFunc(Encoding.UTF8.GetString(response.Memory.Span[..len]));
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
            var endianBuffer = new Memory<byte>(new byte[1]);
            var numberBuffer = new Memory<byte>(new byte[sizeof(uint)]);
            while (sslStream.CanRead)
            {
                await sslStream.ReadNumberAsync(endianBuffer, numberBuffer, cancellation)
                    .MapOk(x => _taskCompletionSources.TryGetValue(x, out var val)
                        ? Result<TaskCompletionSource<(IMemoryOwner<byte>, int)>, Err>.Ok(val)
                        : Err.Throw<TaskCompletionSource<(IMemoryOwner<byte>, int)>>("INTERNAL",
                            $"Cannot get task for command {x}"))
                    .WhenOkAsync(async x =>
                    {
                        await sslStream.ReadNumberAsync(endianBuffer, numberBuffer, cancellation)
                            .WhenOkAsync(async y =>
                            {
                                var size = (int)y;
                                var memory = MemoryPool<byte>.Shared.Rent(size);
                                await sslStream.ReadExactlyAsync(memory.Memory[..size], cancellation);

                                x.TrySetResult((memory, size));
                            });
                    });
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

file static class Command
{
    public const byte Login = 1;
}

public class RemoteOptions
{
    public required IPEndPoint Host { get; init; }
}

file static class Exts
{
    public static async Task<Result<uint, Err>> ReadNumberAsync(
        this Stream stream,
        Memory<byte> endian,
        Memory<byte> buffer,
        CancellationToken cancellation = default
    )
    {
        await stream.ReadExactlyAsync(endian, cancellation);
        await stream.ReadExactlyAsync(buffer, cancellation);

        return Frame.ToUInt(endian, buffer);
    }
}

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