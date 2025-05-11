using System.Reflection;

namespace RemoteAccessTool.Application;

public static class AssemblyLocator
{
    public static readonly Assembly Assembly = typeof(AssemblyLocator).Assembly;
}