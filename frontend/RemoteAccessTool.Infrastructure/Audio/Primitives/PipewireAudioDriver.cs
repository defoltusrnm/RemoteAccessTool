using System.Buffers;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using RemoteAccessTool.Application.Audio.Interfaces;

namespace RemoteAccessTool.Infrastructure.Audio.Primitives;

public sealed class PipewireAudioDriver : IAudioDriver, IDisposable
{
    private readonly IntPtr _mainLoop;
    private readonly IntPtr _loopApi;
    private readonly IntPtr _context;
    private readonly IntPtr _core;
    private readonly IntPtr _stream;
    private IntPtr _listener;
    private readonly pw_stream_process_delegate _callback;
    private GCHandle _callbackHandle;

    public PipewireAudioDriver()
    {
        const int pwDirectionOutput = 0;
        const int pwStreamFlagAutoconnect = 1 << 0;

        PipeWireInterop.pw_init(IntPtr.Zero, IntPtr.Zero);
        _mainLoop = PipeWireInterop.pw_main_loop_new(IntPtr.Zero);
        _loopApi = PipeWireInterop.pw_main_loop_get_loop(_mainLoop);
        _context = PipeWireInterop.pw_context_new(_loopApi, IntPtr.Zero, 0);
        _core = PipeWireInterop.pw_context_connect(_context, IntPtr.Zero, 0);

        IntPtr[] paramsArray = new IntPtr[] { CreateAudioParams() };
        int paramCount = paramsArray.Length;

        _stream = PipeWireInterop.pw_stream_new(_core, "MegaShit.RemoteAccessTool.Client", IntPtr.Zero);
        var result = PipeWireInterop.pw_stream_connect(
            _stream,
            pwDirectionOutput,
            IntPtr.Zero,
            pwStreamFlagAutoconnect,
            paramsArray,
            paramCount
        );

        if (result < 0)
        {
            throw new InvalidOperationException("Failed to connect to pipewire");
        }

        _callback = OnProcess;
        _callbackHandle = GCHandle.Alloc(_callback);
        var events = new pw_stream_events
        {
            version = PipeWireInterop.PW_VERSION_STREAM_EVENTS,
            process = _callback
        };
        var listenerResult = PipeWireInterop.pw_stream_add_listener(_stream, out _listener, ref events, IntPtr.Zero);
        if (listenerResult < 0)
        {
            throw new InvalidOperationException("Failed to add listener");
        }

        var loopResult = PipeWireInterop.pw_main_loop_run(_mainLoop);
        if (loopResult < 0)
        {
            throw new InvalidOperationException("Failed to run loop");
        }

        // Dispose();
    }

    private static IntPtr CreateAudioParams()
    {
        // Here you’d build a SPA Pod describing format — for now, you can just pass IntPtr.Zero
        return IntPtr.Zero;
    }

    public void Play(IMemoryOwner<byte> sample, int size)
    {
        throw new NotImplementedException();
    }

    private void OnProcess(IntPtr data)
    {
        // var pwBufPtr = PipeWireInterop.pw_stream_dequeue_buffer(_stream);
        // if (pwBufPtr == IntPtr.Zero)
        // {
        //     Console.WriteLine("No available buffer");
        //     return;
        // }
        //
        // // 1. Marshal pw_buffer
        // var pwBuf = Marshal.PtrToStructure<pw_buffer>(pwBufPtr);
        //
        // // 2. Marshal spa_buffer
        // var spaBuf = Marshal.PtrToStructure<spa_buffer>(pwBuf.buffer);
        //
        // // 3. Get pointer to first spa_data (datas[0])
        // IntPtr spaDataPtr = Marshal.ReadIntPtr(spaBuf.datas); // datas is pointer to array
        // spa_data dataStruct = Marshal.PtrToStructure<spa_data>(spaDataPtr);
        //
        // // int bytesToWrite = (int)Math.Min(audioData.Length - audioOffset, dataStruct.maxsize);
        // // if (bytesToWrite <= 0)
        // // {
        // //     Console.WriteLine("Audio finished");
        // //     return;
        // // }
        // //
        // // // 4. Copy from audioData into the PipeWire buffer
        // // Marshal.Copy(audioData, audioOffset, dataStruct.data, bytesToWrite);
        // // audioOffset += bytesToWrite;
        //
        // // 5. Queue buffer back to PipeWire
        // PipeWireInterop.pw_stream_queue_buffer(_stream, pwBufPtr);
    }

    public void Dispose()
    {
        PipeWireInterop.pw_deinit();
        _callbackHandle.Free();
    }
}

[StructLayout(LayoutKind.Sequential)]
public struct pw_buffer
{
    public IntPtr buffer; // spa_buffer*
    public IntPtr user_data;
    public IntPtr buffer_id;
}

internal partial class PipeWireInterop
{
    public const uint PW_VERSION_STREAM_EVENTS = 0;

    private const string PipewireLib = "libpipewire-0.3.so.0";

    [LibraryImport(PipewireLib)]
    public static partial void pw_init(IntPtr argc, IntPtr argv);

    [LibraryImport(PipewireLib)]
    public static partial void pw_deinit();

    [LibraryImport(PipewireLib)]
    public static partial IntPtr pw_main_loop_new(IntPtr properties);

    [LibraryImport(PipewireLib)]
    public static partial int pw_main_loop_run(IntPtr loop);

    [LibraryImport(PipewireLib)]
    public static partial IntPtr pw_main_loop_get_loop(IntPtr mainLoop);

    [LibraryImport(PipewireLib)]
    public static partial IntPtr pw_context_new(IntPtr loop, IntPtr properties, int size);

    [LibraryImport(PipewireLib)]
    public static partial IntPtr pw_context_connect(IntPtr context, IntPtr properties, int flags);

    [LibraryImport(PipewireLib)]
    public static partial IntPtr pw_stream_new(
        IntPtr core,
        [MarshalAs(UnmanagedType.LPStr)] string name,
        IntPtr props);

    [LibraryImport(PipewireLib)]
    public static partial int pw_stream_connect(
        IntPtr stream,
        int direction,
        IntPtr target_id,
        int flags,
        IntPtr[] paramsArray,
        int n_params);

    [DllImport(PipewireLib)]
    public static extern int pw_stream_add_listener(
        IntPtr stream,
        out IntPtr listener,
        ref pw_stream_events events,
        IntPtr data
    );

    [LibraryImport(PipewireLib)]
    public static partial IntPtr pw_stream_dequeue_buffer(IntPtr stream);

    [LibraryImport(PipewireLib)]
    public static partial int pw_stream_queue_buffer(IntPtr stream, IntPtr buffer);
}

[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
public delegate void pw_stream_process_delegate(IntPtr data);

[StructLayout(LayoutKind.Sequential)]
public struct pw_stream_events
{
    public uint version;
    public pw_stream_process_delegate process;
}

[StructLayout(LayoutKind.Sequential)]
public struct spa_data
{
    public IntPtr data; // pointer to buffer memory
    public uint maxsize;
    public uint chunk; // pointer to spa_chunk (optional)
    public uint flags;
    public uint type;
    public uint fd;
}

[StructLayout(LayoutKind.Sequential)]
public struct spa_buffer
{
    public uint n_datas;
    public IntPtr datas; // points to spa_data[]
}