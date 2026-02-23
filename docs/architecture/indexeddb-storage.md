# IndexedDB Storage Architecture Plan

## 1. Feasibility Analysis

### 1.1 Storage Capacity

Browser IndexedDB quotas are generous for this use case:

| Browser | Quota | Notes |
|---------|-------|-------|
| Chrome/Edge | Up to 80% of total disk space (per origin) | Typically hundreds of GB |
| Firefox | Up to 50% of disk, max 2 GB per group | Grouped by eTLD+1 |
| Safari | ~1 GB initially, prompts user for more | Can be purged in "clear website data" |

**Our storage needs are tiny relative to these limits:**

| Data Type | Typical Size | Max Expected Count | Total |
|-----------|-------------|-------------------|-------|
| NES ROM | 256 KB–2 MB | ~20 ROMs | ~40 MB |
| Savestate (.sav) | ~7 KB | ~1000 (quick+auto) | ~7 MB |
| Config (TOML) | ~2 KB | 1 | ~2 KB |
| **Total** | | | **~50 MB worst case** |

**Verdict:** ✅ Storage capacity is not a concern. Even with aggressive use, we won't hit 100 MB.

### 1.2 Performance

IndexedDB operations are asynchronous and well-optimized in modern browsers:

| Operation | Typical Latency | Notes |
|-----------|----------------|-------|
| Single read (by key) | <1 ms | Key lookups are O(1) |
| Single write (7 KB savestate) | 1–5 ms | Includes commit |
| Prefix scan (100 items) | 5–20 ms | Uses key range cursor |
| Write 2 MB ROM | 5–15 ms | Binary data stored natively as ArrayBuffer |

**Critical paths and their performance impact:**

1. **Quicksave/Quickload**: 7 KB write/read → <5 ms. No perceptible delay.
2. **ROM loading**: 2 MB write on first upload → <15 ms. Once cached, reads are <5 ms.
3. **ROM matching on savestate load**: Requires scanning ROM keys and reading checksums. With an index on `checksum`, this is a single index lookup: <2 ms.
4. **Autosave cleanup**: List + delete of old saves. Can run in background via `spawn_local`.

**Verdict:** ✅ Performance is excellent. IndexedDB is designed for exactly this data volume.

### 1.3 Library Comparison

| Library | Version | API Style | Maturity | Notes |
|---------|---------|-----------|----------|-------|
| **rexie** | 0.6.2 | High-level, Futures-based | Good, active | **Already in Cargo.toml**. Wraps `idb` crate. Clean Rust async API. Supports KeyRange for prefix queries. |
| **indexed_db_futures** | 0.5 | Mid-level, Futures-based | Mature | More control over raw IDB API, but more boilerplate. |
| **idb** | 0.6 | Low-level | Mature | Used by rexie internally. Direct web-sys wrapper. |

**Recommendation:** Use **rexie** (already a dependency). It provides the cleanest API:
- `Store::get(key)` / `Store::put(value, key)` / `Store::delete(key)` for CRUD
- `Store::get_all_keys(key_range, limit)` for prefix queries
- `Store::scan(key_range, ...)` for listing with values
- `KeyRange::bound(lower, upper)` for prefix-based range queries

### 1.4 Key Querying and Prefix Scans

IndexedDB sorts string keys lexicographically. This means prefix-based queries work naturally:

```javascript
// To find all keys starting with "data/saves/SuperMario/"
KeyRange.bound("data/saves/SuperMario/", "data/saves/SuperMario/\uffff")
```

In Rust with rexie:
```rust
let lower = JsValue::from_str("data/saves/SuperMario/");
let upper = JsValue::from_str(&format!("data/saves/SuperMario/{}", '\u{ffff}'));
let range = KeyRange::bound(&lower, &upper, Some(false), Some(false))?;
let keys = store.get_all_keys(Some(range), None).await?;
```

**This maps perfectly to the existing `Storage::list(prefix)` API.** No tree structure or runtime index is needed — IndexedDB's native key ordering provides efficient prefix queries out of the box.

