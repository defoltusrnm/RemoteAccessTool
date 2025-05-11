using Microsoft.Extensions.Options;

namespace RemoteAccessTool.Infrastructure.Options;

public class PlainOptions<T> : IOptions<T> where T : class
{
    public PlainOptions(T value)
    {
        Value = value;
    }

    public T Value { get; }
}