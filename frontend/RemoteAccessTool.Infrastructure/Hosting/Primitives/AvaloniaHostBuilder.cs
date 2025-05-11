using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Diagnostics.Metrics;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Hosting.Internal;
using Microsoft.Extensions.Logging;
using RemoteAccessTool.Infrastructure.Hosting.Interfaces;

namespace RemoteAccessTool.Infrastructure.Hosting.Primitives;

public class AvaloniaHostBuilder : IAvaloniaHostBuilder
{
    public AvaloniaHostBuilder()
    {
        Services = new ServiceCollection();
        Configuration = new ConfigurationManager();
        Properties = new Dictionary<object, object>();
        Environment = new HostingEnvironment();
        Logging = new LoggingBuilder(Services);
        Metrics = new MetricsBuilder(Services);
    }

    public IDictionary<object, object> Properties { get; }
    public IConfigurationManager Configuration { get; }
    public IHostEnvironment Environment { get; }
    public ILoggingBuilder Logging { get; }
    public IMetricsBuilder Metrics { get; }
    public IServiceCollection Services { get; }

    public ISyncHost Build() => new AvaloniaHost(Services.BuildServiceProvider());

    public void ConfigureContainer<TContainerBuilder>(
        IServiceProviderFactory<TContainerBuilder> factory,
        Action<TContainerBuilder>? configure = null
    ) where TContainerBuilder : notnull => throw new NotImplementedException();
}

internal class LoggingBuilder : ILoggingBuilder
{
    public LoggingBuilder(IServiceCollection services)
    {
        Services = services;
    }

    public IServiceCollection Services { get; }
}

internal class MetricsBuilder : IMetricsBuilder
{
    public MetricsBuilder(IServiceCollection services)
    {
        Services = services;
    }

    public IServiceCollection Services { get; }
}