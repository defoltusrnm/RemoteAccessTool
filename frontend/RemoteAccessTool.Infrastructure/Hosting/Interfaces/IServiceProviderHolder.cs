namespace RemoteAccessTool.Infrastructure.Hosting.Interfaces;

public interface IServiceProviderHolder
{
    IServiceProvider? ServiceProvider { get; set; }
}