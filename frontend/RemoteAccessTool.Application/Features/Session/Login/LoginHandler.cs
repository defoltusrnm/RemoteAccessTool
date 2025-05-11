using MediatR;
using RemoteAccessTool.Application.Remote.Interfaces;
using RemoteAccessTool.Domain.Common;

namespace RemoteAccessTool.Application.Features.Session.Login;

public class LoginHandler : IRequestHandler<LoginCommand, Result<Empty, Err>>
{
    private readonly IRemote _remote;

    public LoginHandler(IRemote remote)
    {
        _remote = remote;
    }

    public async Task<Result<Empty, Err>> Handle(LoginCommand request, CancellationToken cancellationToken)
    {
        return await _remote.LoginAsync(request.ToRequest(), cancellationToken);
    }
}