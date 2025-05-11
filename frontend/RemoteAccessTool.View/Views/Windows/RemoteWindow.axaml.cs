using Avalonia.Controls;
using RemoteAccessTool.View.ViewModels;

namespace RemoteAccessTool.View.Views.Windows;

public partial class RemoteWindow : Window
{
    public RemoteWindow(RemoteViewModel viewModel)
    {
        DataContext = viewModel;
        InitializeComponent();
    }
}