**Why no tree/index structure is needed:**
- IndexedDB stores keys in a B-tree internally, providing O(log n) prefix scans
- `KeyRange::bound()` maps directly to `IDBKeyRange.bound()` which uses the B-tree for efficient range queries
- The `StorageKey` → string key conversion (`"category/sub_path"`) produces lexicographically-ordered keys that group naturally by directory
- Example: All quicksaves for "SuperMario" share the prefix `data/saves/SuperMario/quicksaves/` and are returned in sorted order

### 1.5 Legal Analysis

**No legal issues.** The approach is legally identical to how any web application stores user data:

1. **Data locality**: All data stays in the user's browser. Nothing is transmitted to any server. The browser's same-origin policy ensures isolation.
2. **ROM storage**: The user is uploading their own ROM file, which is then stored locally in their own browser storage. This is analogous to copying a file to a local directory — no distribution occurs.
3. **Copyright**: ROMs themselves may be copyrighted, but storing a user's own copy locally doesn't constitute distribution or circumvention. This is the same model used by every browser-based emulator (e.g., RetroArch Web, JSNES).
4. **GDPR/Privacy**: No personal data is collected or transmitted. IndexedDB data is controlled entirely by the user and can be cleared via browser settings.
5. **Precedent**: Browser-based emulators like RetroArch Web, Emularity, and JSNES all use IndexedDB for ROM/save persistence with no legal issues.

### 1.6 Complexity Estimate

| Component | Effort | Complexity |
|-----------|--------|------------|
| WasmStorage implementation | Medium | Straightforward CRUD using rexie |
| ROM caching on upload | Low | Add `storage.set()` call after file picker read |
| Savestate caching on upload | Low | Same pattern |
| ROM matching from IndexedDB | Medium | Query ROM store by checksum, or scan all ROMs |
| Quick/auto save via IndexedDB | Low | Already uses `StorageKey`; just need async path |
| Export saves to file | Medium | New UI for listing + download via `rfd` save dialog |
| Save browser UI (load last N) | Medium-High | New egui panel for listing saves from IndexedDB |
| WASM async wrappers | Medium | Need `spawn_local` + channel pattern for sync call sites |

**Total estimate:** ~2-3 sessions of focused work.

---

## 2. Architecture Plan

### 2.1 Database Schema

Single IndexedDB database with a simple key-value object store:

```
Database: "ensemble_emulator" (version 1)

Object Store: "storage"
  - Key: String (the StorageKey path, e.g., "data/saves/SuperMario/quicksave_2024-01-15.sav")
  - Value: Uint8Array (raw file bytes)
  - No indexes needed (prefix queries use KeyRange on primary key)
```

**Why a single flat store?**
1. Simplicity: directly mirrors the native `NativeStorage` which just maps keys to files
2. The `Storage` trait API is purely key→bytes; metadata (checksums, filenames) is handled at the application layer
3. IndexedDB's native key ordering provides efficient prefix queries without indexes
4. Given our data sizes (<50 MB total), there's no performance need for separate metadata

**Key naming convention:**

```
config/config/config.toml           → App config
data/roms/<filename>.nes            → Cached ROM files
data/saves/<game>/quicksaves/...    → Quicksaves (matches existing quicksave_key())
data/saves/<game>/autosaves/...     → Autosaves (matches existing autosave_key())
data/uploads/savestates/<name>.sav  → User-uploaded savestates
cache/upload_cache/...              → Temporary upload data
```

These keys are identical to the ones already generated by `quicksave_key()`, `autosave_key()`, `config_key()` etc. in `storage.rs`. The `Storage::list(prefix)` implementation maps to `KeyRange::bound(prefix, prefix + '\uffff')`.

### 2.2 WasmStorage Implementation

The current `WasmStorage` stub returns `NotAvailable` for all operations. Here is the full implementation plan using rexie:

