# WASM Storage Abstraction - Implementation Guide

This document outlines the architecture and implementation details for WASM-compatible storage in the Ensemble Emulator.

## Overview

The emulator needs to work in both native and WASM environments. This requires abstracting away direct filesystem access for features like:

1. **Configuration persistence** (`config.toml`)
2. **Quicksaves/Autosaves** (binary savestate files)
3. **Palette fallback** (loading last known good palette)
4. **File dialogs** (loading ROMs, savestates, palettes)

## Architecture

### Storage Abstraction (`frontend/src/frontend/storage.rs`)

The `Storage` trait provides a unified async interface for both platforms using `async_trait`:

```rust
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Storage: Send + Sync {
    async fn get(&self, key: &str) -> StorageResult<Vec<u8>>;
    async fn set(&self, key: &str, data: Vec<u8>) -> StorageResult<()>;
    async fn delete(&self, key: &str) -> StorageResult<()>;
    async fn exists(&self, key: &str) -> StorageResult<bool>;
    async fn list(&self, prefix: &str) -> StorageResult<Vec<StorageMetadata>>;
    fn get_display_path(&self, key: &str) -> String;
}
```

Note: On WASM, futures don't need to be `Send` since JavaScript is single-threaded.

### Key Format

Storage keys use a path-like format with category prefixes:
- `config/config.toml` - Application configuration
- `data/saves/<game>/quicksaves/quicksave_<timestamp>.sav` - Quicksaves
- `data/saves/<game>/autosaves/autosave_<timestamp>.sav` - Autosaves
- `cache/...` - Cached data that can be regenerated

### Native Implementation (`NativeStorage`)

Uses the `directories` crate to map categories to OS-appropriate paths:
- **Config**: `~/.config/EnsembleEmulator/` (Linux), `%APPDATA%/EnsembleEmulator/` (Windows)
- **Data**: `~/.local/share/EnsembleEmulator/` (Linux), `%LOCALAPPDATA%/EnsembleEmulator/` (Windows)
- **Cache**: `~/.cache/EnsembleEmulator/` (Linux), `%LOCALAPPDATA%/EnsembleEmulator/cache/` (Windows)

### WASM Implementation (`WasmStorage`)

Uses IndexedDB via the `indexed_db_futures` crate:

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
indexed_db_futures = "0.4"
js-sys = "0.3"
```

IndexedDB Structure:
- **Database**: `ensemble_emulator`
- **Object Store**: `storage` (key-value where key is the storage path)

## File Dialog Handling

### rfd::AsyncFileDialog

Already WASM-compatible! The `rfd` crate provides async file dialogs that:
- **Native**: Show native OS file dialogs
- **WASM**: Show browser file input or download dialogs

Key differences:
- **Native**: `FileHandle.path()` returns a `PathBuf`
- **WASM**: `FileHandle.path()` is not available; use `FileHandle.file_name()` instead

### Async Pattern

All file operations now use `spawn_async`:

```rust
#[cfg(target_arch = "wasm32")]
fn spawn_async<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_async<F: Future<Output = ()> + Send + 'static>(f: F) {
    tokio::spawn(f);
}
```

## Migration Checklist

### Phase 1: Async File Dialogs ✅ Complete
- [x] Convert `spawn_palette_picker` to async
- [x] Convert `spawn_rom_picker` to async
- [x] Convert `spawn_save_dialog` to async
- [x] Convert `spawn_savestate_picker` to async
- [x] Convert `spawn_rom_picker_for_savestate` to async
- [x] Add `try_load_state_from_bytes` for loading savestates from memory

### Phase 2: Storage Abstraction (Partial)
- [x] Create `Storage` trait
- [x] Implement `NativeStorage` 
- [ ] Implement `WasmStorage` with IndexedDB
- [ ] Add `indexed_db_futures` dependency for WASM

### Phase 3: Persistence Migration
Replace direct filesystem access in `persistence.rs`:

```rust
// Before
pub fn load_config() -> Option<PersistentConfig> {
    let config_path = get_config_file_path(CONFIG_FILENAME)?;
    let contents = fs::read_to_string(&config_path).ok()?;
    toml::from_str(&contents).ok()
}

