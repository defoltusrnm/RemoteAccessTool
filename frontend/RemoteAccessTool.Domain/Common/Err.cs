namespace RemoteAccessTool.Domain.Common;

public struct Err
{
    public static readonly Err None = new();

    private Err(string code, string message)
    {
        Code = code;
        Message = message;
    }

    public string Code { get; }

    public string Message { get; }

    public static Result<T, Err> Throw<T>(string code, string message) => Result<T, Err>.Err(new(code, message));
}