```rust
#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use rexie::{Rexie, TransactionMode, KeyRange};
    use wasm_bindgen::JsValue;
    use js_sys::Uint8Array;
    use std::cell::RefCell;
    use std::rc::Rc;

    const DB_NAME: &str = "ensemble_emulator";
    const DB_VERSION: u32 = 1;
    const STORE_NAME: &str = "storage";

    // Cache the database connection for reuse (WASM is single-threaded)
    thread_local! {
        static DB_CACHE: RefCell<Option<Rexie>> = RefCell::new(None);
    }

    pub struct WasmStorage;

    impl WasmStorage {
        pub fn new() -> Self { WasmStorage }

        /// Convert StorageKey to the string key used in IndexedDB
        fn key_string(key: &StorageKey) -> String {
            format!("{}/{}", key.category.prefix(), key.sub_path)
        }
    }

    #[async_trait(?Send)]
    impl Storage for WasmStorage {
        async fn get(&self, key: &StorageKey) -> StorageResult<Vec<u8>> {
            let db = open_db().await?;
            let tx = db.transaction(&[STORE_NAME], TransactionMode::ReadOnly)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let store = tx.store(STORE_NAME)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;

            let key_js = JsValue::from_str(&Self::key_string(key));
            match store.get(key_js).await
                .map_err(|e| StorageError::ReadError(e.to_string()))? {
                Some(val) => {
                    let array = Uint8Array::new(&val);
                    Ok(array.to_vec())
                }
                None => Err(StorageError::NotFound),
            }
        }

        async fn set(&self, key: &StorageKey, data: Vec<u8>) -> StorageResult<()> {
            let db = open_db().await?;
            let tx = db.transaction(&[STORE_NAME], TransactionMode::ReadWrite)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            let store = tx.store(STORE_NAME)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;

            let key_js = JsValue::from_str(&Self::key_string(key));
            let value_js: JsValue = Uint8Array::from(data.as_slice()).into();
            store.put(&value_js, Some(&key_js)).await
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            tx.done().await
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            Ok(())
        }

        async fn delete(&self, key: &StorageKey) -> StorageResult<()> {
            let db = open_db().await?;
            let tx = db.transaction(&[STORE_NAME], TransactionMode::ReadWrite)
                .map_err(|e| StorageError::DeleteError(e.to_string()))?;
            let store = tx.store(STORE_NAME)
                .map_err(|e| StorageError::DeleteError(e.to_string()))?;

            let key_js = JsValue::from_str(&Self::key_string(key));
            store.delete(key_js).await
                .map_err(|e| StorageError::DeleteError(e.to_string()))?;
            tx.done().await
                .map_err(|e| StorageError::DeleteError(e.to_string()))?;
            Ok(())
        }

        async fn exists(&self, key: &StorageKey) -> StorageResult<bool> {
            let db = open_db().await?;
            let tx = db.transaction(&[STORE_NAME], TransactionMode::ReadOnly)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let store = tx.store(STORE_NAME)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;

            let key_js = JsValue::from_str(&Self::key_string(key));
            store.key_exists(key_js).await
                .map_err(|e| StorageError::ReadError(e.to_string()))
        }

        async fn list(&self, prefix: &StorageKey) -> StorageResult<Vec<StorageMetadata>> {
            let db = open_db().await?;
            let tx = db.transaction(&[STORE_NAME], TransactionMode::ReadOnly)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let store = tx.store(STORE_NAME)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;

            let prefix_str = Self::key_string(prefix);
            let lower = JsValue::from_str(&prefix_str);
            let upper = JsValue::from_str(&format!("{}\u{ffff}", prefix_str));
            let range = KeyRange::bound(&lower, &upper, Some(false), Some(false))
                .map_err(|e| StorageError::ReadError(format!("{:?}", e)))?;

            let keys = store.get_all_keys(Some(range), None).await
                .map_err(|e| StorageError::ReadError(e.to_string()))?;

            Ok(keys.into_iter()
                .filter_map(|k| k.as_string().map(|s| StorageMetadata {
                    key: StorageKey::from(&s),
                }))
                .collect())
        }

        fn get_display_path(&self, key: &StorageKey) -> String {
            format!("indexeddb://ensemble_emulator/{}", Self::key_string(key))
        }

        fn key_to_path_opt(&self, _key: Option<&StorageKey>) -> Option<PathBuf> {
            None // WASM has no filesystem paths
        }

        fn key_to_path(&self, _key: &StorageKey) -> Option<PathBuf> {
            None // WASM has no filesystem paths
        }
    }

    /// Open (or reuse cached) database connection
    async fn open_db() -> Result<Rexie, StorageError> {
        Rexie::builder(DB_NAME)
            .version(DB_VERSION)
            .add_object_store(rexie::ObjectStore::new(STORE_NAME))
            .build()
            .await
            .map_err(|e| StorageError::IndexedDbError(e.to_string()))
    }
}
```

