//! Cross-platform storage abstraction for native and WASM environments.
//!
//! This module provides a unified interface for persistent storage that works across:
//! - **Native**: Uses the file system via the `directories` crate for OS-appropriate paths
//! - **WASM**: Uses IndexedDB via `indexed_db_futures` for structured data storage
//!
//! # Architecture
//!
//! The storage system is built around three main concepts:
//!
//! 1. **StorageKey**: Identifies what data is being stored (config, saves, palettes, etc.)
//! 2. **Storage trait**: Async interface for get/set/delete/list operations
//! 3. **Platform-specific implementations**: NativeStorage and WasmStorage
//!
//! # Usage
//!
//! ```ignore
//! // Get the platform-appropriate storage instance
//! let storage = get_storage();
//!
//! // Save some data
//! storage.set("saves/my_game/quicksave.sav", data).await?;
//!
//! // Read it back
//! let data = storage.get("saves/my_game/quicksave.sav").await?;
//!
//! // List all saves for a game
//! let saves = storage.list("saves/my_game/").await?;
//! ```
//!
//! # WASM Considerations
//!
//! On WASM, storage has different characteristics:
//! - **localStorage**: ~5MB limit, synchronous, string-only (not suitable for binary data)
//! - **IndexedDB**: Larger storage (~50MB+), async, supports binary data (recommended)
//!
//! This module uses IndexedDB for WASM to support save states and other binary data.

use async_trait::async_trait;

/// Type alias for async storage results
pub type StorageResult<T> = Result<T, StorageError>;

/// Errors that can occur during storage operations
#[derive(Debug, Clone)]
pub enum StorageError {
    /// The requested key was not found
    NotFound,
    /// Failed to read data
    ReadError(String),
    /// Failed to write data
    WriteError(String),
    /// Failed to delete data
    DeleteError(String),
    /// Storage is not available on this platform
    NotAvailable,
    /// Serialization/deserialization error
    SerializationError(String),
    /// IndexedDB specific error (WASM only)
    #[cfg(target_arch = "wasm32")]
    IndexedDbError(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::NotFound => write!(f, "Key not found"),
            StorageError::ReadError(e) => write!(f, "Read error: {}", e),
            StorageError::WriteError(e) => write!(f, "Write error: {}", e),
            StorageError::DeleteError(e) => write!(f, "Delete error: {}", e),
            StorageError::NotAvailable => write!(f, "Storage not available"),
            StorageError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            #[cfg(target_arch = "wasm32")]
            StorageError::IndexedDbError(e) => write!(f, "IndexedDB error: {}", e),
        }
    }
}

impl std::error::Error for StorageError {}

/// Metadata about a stored item
#[derive(Debug, Clone)]
pub struct StorageMetadata {
    /// The key/path of the item
    pub key: String,
    /// Size in bytes (if available)
    pub size: Option<u64>,
    /// Last modified timestamp (if available)
    pub modified: Option<u64>,
}

/// Storage categories for organizing data
///
/// These categories help organize data and may map to different directories
/// on native platforms or different IndexedDB object stores on WASM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageCategory {
    /// Application configuration (config.toml, keybindings, etc.)
    Config,
    /// User data (save states, quicksaves, autosaves)
    Data,
    /// Cached data that can be regenerated (thumbnails, compiled shaders)
    Cache,
}

impl StorageCategory {
    /// Get the string prefix for this category
    pub fn prefix(&self) -> &'static str {
        match self {
            StorageCategory::Config => "config",
            StorageCategory::Data => "data",
            StorageCategory::Cache => "cache",
        }
    }
}

/// Async storage interface that works across platforms.
///
/// All operations are async to support both native (thread-based) and
/// WASM (Promise-based) implementations.
///
/// Note: On WASM, futures don't need to be Send since JavaScript is single-threaded.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Storage: Send + Sync {
    /// Get data by key
    async fn get(&self, key: &str) -> StorageResult<Vec<u8>>;

    /// Set data for a key
    async fn set(&self, key: &str, data: Vec<u8>) -> StorageResult<()>;

    /// Delete data by key
    async fn delete(&self, key: &str) -> StorageResult<()>;

    /// Check if a key exists
    async fn exists(&self, key: &str) -> StorageResult<bool>;

    /// List all keys with a given prefix
    async fn list(&self, prefix: &str) -> StorageResult<Vec<StorageMetadata>>;

    /// Get the full path/URL for a key (for display purposes)
    fn get_display_path(&self, key: &str) -> String;
}

