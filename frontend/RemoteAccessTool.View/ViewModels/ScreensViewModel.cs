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
        if (screen.Size != screen.Width * screen.Height * 4)
            throw new InvalidOperationException("Frame size mismatch");

        using (screen.Image)
        {
            ref var bitmap = ref CollectionsMarshal.GetValueRefOrAddDefault(_screens, screen.Id, out var exists);

            int width = (int)screen.Width;
            int height = (int)screen.Height;

            if (bitmap == null ||
                bitmap.PixelSize.Width != width ||
                bitmap.PixelSize.Height != height)
            {
                bitmap = new WriteableBitmap(
                    new PixelSize(width, height),
                    new Vector(96, 96),
                    PixelFormat.Bgra8888,
                    AlphaFormat.Premul
                );
            }

            using (var fb = bitmap.Lock())
            {
                var rgba = screen.Image.Memory.Span[..screen.Size];

                if (rgba.Length != width * height * 4)
                    throw new InvalidOperationException("Invalid image size");

                Span<byte> bgra = new byte[rgba.Length];
                ConvertRgbaToBgra(rgba, bgra);

                Marshal.Copy(bgra.ToArray(), 0, fb.Address, bgra.Length);
            }

            OnPropertyChanged(nameof(Screens));
        }
    }

    private static void ConvertRgbaToBgra(ReadOnlySpan<byte> rgba, Span<byte> bgra)
    {
        if (rgba.Length != bgra.Length)
            throw new ArgumentException("RGBA and BGRA spans must be the same length.");

        for (int i = 0; i < rgba.Length; i += 4)
        {
            bgra[i + 0] = rgba[i + 2]; // B
            bgra[i + 1] = rgba[i + 1]; // G
            bgra[i + 2] = rgba[i + 0]; // R
            bgra[i + 3] = rgba[i + 3]; // A
        }
    }
}

public partial class ScreenViewModel : ObservableObject
{
    [ObservableProperty] private Bitmap? _image;
    [ObservableProperty] private uint _id;
}