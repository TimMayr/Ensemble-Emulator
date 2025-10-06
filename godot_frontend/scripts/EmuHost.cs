using System;
using System.IO;
using System.Runtime.InteropServices;
using Godot;
// ReSharper disable NullableWarningSuppressionIsUsed

namespace Emulator_fronted.scripts;

public partial class EmuHost : Node
{
    // === Delegates & structs from before ===
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    private delegate IntPtr EmuCreateDelegate();
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    private delegate void EmuDestroyDelegate(IntPtr emu);
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    private delegate int EmuGetVideoSpecDelegate(IntPtr emu, out EmuVideoSpec spec);
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    private delegate int EmuSetVideoBufferDelegate(IntPtr emu, IntPtr ptr, UIntPtr len);
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    private delegate int EmuStepFrameDelegate(IntPtr emu);

    [StructLayout(LayoutKind.Sequential)]
    public struct EmuVideoSpec
    {
        public int width;
        public int height;
        public int stride;
        public int format;
    }

    // === Private fields ===
    private IntPtr libHandle;
    private IntPtr emu;
    private EmuCreateDelegate? emuCreate;
    private EmuDestroyDelegate? emuDestroy;
    private EmuGetVideoSpecDelegate? emuGetVideoSpec;
    private EmuSetVideoBufferDelegate? emuSetVideoBuffer;
    private EmuStepFrameDelegate? emuStepFrame;

    // === Framebuffer management ===
    private byte[]? videoBuffer;
    private GCHandle videoHandle;
    private EmuVideoSpec videoSpec;

    private Image image = new Image();
    private ImageTexture texture = new ImageTexture();
    private Sprite2D? sprite;

    // === Godot lifecycle ===
    public override void _Ready()
    {
        try
        {
            this.loadLibrary();
            this.resolveFunctions();
            this.initializeEmulator();
            this.setupFramebuffer();
            this.setupDisplay();
        }
        catch (Exception e)
        {
            GD.PrintErr($"[EmuHost] Initialization failed: {e.Message}");
        }
    }

    public override void _Process(double delta)
    {
        if (this.emu == IntPtr.Zero) {
            return;
        }

        // Run one frame of emulation
        this.emuStepFrame?.Invoke(this.emu);

        // Update texture from Rust’s pixel buffer
        this.image.SetData(this.videoSpec.width, this.videoSpec.height, false, Image.Format.Rgba8, this.videoBuffer);
        this.texture.Update(this.image);
    }

    public override void _ExitTree()
    {
        this.shutdownEmulator();
        this.unloadLibrary();
    }

    // === Library loading methods (same as before) ===
    private void loadLibrary()
    {
        string libFile =
        #if WINDOWS
            "nes_ffi.dll";
        #elif MACOS
            "libnes_ffi.dylib";
        #else
            "libnes_ffi.so";
    #endif

        string absPath = Path.Combine(ProjectSettings.GlobalizePath("res://bin"), libFile);
        this.libHandle = NativeLibrary.Load(absPath);
        GD.Print($"[EmuHost] Library loaded: {absPath}");
    }

    private void resolveFunctions()
    {
        this.emuCreate = this.loadSymbol<EmuCreateDelegate>("emu_create");
        this.emuDestroy = this.loadSymbol<EmuDestroyDelegate>("emu_destroy");
        this.emuGetVideoSpec = this.loadSymbol<EmuGetVideoSpecDelegate>("emu_get_video_spec");
        this.emuSetVideoBuffer = this.loadSymbol<EmuSetVideoBufferDelegate>("emu_set_video_buffer");
        this.emuStepFrame = this.loadSymbol<EmuStepFrameDelegate>("emu_step_frame");
    }

    private T loadSymbol<T>(string name) where T : Delegate
    {
        IntPtr ptr = NativeLibrary.GetExport(this.libHandle, name);
        return Marshal.GetDelegateForFunctionPointer<T>(ptr);
    }

    private void unloadLibrary()
    {
        if (this.libHandle != IntPtr.Zero)
        {
            NativeLibrary.Free(this.libHandle);
            this.libHandle = IntPtr.Zero;
        }
    }

    private void initializeEmulator()
    {
        this.emu = this.emuCreate!();
        if (this.emuGetVideoSpec!(this.emu, out this.videoSpec) != 0) {
            throw new Exception("Failed to get video spec from Rust");
        }

        GD.Print($"[EmuHost] Video spec: {this.videoSpec.width}x{this.videoSpec.height}");
    }

    private void shutdownEmulator() {
        if (this.emu == IntPtr.Zero) {
            return;
        }

        this.emuDestroy?.Invoke(this.emu);
        this.emu = IntPtr.Zero;
    }

    // === Framebuffer setup ===
    private void setupFramebuffer()
    {
        int size = this.videoSpec.width * this.videoSpec.height * 4;
        this.videoBuffer = new byte[size];
        this.videoHandle = GCHandle.Alloc(this.videoBuffer, GCHandleType.Pinned);

        IntPtr ptr = this.videoHandle.AddrOfPinnedObject();
        UIntPtr len = (UIntPtr)size;

        int result = this.emuSetVideoBuffer!(this.emu, ptr, len);
        if (result != 0) {
            throw new Exception("Failed to set video buffer.");
        }
    }

    // === Display setup ===
    private void setupDisplay()
    {
        this.image = Image.CreateEmpty(this.videoSpec.width, this.videoSpec.height, false, Image.Format.Rgba8);
        this.texture = ImageTexture.CreateFromImage(this.image);

        this.sprite = new Sprite2D
        {
            Texture = this.texture,
            TextureFilter = CanvasItem.TextureFilterEnum.Nearest,
            Scale = new Vector2(3, 3), // scale up pixels 3×
        };

        this.AddChild(this.sprite);
        GD.Print("[EmuHost] Display initialized.");
    }
}