### 2.3 File Caching on Upload

When a user picks a file via `rfd`, the data is already read into memory. We persist it to IndexedDB so it can be re-read later (for ROM matching, quickloads, etc.).

**Changes to `util.rs` file pickers:**

#### ROM Upload (`spawn_rom_picker`)
After reading the ROM data, also store it in IndexedDB:

```rust
// After: let data = handle.read().await;
// Add:
let rom_storage_key = StorageKey {
    category: StorageCategory::Data,
    sub_path: format!("roms/{}", name),
};
let _ = get_storage().set(&rom_storage_key, data.clone()).await;
```

The ROM is stored at `data/roms/<filename>`. This creates a "virtual directory" of cached ROMs that can be scanned for ROM matching.

#### Savestate Upload (`spawn_savestate_picker`)
After reading the savestate data:

```rust
let save_storage_key = StorageKey {
    category: StorageCategory::Data,
    sub_path: format!("uploads/savestates/{}", savestate_name),
};
let _ = get_storage().set(&save_storage_key, data.clone()).await;
```

#### Palette Upload
Explicitly excluded from caching per the requirement ("apart from palette"). Palettes are small, only affect rendering, and don't need to persist in IndexedDB.

### 2.4 ROM Matching on WASM

On native, ROM matching scans a filesystem directory. On WASM, we scan the IndexedDB "virtual ROM directory":

```rust
#[cfg(target_arch = "wasm32")]
async fn find_matching_rom_in_storage(
    context: &SavestateLoadContext,
) -> Option<LoadedRom> {
    let rom_prefix = StorageKey {
        category: StorageCategory::Data,
        sub_path: "roms/".to_string(),
    };
    let storage = get_storage();
    let expected_checksum = &context.savestate.rom_file.data_checksum;

    // If we know the ROM filename, try it directly first
    if let Some(ref rom_name) = context.savestate.rom_file.name {
        let direct_key = StorageKey {
            category: StorageCategory::Data,
            sub_path: format!("roms/{}", rom_name),
        };
        if let Ok(data) = storage.get(&direct_key).await {
            let checksum = compute_data_checksum(&data);
            if &checksum == expected_checksum {
                return Some(LoadedRom {
                    data,
                    name: rom_name.clone(),
                    directory: direct_key,
                });
            }
        }
    }

    // Scan all cached ROMs
    if let Ok(entries) = storage.list(&rom_prefix).await {
        for entry in entries {
            if let Ok(data) = storage.get(&entry.key).await {
                let checksum = compute_data_checksum(&data);
                if &checksum == expected_checksum {
                    let name = entry.key.sub_path
                        .rsplit('/').next()
                        .unwrap_or("unknown.nes").to_string();
                    return Some(LoadedRom {
                        data,
                        name,
                        directory: entry.key,
                    });
                }
            }
        }
    }

    None
}
```

This mirrors the native `find_matching_rom_in_directory` but reads from IndexedDB instead of the filesystem.

### 2.5 Quick/Auto Saves on WASM

The existing key generation functions (`quicksave_key()`, `autosave_key()`) already produce correct StorageKeys. On native, these are written via `sync_wrappers::write_sync()`. On WASM, all storage is async.

**Quicksave (write):**
```rust
// In handle_quicksave, WASM path:
let key = quicksave_key(&display_name, &timestamp);
let data = savestate.to_bytes(None);
wasm_bindgen_futures::spawn_local(async move {
    let storage = get_storage();
    let _ = storage.set(&key, data).await;
});
```

**Quickload (read):**
Since `handle_quickload` currently uses `storage::read_sync()`, the WASM path needs an async alternative. Add a new message variant:

```rust
// New message variant
AsyncFrontendMessage::QuickloadData(Box<SaveState>)

// In handle_quickload, WASM path:
let sender = self.async_sender.clone();
wasm_bindgen_futures::spawn_local(async move {
    let storage = get_storage();
    if let Ok(data) = storage.get(&key).await {
        if let Some(savestate) = try_load_state_from_bytes(&data) {
            let _ = sender.send(AsyncFrontendMessage::QuickloadData(Box::new(savestate)));
        }
    }
});
```

