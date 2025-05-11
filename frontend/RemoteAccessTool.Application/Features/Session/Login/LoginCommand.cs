using MediatR;
using RemoteAccessTool.Application.Remote.Interfaces;
using RemoteAccessTool.Domain.Common;

namespace RemoteAccessTool.Application.Features.Session.Login;

public sealed record LoginCommand(string Login, string Password) : IRequest<Result<Empty, Err>>
{
    public LoginRequest ToRequest() => new(Login, Password);
}