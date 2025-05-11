using Microsoft.Extensions.Hosting;

namespace RemoteAccessTool.Infrastructure.Hosting.Interfaces;

public interface IAvaloniaHostBuilder : IHostApplicationBuilder
{
    ISyncHost Build();
}