**Autosave cleanup:**
Same pattern — use `spawn_local` with async storage calls instead of sync wrappers.

### 2.6 WASM Sync Wrapper Strategy — IMPLEMENTED

Strategy A (cfg-gated async alternatives) was implemented. The call sites that needed WASM alternatives:

1. ✅ `handle_quicksave` (write savestate) → `spawn_local` + async `storage.set()`
2. ✅ `handle_quickload` (read savestate) → `spawn_local` + full async flow (no new message variant needed)
3. ✅ `create_auto_save` (write autosave) → `spawn_local` + async `storage.set()`
4. ✅ `cleanup_old_autosaves_async` (list + delete) → `spawn_local` + async `storage.list()`/`delete()`
5. ✅ `get_current_quicksave_key` (list quicksaves) → extracted shared `find_newest_quicksave()`, WASM quickload does the listing inline
6. ℹ️ `load_config` / `save_config` → not changed; egui persistence on WASM uses localStorage, which is sufficient for config

### 2.7 Export Saves to File (WASM-specific UI)

Users need to export saves from IndexedDB to actual files. Add a WASM-specific menu:

```
File → Savestates → Export Save...
```

Implementation:
1. List saves from IndexedDB using `storage.list(saves_prefix)`
2. Display in an egui window with filename, date, size
3. On "Export" click, use `rfd::AsyncFileDialog::save_file()` to write to the host filesystem
4. Read data from IndexedDB, write via the file handle

This is also useful on native to export from the app's data directory.

### 2.8 Save Browser UI (Load from IndexedDB)

Since `rfd` only picks OS files, we need a custom egui panel for loading saves from IndexedDB:

```
File → Savestates → Load Quick/Auto Save...
```

This panel shows:
- Recent quicksaves (sorted by date, newest first)
- Recent autosaves (sorted by date, newest first)
- Each entry: filename, date/time, ROM name
- Click to load directly from IndexedDB

This panel is useful on **both** platforms (native users also benefit from browsing their saves).

Implementation:
1. Add a `SaveBrowserState` to `PendingDialogs` (or a new `Pane` variant)
2. On open, async-list saves from IndexedDB (or sync on native)
3. Cache the list in a `Vec<SaveEntry>` on the dialog state
4. On click, read save data and send `LoadSaveState` message

---

## 3. Edge Cases and Potential Issues

### 3.1 Database Versioning
IndexedDB supports schema migrations via version numbers. Start at version 1. If schema changes are needed later, increment the version and add migration logic in rexie's builder.

### 3.2 Safari Storage Eviction
Safari may evict IndexedDB data after 7 days of inactivity (ITP policy) or when the user clears website data. Mitigations:
- Request persistent storage via `navigator.storage.persist()` at startup
- The export feature (§2.7) serves as a manual backup mechanism
- Document this behavior for users

### 3.3 Private/Incognito Browsing
IndexedDB works in private browsing but data is deleted when the window closes. This is acceptable — private browsing is inherently ephemeral.

### 3.4 Cross-Tab Consistency
IndexedDB handles concurrent access correctly via transactions. Two tabs share the same database. No special handling needed.

### 3.5 ROM De-duplication
If a user uploads the same ROM twice with different filenames, both copies are stored. ROM matching by checksum will find either. De-duplication could be added later but isn't critical.

### 3.6 Config Persistence on WASM
Currently, config persistence uses `load_config()` / `save_config()` which use sync wrappers. On WASM:
- **Option A**: Use egui's built-in persistence (localStorage) for config only, IndexedDB for binary data
- **Option B**: Make config load/save fully async via IndexedDB

Option A is simpler since config is just TOML text (<2 KB), and egui already has WASM localStorage persistence.

---

## 4. Implementation Status

### ✅ Phase 1: Core WasmStorage — COMPLETE
- [x] Full `WasmStorage` CRUD implementation using rexie (`get`, `set`, `delete`, `exists`, `list`)
- [x] Added `js-sys` dependency for `Uint8Array` conversion
- [x] `KeyRange::bound()` for prefix queries in `list()`
- [x] Database: `"ensemble_emulator"`, store: `"storage"`, version 1

> Implemented in `frontend/src/frontend/storage.rs` (wasm module, lines 374–518)

