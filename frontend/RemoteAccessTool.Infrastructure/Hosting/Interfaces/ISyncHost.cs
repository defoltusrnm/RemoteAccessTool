namespace RemoteAccessTool.Infrastructure.Hosting.Interfaces;

public interface ISyncHost
{
    void Start();
    IServiceProvider Services { get; }
}