// After
pub async fn load_config() -> Option<PersistentConfig> {
    let storage = get_storage();
    let data = storage.get("config/config.toml").await.ok()?;
    let contents = String::from_utf8(data).ok()?;
    toml::from_str(&contents).ok()
}
```

### Phase 4: Quicksave/Autosave Migration
Update `egui_frontend.rs` and `emulator_handler.rs`:

```rust
// Before (egui_frontend.rs)
fn create_auto_save(&self, savestate: Box<SaveState>) {
    let path = get_data_file_path(...);
    let _ = write_file_async(path, savestate.to_bytes(None), false);
}

// After
async fn create_auto_save(&self, savestate: Box<SaveState>) {
    let storage = get_storage();
    let key = autosave_key(&display_name, &timestamp);
    storage.set(&key, savestate.to_bytes(None)).await.ok();
}
```

### Phase 5: Palette Fallback
The palette fallback path (`previous_palette_path`) needs to be converted to:
1. Store the palette data in storage with a known key
2. Load from storage instead of filesystem path

## WASM IndexedDB Implementation Notes

### Required Dependencies

Add to `frontend/Cargo.toml`:
```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
indexed_db_futures = "0.4"
js-sys = "0.3"
```

### Database Initialization

```rust
use indexed_db_futures::prelude::*;

const DB_NAME: &str = "ensemble_emulator";
const STORE_NAME: &str = "storage";
const DB_VERSION: u32 = 1;

async fn open_db() -> Result<IdbDatabase, StorageError> {
    let mut db_req = IdbDatabase::open_u32(DB_NAME, DB_VERSION)?;
    
    db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
        if !evt.db().object_store_names().any(|n| n == STORE_NAME) {
            evt.db().create_object_store(STORE_NAME)?;
        }
        Ok(())
    }));
    
    db_req.await.map_err(|e| StorageError::IndexedDbError(format!("{:?}", e)))
}
```

### Read/Write Operations

```rust
impl Storage for WasmStorage {
    fn get(&self, key: &str) -> ... {
        Box::pin(async move {
            let db = open_db().await?;
            let tx = db.transaction_on_one_with_mode(STORE_NAME, IdbTransactionMode::Readonly)?;
            let store = tx.object_store(STORE_NAME)?;
            
            let value = store.get(&JsValue::from_str(key))?.await?;
            match value {
                Some(v) => {
                    let array = js_sys::Uint8Array::new(&v);
                    Ok(array.to_vec())
                }
                None => Err(StorageError::NotFound),
            }
        })
    }

    fn set(&self, key: &str, data: Vec<u8>) -> ... {
        Box::pin(async move {
            let db = open_db().await?;
            let tx = db.transaction_on_one_with_mode(STORE_NAME, IdbTransactionMode::Readwrite)?;
            let store = tx.object_store(STORE_NAME)?;
            
            let array = js_sys::Uint8Array::from(&data[..]);
            store.put_key_val(&JsValue::from_str(key), &array)?;
            tx.await.into_result()?;
            
            Ok(())
        })
    }
}
```

## Storage Limits

### Browser Storage Limits
- **localStorage**: ~5MB per origin (not suitable for savestates)
- **IndexedDB**: ~50MB+ per origin (browser dependent)
  - Chrome: Min(10% of disk, 60GB)
  - Firefox: Min(10% of disk, 2GB) per origin group
  - Safari: 1GB+ (prompts user for more)

### Recommended Approach
Use IndexedDB for all storage in WASM. It supports:
- Binary data (ArrayBuffer/Uint8Array)
- Async operations (required for WASM)
- Large storage (enough for savestates)
- Structured data with indices

## Testing WASM Build

```bash
# Install wasm target
rustup target add wasm32-unknown-unknown

# Check compilation
cargo check -p ensemble-ballroom --target wasm32-unknown-unknown

# Build with trunk or wasm-pack
trunk build frontend/index.html
```

## Future Considerations

1. **Cloud Sync**: Storage trait could be extended for cloud backends
2. **Export/Import**: Add methods for bulk export/import of saves
3. **Quota Management**: Monitor storage usage and clean old saves
4. **Offline Support**: Use Service Workers for offline WASM support
