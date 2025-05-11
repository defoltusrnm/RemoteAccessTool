using CommunityToolkit.Mvvm.ComponentModel;

namespace RemoteAccessTool.View.ViewModels;

public partial class RemoteViewModel : ObservableObject
{
    [ObservableProperty] private string _greeting = "Hello";
}