using System;
using System.Net;
using Avalonia;
using Microsoft.Extensions.DependencyInjection;
using RemoteAccessTool.Application;
using RemoteAccessTool.Application.Remote.Interfaces;
using RemoteAccessTool.Infrastructure.Extensions;
using RemoteAccessTool.Infrastructure.Remote.Primitives;
using RemoteAccessTool.View.Views.Windows;

namespace RemoteAccessTool.View;

internal sealed class Program
{
    [STAThread]
    public static void Main(string[] args)
    {
        var assembly = typeof(Program).Assembly;

        var builder = BuilderEx.CreateDefaultBuilder(AppBuilder
            .Configure<App>()
            .UsePlatformDetect()
            .WithInterFont()
            .LogToTrace());

        builder.Services
            .AddViewModels(assembly)
            .AddWindows<RemoteWindow>(assembly)
            .AddMediatR(cfg => cfg.RegisterServicesFromAssembly(AssemblyLocator.Assembly))
            .AddPlainOptions(new RemoteOptions { Host = IPEndPoint.Parse("127.0.0.1:4141") })
            .AddTransient<IRemote, Remote>();

        var app = builder.Build();

        app.Start(args);
    }
}