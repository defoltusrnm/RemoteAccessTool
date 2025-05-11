using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Microsoft.Extensions.Options;
using RemoteAccessTool.Application.Remote.Interfaces;
using RemoteAccessTool.Domain.Common;
using RemoteAccessTool.Infrastructure.Remote.Primitives;

namespace RemoteAccessTool.View.ViewModels;

public partial class RemoteViewModel : ObservableObject
{
    private readonly IOptions<RemoteOptions> _remoteOptions;
    private readonly IRemote _remote;

    [ObservableProperty] private string _loginResult = "haven't tried";

    public RemoteViewModel(IOptions<RemoteOptions> remoteOptions, IRemote remote)
    {
        _remoteOptions = remoteOptions;
        _remote = remote;

        Login = new AsyncRelayCommand(OnLogin);
    }

    public IAsyncRelayCommand Login { get; }

    private async Task OnLogin()
    {
        await _remote.LoginAsync(new LoginRequest("login", "pwd"))
            .Fold(_ => LoginResult = "Authed", err => LoginResult = err.Message);
    }
}