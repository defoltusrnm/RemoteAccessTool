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
    public static Result<Empty, Err> Err(Err err) => Result<Empty, Err>.Err(err);
}