### ✅ Phase 2: File Caching on Upload — COMPLETE
- [x] ROMs cached to `data/roms/<name>` via `rom_cache_key()` in `spawn_rom_picker` and `spawn_rom_picker_for_savestate`
- [x] Savestates cached to `data/uploads/savestates/<name>` via `uploaded_savestate_key()` in `spawn_savestate_picker`
- [x] Helper functions: `rom_cache_key()`, `roms_prefix()`, `uploaded_savestate_key()`
- [x] Palettes excluded by design

> Implemented in `frontend/src/frontend/util.rs` (caching calls) and `frontend/src/frontend/storage.rs` (key helpers)

### ✅ Phase 3: WASM Quick/Auto Saves — COMPLETE
- [x] `handle_quicksave`: cfg-gated `spawn_local` + `storage.set()` for WASM
- [x] `create_auto_save`: cfg-gated `spawn_local` + `storage.set()` for WASM
- [x] `cleanup_old_autosaves_async`: cfg-gated `spawn_local` + `storage.list()`/`delete()` for WASM
- [x] `handle_quickload`: full async flow on WASM (list → find newest → read → verify → load)
- [x] Extracted shared `find_newest_quicksave()` used by both native and WASM paths

> **Note**: The original plan mentioned adding a `QuickloadData` message variant, but the implementation used a simpler approach: the WASM quickload does the full flow in `spawn_local` and sends `LoadSaveState` directly to the emulator, avoiding a new message variant.

> Implemented in:
> - `frontend/src/frontend/egui/message_handlers/emulator_handler.rs` (quicksave)
> - `frontend/src/frontend/egui_frontend.rs` (autosave, cleanup, find_newest_quicksave)
> - `frontend/src/frontend/egui/message_handlers/async_handler.rs` (quickload)

### ✅ Phase 4: ROM Matching on WASM — COMPLETE
- [x] `find_matching_rom_in_storage()` async function scans IndexedDB ROM cache
- [x] Direct filename lookup first (fast path), then full scan by SHA256 checksum
- [x] Wired into `handle_savestate_loaded` with `#[cfg(target_arch = "wasm32")]`

> Implemented in `frontend/src/frontend/egui/message_handlers/async_handler.rs`

### ✅ Phase 5: Save Browser UI — COMPLETE
- [x] Save listing dialog/panel (egui window listing quick/auto saves from storage)
- [x] Export/download functionality (read from storage, write to host filesystem via `rfd`)
- [x] Menu integration (`File → Savestates → Browse Saves...`, gated on ROM loaded)
- [x] `SaveBrowserState` in `PendingDialogs`
- [x] `SaveEntry`/`SaveEntryType` data types for quicksaves and autosaves
- [x] Filter toggles for quicksaves/autosaves
- [x] Load and Export buttons per entry
- [x] ROM checksum verification before loading
- [x] Close button available during loading state

> Implemented in:
> - `frontend/src/frontend/egui/ui/save_browser.rs` (UI rendering)
> - `frontend/src/frontend/egui/config.rs` (SaveBrowserState, SaveEntry, SaveEntryType)
> - `frontend/src/frontend/messages.rs` (OpenSaveBrowser, SaveBrowserLoaded, LoadSaveFromBrowser, ExportSaveFromBrowser)
> - `frontend/src/frontend/egui/message_handlers/async_handler.rs` (handlers, list_save_entries, parse_save_entry, ExportableData)
> - `frontend/src/frontend/egui/ui/menu_bar.rs` ("Browse Saves..." menu item)

---

## 5. Dependencies

Already in `Cargo.toml` for `wasm32`:
```toml
rexie = "0.6.2"
web-sys = { version = "0.3", features = ["Window", "Document", "HtmlCanvasElement"] }
wasm-bindgen = "0.2.108"
wasm-bindgen-futures = "0.4.58"
```

Note: IndexedDB-related `web-sys` features (`IdbDatabase`, `IdbObjectStore`, `IdbTransaction`, `IdbKeyRange`) are pulled in transitively through rexie's `idb` dependency — they do not need to be declared explicitly.

May need to add:
```toml
js-sys = "0.3"  # For Uint8Array conversion (may already be transitive)
```

No additional libraries required. The rexie + wasm-bindgen stack provides everything needed.
