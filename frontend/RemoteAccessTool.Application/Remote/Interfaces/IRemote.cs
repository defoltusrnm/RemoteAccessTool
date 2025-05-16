using System.Buffers;
using RemoteAccessTool.Domain.Common;

namespace RemoteAccessTool.Application.Remote.Interfaces;

public interface IRemote
{
    Task<Result<Empty, Err>> LoginAsync(LoginRequest loginRequest, CancellationToken cancellation = default);
    IAsyncEnumerable<ScreenEvent> ReceiveScreenAsync(CancellationToken cancellation = default);
}

public sealed record LoginRequest(string Login, string Password);

public record struct ScreenEvent(uint Id, uint Width, uint Height, IMemoryOwner<byte> Image, int Size);

public record struct AudioEvent(IMemoryOwner<float> Sample);