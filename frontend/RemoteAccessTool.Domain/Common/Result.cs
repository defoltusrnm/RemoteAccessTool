using UnionStruct.Unions;

namespace RemoteAccessTool.Domain.Common;

[Union]
public partial struct Result<TOk, TErr>
{
    [UnionPart(AddMap = true)] private readonly TOk? _ok;
    [UnionPart(AddMap = true)] private readonly TErr? _err;
}

public static class Result
{
    public static Result<Empty, Err> Ok() => Result<Empty, Err>.Ok(Empty.Value);

    public static Result<TOut, Err> Union<T1, T2, TOut>(
        Result<T1, Err> result1,
        Result<T2, Err> result2,
        Func<T1, T2, TOut> mapper
    )
        => (result1, result2) switch
        {
            var (x, y) when x.IsErr(out var xErr) && y.IsErr(out var yErr)
                => Err.Throw<TOut>(
                    xErr.Code + "+" + yErr.Code,
                    xErr.Message + "+" + yErr.Message
                ),
            var (x, y) when x.IsOk(out var xOk) && y.IsOk(out var yOk)
                => Result<TOut, Err>.Ok(mapper(xOk, yOk)),
            var (x, y) when x.IsOk(out _) && y.IsErr(out var err) => Result<TOut, Err>.Err(err),
            var (x, y) when x.IsErr(out var err) && y.IsOk(out _) => Result<TOut, Err>.Err(err),
            _ => Err.Throw<TOut>("UnionFail", "Failed to union results")
        };
}