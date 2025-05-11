namespace RemoteAccessTool.Infrastructure.Hosting.Interfaces;

public interface ISyncHost
{
    void Start(string[] args);
    IServiceProvider Services { get; }
}