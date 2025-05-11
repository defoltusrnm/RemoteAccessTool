using Avalonia;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.FileProviders;
using RemoteAccessTool.Infrastructure.Hosting.Interfaces;
using RemoteAccessTool.Infrastructure.Hosting.Primitives;

namespace RemoteAccessTool.Infrastructure.Extensions;

public static class BuilderEx
{
    public static IAvaloniaHostBuilder CreateDefaultBuilder(AppBuilder appBuilder, string? appName = null)
    {
        var builder = new AvaloniaHostBuilder();

        var fp = new PhysicalFileProvider(Environment.CurrentDirectory);

        builder.Environment.ApplicationName = appName ?? "app";
        builder.Environment.ContentRootFileProvider = fp;
        builder.Environment.ContentRootPath = Environment.CurrentDirectory;
        builder.Environment.EnvironmentName = Environment.GetEnvironmentVariable("APP_ENV") ?? "LOCAL";

        builder.Services.AddSingleton<IFileProvider>(fp);
        builder.Configuration.AddJsonFile(fp, "appsettings.json", false, true);

        builder.Services.AddSingleton(appBuilder);

        return builder;
    }
}