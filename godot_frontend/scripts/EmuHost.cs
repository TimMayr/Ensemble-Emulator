using System;
using System.IO;
using System.Runtime.InteropServices;
using Godot;

// ReSharper disable NullableWarningSuppressionIsUsed

namespace Emulator_fronted.scripts;

public partial class EmuHost : Node {
	private IntPtr emu;
	private EmuCreateDelegate? emuCreate;
	private EmuDestroyDelegate? emuDestroy;
	private EmuEnableTraceLogDelegate? emuEnableTraceLog;
	private EmuGetVideoSpecDelegate? emuGetVideoSpec;
	private EmuLoadRomDelegate? emuLoadRom;
	private EmuResetDelegate? emuReset;
	private EmuSetPausedDelegate? emuSetPaused;
	private EmuSetVideoBufferDelegate? emuSetVideoBuffer;
	private EmuStepFrameDelegate? emuStepFrame;
	private Button? enableTraceLogButton;

	private Image image = new Image();
	private bool isPaused;

	// === Private fields ===
	private IntPtr libHandle;
	private Button? loadRomButton;
	private Button? pauseButton;
	private FileDialog? romDialog;
	private Sprite2D? sprite;
	private ImageTexture texture = new ImageTexture();

	// === UI state ===
	private CanvasLayer? uiLayer;

	// === Framebuffer management ===
	private byte[]? videoBuffer;
	private GCHandle videoHandle;
	private EmuVideoSpec videoSpec;

	// === Godot lifecycle ===
	public override void _Ready() {
		try {
			this.loadLibrary();
			this.resolveFunctions();
			this.initializeEmulator();
			this.setupFramebuffer();
			this.setupDisplay();
			this.setupUi();
		} catch (Exception e) {
			GD.PrintErr($"[EmuHost] Initialization failed: {e.Message}");
		}
	}

	public override void _Process(double delta) {
		if (this.emu == IntPtr.Zero) {
			return;
		}

		if (this.videoBuffer == null) {
			return;
		}

		// Run one frame of emulation
		if (!this.isPaused) {
			int? res = this.emuStepFrame?.Invoke(this.emu);
			if (res != 0) {
				this.setPaused(true);
				GD.Print("[EmuHost] Emulation stopped");
			}
		}

		// Update texture from Rust’s pixel buffer
		this.image.SetData(this.videoSpec.width, this.videoSpec.height, false, Image.Format.Rgba8, this.videoBuffer);
		this.texture.Update(this.image);
	}

	public override void _ExitTree() {
		this.shutdownEmulator();
		this.unloadLibrary();

		if (this.videoHandle.IsAllocated) {
			this.videoHandle.Free();
		}
	}