// ============================================================================
// Native Implementation
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::*;
    use directories::ProjectDirs;
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use std::sync::OnceLock;

    const APP_QUALIFIER: &str = "com";
    const APP_ORGANIZATION: &str = "Lightsong";
    const APP_NAME: &str = "EnsembleEmulator";

    static PROJECT_DIRS: OnceLock<Option<ProjectDirs>> = OnceLock::new();

    fn get_project_dirs() -> Option<&'static ProjectDirs> {
        PROJECT_DIRS
            .get_or_init(|| ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME))
            .as_ref()
    }

    /// Native file system storage implementation
    pub struct NativeStorage;

    impl NativeStorage {
        pub fn new() -> Self {
            NativeStorage
        }

        fn get_base_dir(&self, category: StorageCategory) -> Option<PathBuf> {
            let dirs = get_project_dirs()?;
            let base = match category {
                StorageCategory::Config => dirs.config_dir(),
                StorageCategory::Data => dirs.data_dir(),
                StorageCategory::Cache => dirs.cache_dir(),
            };
            Some(base.to_path_buf())
        }

        pub fn key_to_path(&self, key: &str) -> Option<PathBuf> {
            // Parse the category from the key prefix
            let (category, rest) = if let Some(rest) = key.strip_prefix("config/") {
                (StorageCategory::Config, rest)
            } else if let Some(rest) = key.strip_prefix("data/") {
                (StorageCategory::Data, rest)
            } else if let Some(rest) = key.strip_prefix("cache/") {
                (StorageCategory::Cache, rest)
            } else {
                // Default to data category
                (StorageCategory::Data, key)
            };

            let base = self.get_base_dir(category)?;
            Some(base.join(rest))
        }
    }

    #[async_trait]
    impl Storage for NativeStorage {
        async fn get(&self, key: &str) -> StorageResult<Vec<u8>> {
            let path = self
                .key_to_path(key)
                .ok_or(StorageError::NotAvailable)?;

            if !path.exists() {
                return Err(StorageError::NotFound);
            }

            let mut file = std::fs::File::open(&path)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;

            let mut data = Vec::new();
            file.read_to_end(&mut data)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;

            Ok(data)
        }

        async fn set(&self, key: &str, data: Vec<u8>) -> StorageResult<()> {
            let path = self
                .key_to_path(key)
                .ok_or(StorageError::NotAvailable)?;

            // Create parent directories
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;
            }

            let mut file = std::fs::File::create(&path)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;

            file.write_all(&data)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;

            Ok(())
        }

        async fn delete(&self, key: &str) -> StorageResult<()> {
            let path = self
                .key_to_path(key)
                .ok_or(StorageError::NotAvailable)?;

            if path.exists() {
                std::fs::remove_file(&path)
                    .map_err(|e| StorageError::DeleteError(e.to_string()))?;
            }

            Ok(())
        }

        async fn exists(&self, key: &str) -> StorageResult<bool> {
            let path = self
                .key_to_path(key)
                .ok_or(StorageError::NotAvailable)?;

            Ok(path.exists())
        }

        async fn list(&self, prefix: &str) -> StorageResult<Vec<StorageMetadata>> {
            let base_path = self
                .key_to_path(prefix)
                .ok_or(StorageError::NotAvailable)?;

            if !base_path.exists() {
                return Ok(Vec::new());
            }

            let mut results = Vec::new();

            if base_path.is_dir() {
                Self::collect_files(&base_path, prefix, &mut results)?;
            } else if base_path.is_file() {
                if let Ok(metadata) = std::fs::metadata(&base_path) {
                    results.push(StorageMetadata {
                        key: prefix.to_string(),
                        size: Some(metadata.len()),
                        modified: metadata
                            .modified()
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs()),
                    });
                }
            }

            Ok(results)
        }

        fn get_display_path(&self, key: &str) -> String {
            self.key_to_path(key)
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| key.to_string())
        }
    }

    impl NativeStorage {
        fn collect_files(
            dir: &PathBuf,
            prefix: &str,
            results: &mut Vec<StorageMetadata>,
        ) -> StorageResult<()> {
            let entries = std::fs::read_dir(dir)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;

            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let key = if prefix.ends_with('/') {
                    format!("{}{}", prefix, name)
                } else {
                    format!("{}/{}", prefix, name)
                };

                if path.is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        results.push(StorageMetadata {
                            key,
                            size: Some(metadata.len()),
                            modified: metadata
                                .modified()
                                .ok()
                                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                .map(|d| d.as_secs()),
                        });
                    }
                } else if path.is_dir() {
                    Self::collect_files(&path, &key, results)?;
                }
            }

            Ok(())
        }
    }
}

