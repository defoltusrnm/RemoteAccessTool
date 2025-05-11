using Avalonia;
using Avalonia.Controls;
using Microsoft.Extensions.DependencyInjection;

namespace RemoteAccessTool.Infrastructure.Hosting.Interfaces;

public class AvaloniaHost : ISyncHost
{
    public AvaloniaHost(IServiceProvider services)
    {
        Services = services;
    }

    public void Start()
    {
        var builder = Services.GetRequiredService<AppBuilder>();

        builder.StartWithClassicDesktopLifetime([],
            cfg => { cfg.MainWindow = Services.GetRequiredKeyedService<Window>("MainWindow"); });
    }

    public IServiceProvider Services { get; }
}