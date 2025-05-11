using Avalonia.Markup.Xaml;

namespace RemoteAccessTool.Infrastructure.Markup;

public class Inject : MarkupExtension
{
    public required Type Type { get; init; }

    public override object ProvideValue(IServiceProvider serviceProvider)
    {
        return new object();
    }
}