// ============================================================================
// WASM Implementation
// ============================================================================

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;

    /// WASM storage implementation using IndexedDB
    ///
    /// This provides persistent storage in the browser using IndexedDB,
    /// which supports larger storage limits and binary data.
    ///
    /// # Database Structure
    ///
    /// - Database name: "ensemble_emulator"
    /// - Object store: "storage" (key-value pairs where key is the storage path)
    ///
    /// # Dependencies Required
    ///
    /// Add to Cargo.toml:
    /// ```toml
    /// [target.'cfg(target_arch = "wasm32")'.dependencies]
    /// indexed_db_futures = "0.4"
    /// js-sys = "0.3"
    /// ```
    pub struct WasmStorage;

    impl WasmStorage {
        pub fn new() -> Self {
            WasmStorage
        }
    }

    // Note: The actual implementation requires the indexed_db_futures crate.
    // For now, we provide a stub that returns NotAvailable errors.
    //
    // When implementing, use:
    // - indexed_db_futures::IdbDatabase for database access
    // - Store binary data directly as Uint8Array
    // - Use key paths that match our storage key format
    //
    // Example implementation pattern:
    // ```rust
    // async fn open_db() -> Result<IdbDatabase, StorageError> {
    //     let mut db_req = IdbDatabase::open("ensemble_emulator")?;
    //     db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| {
    //         if !evt.db().object_store_names().contains(&"storage") {
    //             evt.db().create_object_store("storage")?;
    //         }
    //         Ok(())
    //     }));
    //     db_req.await.map_err(|e| StorageError::IndexedDbError(e.to_string()))
    // }
    // ```

    #[async_trait(?Send)]
    impl Storage for WasmStorage {
        async fn get(&self, _key: &str) -> StorageResult<Vec<u8>> {
            // TODO: Implement IndexedDB read
            // 1. Open database
            // 2. Create read transaction
            // 3. Get value from object store
            // 4. Convert Uint8Array to Vec<u8>
            Err(StorageError::NotAvailable)
        }

        async fn set(&self, _key: &str, _data: Vec<u8>) -> StorageResult<()> {
            // TODO: Implement IndexedDB write
            // 1. Open database
            // 2. Create readwrite transaction
            // 3. Put value in object store
            Err(StorageError::NotAvailable)
        }

        async fn delete(&self, _key: &str) -> StorageResult<()> {
            // TODO: Implement IndexedDB delete
            Err(StorageError::NotAvailable)
        }

        async fn exists(&self, _key: &str) -> StorageResult<bool> {
            // TODO: Implement IndexedDB exists check
            Err(StorageError::NotAvailable)
        }

        async fn list(&self, _prefix: &str) -> StorageResult<Vec<StorageMetadata>> {
            // TODO: Implement IndexedDB list
            // Use IDBKeyRange.bound(prefix, prefix + '\uffff') to get all keys with prefix
            Err(StorageError::NotAvailable)
        }

        fn get_display_path(&self, key: &str) -> String {
            format!("indexeddb://ensemble_emulator/{}", key)
        }
    }
}

// ============================================================================
// Platform Selection
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
pub use native::NativeStorage;

#[cfg(target_arch = "wasm32")]
pub use wasm::WasmStorage;

/// Get the platform-appropriate storage implementation
#[cfg(not(target_arch = "wasm32"))]
pub fn get_storage() -> impl Storage {
    NativeStorage::new()
}

