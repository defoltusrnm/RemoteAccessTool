using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Microsoft.Extensions.DependencyInjection;
using RemoteAccessTool.Infrastructure.Hosting.Interfaces;

namespace RemoteAccessTool.Infrastructure.Hosting.Primitives;

public class AvaloniaHost : ISyncHost
{
    public AvaloniaHost(IServiceProvider services)
    {
        Services = services;
    }

    public void Start(string[] args)
    {
        var builder = Services.GetRequiredService<AppBuilder>();

        builder.AfterSetup(x =>
        {
            if (x.Instance?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                desktop.MainWindow = Services.GetRequiredKeyedService<Window>("MainWindow");
            }
        });
        builder.StartWithClassicDesktopLifetime(args);
    }

    public IServiceProvider Services { get; }
}