using System;
using Avalonia;
using Microsoft.Extensions.DependencyInjection;
using RemoteAccessTool.Application;
using RemoteAccessTool.Infrastructure.Extensions;
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
            .AddMediatR(cfg => cfg.RegisterServicesFromAssembly(AssemblyLocator.Assembly));

        var app = builder.Build();

        app.Start(args);
    }
}