/// Get the platform-appropriate storage implementation
#[cfg(target_arch = "wasm32")]
pub fn get_storage() -> impl Storage {
    WasmStorage::new()
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Generate a storage key for a quicksave
pub fn quicksave_key(game_name: &str, timestamp: &str) -> String {
    format!("data/saves/{}/quicksaves/quicksave_{}.sav", game_name, timestamp)
}

/// Generate a storage key for an autosave
pub fn autosave_key(game_name: &str, timestamp: &str) -> String {
    format!("data/saves/{}/autosaves/autosave_{}.sav", game_name, timestamp)
}

/// Generate the prefix for listing autosaves for a game
pub fn autosaves_prefix(game_name: &str) -> String {
    format!("data/saves/{}/autosaves/", game_name)
}

/// Generate the prefix for listing quicksaves for a game
pub fn quicksaves_prefix(game_name: &str) -> String {
    format!("data/saves/{}/quicksaves/", game_name)
}

/// Generate a storage key for the application config
pub fn config_key() -> String {
    "config/config.toml".to_string()
}

/// Generate a storage key for egui state
pub fn egui_state_key() -> String {
    "config/egui_state".to_string()
}

// ============================================================================
// Synchronous Wrappers (Native Only)
// ============================================================================
//
// These provide synchronous access to storage for code that can't be async,
// such as startup config loading and shutdown config saving.

#[cfg(not(target_arch = "wasm32"))]
mod sync_wrappers {
    use super::*;

    /// Global storage instance for synchronous access
    static STORAGE: std::sync::OnceLock<NativeStorage> = std::sync::OnceLock::new();

    fn get_storage_instance() -> &'static NativeStorage {
        STORAGE.get_or_init(NativeStorage::new)
    }

    /// Get the full filesystem path for a storage key (native only)
    pub fn get_path_for_key(key: &str) -> Option<std::path::PathBuf> {
        get_storage_instance().key_to_path(key)
    }

    /// Read data synchronously from storage
    pub fn read_sync(key: &str) -> StorageResult<Vec<u8>> {
        let storage = get_storage_instance();
        let path = storage
            .key_to_path(key)
            .ok_or(StorageError::NotAvailable)?;

        if !path.exists() {
            return Err(StorageError::NotFound);
        }

        std::fs::read(&path).map_err(|e| StorageError::ReadError(e.to_string()))
    }

    /// Write data synchronously to storage
    pub fn write_sync(key: &str, data: &[u8]) -> StorageResult<()> {
        let storage = get_storage_instance();
        let path = storage
            .key_to_path(key)
            .ok_or(StorageError::NotAvailable)?;

        // Create parent directories
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
        }

        std::fs::write(&path, data).map_err(|e| StorageError::WriteError(e.to_string()))
    }

    /// Delete data synchronously from storage
    pub fn delete_sync(key: &str) -> StorageResult<()> {
        let storage = get_storage_instance();
        let path = storage
            .key_to_path(key)
            .ok_or(StorageError::NotAvailable)?;

        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| StorageError::DeleteError(e.to_string()))?;
        }

        Ok(())
    }

    /// Check if a key exists synchronously
    pub fn exists_sync(key: &str) -> StorageResult<bool> {
        let storage = get_storage_instance();
        let path = storage
            .key_to_path(key)
            .ok_or(StorageError::NotAvailable)?;

        Ok(path.exists())
    }

    /// List all keys with a given prefix synchronously
    pub fn list_sync(prefix: &str) -> StorageResult<Vec<StorageMetadata>> {
        let storage = get_storage_instance();
        let base_path = storage
            .key_to_path(prefix)
            .ok_or(StorageError::NotAvailable)?;

        if !base_path.exists() {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();

        if base_path.is_dir() {
            collect_files_sync(&base_path, prefix, &mut results)?;
        } else if base_path.is_file() {
            if let Ok(metadata) = std::fs::metadata(&base_path) {
                results.push(StorageMetadata {
                    key: prefix.to_string(),
                    size: Some(metadata.len()),
                    modified: metadata
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs()),
                });
            }
        }

        Ok(results)
    }

    /// Get the display path for a key
    pub fn get_display_path(key: &str) -> String {
        get_storage_instance().get_display_path(key)
    }

    fn collect_files_sync(
        dir: &std::path::PathBuf,
        prefix: &str,
        results: &mut Vec<StorageMetadata>,
    ) -> StorageResult<()> {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| StorageError::ReadError(e.to_string()))?;

        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let key = if prefix.ends_with('/') {
                format!("{}{}", prefix, name)
            } else {
                format!("{}/{}", prefix, name)
            };

            if path.is_file() {
                if let Ok(metadata) = entry.metadata() {
                    results.push(StorageMetadata {
                        key,
                        size: Some(metadata.len()),
                        modified: metadata
                            .modified()
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs()),
                    });
                }
            } else if path.is_dir() {
                collect_files_sync(&path, &key, results)?;
            }
        }

        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use sync_wrappers::*;

// ============================================================================
// Migration Notes
// ============================================================================
//
// To migrate the existing code to use this storage abstraction:
//
// 1. **persistence.rs**: Replace direct file operations with Storage trait calls
//    - `get_config_dir()` → Use `config_key()` with storage
//    - `get_data_dir()` → Use `data/` prefix keys with storage
//    - `load_config()` → `storage.get(config_key()).await`
//    - `save_config()` → `storage.set(config_key(), data).await`
//
// 2. **util.rs**: Update file pickers to work with async storage
//    - `spawn_palette_picker` → Use async/await with storage
//    - `spawn_save_dialog` → Use storage.set() instead of File::create
//
// 3. **egui_frontend.rs**: Update quicksave/autosave handling
//    - `get_current_quicksave_path()` → Use storage.list() with async
//    - `create_auto_save()` → Use storage.set() with autosave_key()
//    - `cleanup_old_autosaves_async()` → Use storage.list() and storage.delete()
//
// 4. **Dependencies**: Add to frontend/Cargo.toml:
//    ```toml
//    [target.'cfg(target_arch = "wasm32")'.dependencies]
//    indexed_db_futures = "0.4"
//    ```
//
// 5. **Thread Spawning**: Replace std::thread::spawn with:
//    - Native: tokio::spawn
//    - WASM: wasm_bindgen_futures::spawn_local
//    (Already partially done with spawn_async in util.rs)
