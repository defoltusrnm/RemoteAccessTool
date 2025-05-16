using System.Buffers;

namespace RemoteAccessTool.Application.Audio.Interfaces;

public interface IAudioDriver
{
    public void Play(IMemoryOwner<byte> sample, int size);
}