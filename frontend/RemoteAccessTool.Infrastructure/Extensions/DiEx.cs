using System.Reflection;
using Avalonia.Controls;
using CommunityToolkit.Mvvm.ComponentModel;
using Microsoft.Extensions.DependencyInjection;

namespace RemoteAccessTool.Infrastructure.Extensions;

public static class DiEx
{
    public static IServiceCollection AddViewModels(
        this IServiceCollection services,
        Assembly assembly,
        IDictionary<Type, ServiceLifetime>? overrides = null
    )
        => assembly.GetTypes().Where(x => x.IsAssignableTo(typeof(ObservableObject)))
            .Aggregate(
                (services, overrides),
                (x, type) => (
                    x.overrides?.TryGetValue(type, out var lifetime) == true
                        ? lifetime switch
                        {
                            ServiceLifetime.Singleton => x.services.AddSingleton(type),
                            ServiceLifetime.Scoped => x.services.AddScoped(type),
                            _ => x.services.AddTransient(type)
                        }
                        : x.services.AddTransient(type), x.overrides)
            )
            .services;

    public static IServiceCollection AddWindows<TMainWindow>(
        this IServiceCollection services,
        Assembly assembly,
        IDictionary<Type, ServiceLifetime>? overrides = null
    ) where TMainWindow : Window
        => assembly.GetTypes().Where(x => x.IsAssignableTo(typeof(Window)) && x != typeof(TMainWindow))
            .Aggregate(
                (services, overrides),
                (x, type) => (
                    x.overrides?.TryGetValue(type, out var lifetime) == true
                        ? lifetime switch
                        {
                            ServiceLifetime.Transient => x.services.AddTransient(type),
                            ServiceLifetime.Scoped => x.services.AddScoped(type),
                            _ => x.services.AddSingleton(type)
                        }
                        : x.services.AddSingleton(type), x.overrides)
            )
            .services
            .AddKeyedSingleton<Window, TMainWindow>("MainWindow");
}