	// === Library loading methods (same as before) ===
	private void loadLibrary() {
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

	private void resolveFunctions() {
		this.emuCreate = this.loadSymbol<EmuCreateDelegate>("emu_create");
		this.emuDestroy = this.loadSymbol<EmuDestroyDelegate>("emu_destroy");
		this.emuGetVideoSpec = this.loadSymbol<EmuGetVideoSpecDelegate>("emu_get_video_spec");
		this.emuSetVideoBuffer = this.loadSymbol<EmuSetVideoBufferDelegate>("emu_set_video_buffer");
		this.emuStepFrame = this.loadSymbol<EmuStepFrameDelegate>("emu_step_frame");
		this.emuLoadRom = this.loadSymbol<EmuLoadRomDelegate>("emu_load_rom");
		this.emuSetPaused = this.loadSymbol<EmuSetPausedDelegate>("emu_set_paused");
		this.emuReset = this.loadSymbol<EmuResetDelegate>("emu_reset");
		this.emuEnableTraceLog = this.loadSymbol<EmuEnableTraceLogDelegate>("emu_enable_trace_log");
	}

	private T loadSymbol<T>(string name) where T : Delegate {
		IntPtr ptr = NativeLibrary.GetExport(this.libHandle, name);
		return Marshal.GetDelegateForFunctionPointer<T>(ptr);
	}

	private void unloadLibrary() {
		if (this.libHandle == IntPtr.Zero) {
			return;
		}

		NativeLibrary.Free(this.libHandle);
		this.libHandle = IntPtr.Zero;
	}

	private void initializeEmulator() {
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
	private void setupFramebuffer() {
		int size = this.videoSpec.width * this.videoSpec.height * 4;
		this.videoBuffer = new byte[size];
		this.videoHandle = GCHandle.Alloc(this.videoBuffer, GCHandleType.Pinned);

		IntPtr ptr = this.videoHandle.AddrOfPinnedObject();
		UIntPtr len = (UIntPtr) size;

		int result = this.emuSetVideoBuffer!(this.emu, ptr, len);
		if (result != 0) {
			throw new Exception("Failed to set video buffer.");
		}
	}

	// === Display setup ===
	private void setupDisplay() {
		this.image = Image.CreateEmpty(this.videoSpec.width, this.videoSpec.height, false, Image.Format.Rgba8);
		this.texture = ImageTexture.CreateFromImage(this.image);

		this.sprite = new Sprite2D {
			Texture = this.texture,
			TextureFilter = CanvasItem.TextureFilterEnum.Nearest,
			Scale = new Vector2(3, 3) // scale up pixels 3×
		};

		this.AddChild(this.sprite);
		GD.Print("[EmuHost] Display initialized.");
	}

	private void setupUi() {
		this.uiLayer = new CanvasLayer();
		this.AddChild(this.uiLayer);

		this.loadRomButton = new Button {
			Text = "Load ROM",
			Position = new Vector2(16, 16),
			Size = new Vector2(150, 32)
		};

		this.loadRomButton.Pressed += this.onLoadRomButtonPressed;

		this.pauseButton = new Button {
			Text = "Pause",
			Position = new Vector2(180, 16),
			Size = new Vector2(150, 32)
		};

		this.pauseButton.Pressed += this.onPauseButtonPressed;

		this.romDialog = new FileDialog {
			FileMode = FileDialog.FileModeEnum.OpenFile,
			Access = FileDialog.AccessEnum.Filesystem,
			Title = "Select NES ROM"
		};

		this.romDialog.Filters = ["*.nes ; NES ROM"];
		this.romDialog.FileSelected += this.onRomDialogFileSelected;

		this.enableTraceLogButton = new Button {
			Text = "Enable Trace Log",
			Position = new Vector2(360, 16),
			Size = new Vector2(150, 32)
		};

		this.enableTraceLogButton.Pressed += this.enableTraceLog;

		this.uiLayer.AddChild(this.loadRomButton);
		this.uiLayer.AddChild(this.pauseButton);
		this.uiLayer.AddChild(this.romDialog);
		this.uiLayer.AddChild(this.enableTraceLogButton);

		this.updatePauseButtonLabel();
	}

	private void onLoadRomButtonPressed() {
		this.romDialog?.PopupCentered();
	}

	private void onPauseButtonPressed() {
		this.setPaused(!this.isPaused);
	}

	private void onRomDialogFileSelected(string path) {
		this.loadRomFromPath(path);
	}

	private void loadRomFromPath(string rawPath) {
		if ((this.emu == IntPtr.Zero) || (this.emuLoadRom == null)) {
			return;
		}

		string absolutePath = ProjectSettings.GlobalizePath(rawPath);
		IntPtr pathPtr = IntPtr.Zero;

		try {
			pathPtr = Marshal.StringToHGlobalAnsi(absolutePath);
			int result = this.emuLoadRom(this.emu, pathPtr);
			if (result != 0) {
				GD.PrintErr($"[EmuHost] Failed to load ROM: {absolutePath}");
				return;
			}

			GD.Print($"[EmuHost] ROM loaded: {absolutePath}");

			if (this.emuReset == null) {
				return;
			}

			this.emuReset.Invoke(this.emu);

			GD.Print("[EmuHost] Reset scheduled");

			this.setPaused(false);
		} finally {
			if (pathPtr != IntPtr.Zero) {
				Marshal.FreeHGlobal(pathPtr);
			}
		}
	}

	private void setPaused(bool paused) {
		this.isPaused = paused;

		if ((this.emu != IntPtr.Zero) && (this.emuSetPaused != null)) {
			this.emuSetPaused(this.emu, paused ? 1 : 0);
		}

		this.updatePauseButtonLabel();
	}

	private void updatePauseButtonLabel() {
		if (this.pauseButton != null) {
			this.pauseButton.Text = this.isPaused ? "Resume" : "Pause";
		}
	}

	private void enableTraceLog() {
		GD.Print("[EmuHost] Enabled trace log");
		this.emuEnableTraceLog?.Invoke(this.emu);
	}

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

	[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
	private delegate int EmuLoadRomDelegate(IntPtr emu, IntPtr pathPtr);

	[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
	private delegate int EmuSetPausedDelegate(IntPtr emu, int paused);

	[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
	private delegate void EmuResetDelegate(IntPtr emu);

	[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
	private delegate void EmuEnableTraceLogDelegate(IntPtr emu);

	[StructLayout(LayoutKind.Sequential)]
	public struct EmuVideoSpec {
		public int width;
		public int height;
		public int stride;
		public int format;
	}
}