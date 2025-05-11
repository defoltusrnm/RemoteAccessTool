using RemoteAccessTool.Domain.Common;

namespace RemoteAccessTool.Application.Remote.Interfaces;

public interface IRemote
{
    Task<Result<Empty, Err>> LoginAsync(LoginRequest loginRequest, CancellationToken cancellation = default);
}

public sealed record LoginRequest(string Login, string Password);