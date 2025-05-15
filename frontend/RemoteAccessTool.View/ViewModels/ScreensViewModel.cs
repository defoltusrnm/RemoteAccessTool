using System.Buffers;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using Avalonia;
using Avalonia.Media.Imaging;
using Avalonia.Platform;
using CommunityToolkit.Mvvm.ComponentModel;
using RemoteAccessTool.Application.Remote.Interfaces;

namespace RemoteAccessTool.View.ViewModels;

public partial class ScreensViewModel : ObservableObject
{
    private readonly Dictionary<uint, WriteableBitmap> _screens = new();

    public ICollection<ScreenViewModel> Screens => _screens
        .Select(x => new ScreenViewModel { Id = x.Key, Image = x.Value })
        .ToList();

    public void UpdateView(ScreenEvent screen)
    {
        using (screen.Image)
        {
            ref var bitmap = ref CollectionsMarshal.GetValueRefOrAddDefault(_screens, screen.Id, out var exists);

            var width = (int)screen.Width;
            var height = (int)screen.Height;

            bitmap ??= new WriteableBitmap(
                new PixelSize(width, height),
                new Vector(96, 96),
                PixelFormat.Rgba8888,
                AlphaFormat.Premul
            );

            using (var fb = bitmap.Lock())
            {
                var rgba = screen.Image.Memory.Span[..screen.Size];
                Marshal.Copy(rgba.ToArray(), 0, fb.Address, rgba.Length);
            }

            OnPropertyChanged(nameof(Screens));
        }
    }
}

public partial class ScreenViewModel : ObservableObject
{
    [ObservableProperty] private Bitmap? _image;
    [ObservableProperty] private uint _id;
}