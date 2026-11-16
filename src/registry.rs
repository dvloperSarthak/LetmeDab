use crate::builtin::get_builtin_emotes;
use crate::format::{Emote, EmoteFile};
use crate::network::EmoteHeader;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct EmoteRegistry {
    by_id: HashMap<String, Arc<Emote>>,
    by_uuid: HashMap<Uuid, Arc<Emote>>,
}

impl EmoteRegistry {
    /// Create a new registry and register all standard built-in emotes.
    pub fn new() -> Self {
        let mut registry = Self::default();
        registry.register_builtins();
        registry
    }

    /// Register default built-in emotes (Dab, Wave, Bow, Cheer, Salute, Facepalm, T-Pose).
    pub fn register_builtins(&mut self) {
        for emote in get_builtin_emotes() {
            self.register(emote);
        }
    }

    /// Register an emote into the registry.
    pub fn register(&mut self, emote: Arc<Emote>) {
        self.by_id.insert(emote.id.to_lowercase(), emote.clone());
        self.by_uuid.insert(emote.uuid, emote);
    }

    /// Find an emote by ID or name (case-insensitive).
    pub fn get_by_id(&self, id: &str) -> Option<Arc<Emote>> {
        let clean = id.to_lowercase().replace(' ', "_");
        self.by_id.get(&clean).cloned()
    }

    /// Find an emote by UUID.
    pub fn get_by_uuid(&self, uuid: &Uuid) -> Option<Arc<Emote>> {
        self.by_uuid.get(uuid).cloned()
    }

    /// List all registered emotes.
    pub fn all(&self) -> Vec<Arc<Emote>> {
        self.by_id.values().cloned().collect()
    }

    /// Get lightweight headers for network synchronization.
    pub fn get_headers(&self) -> Vec<EmoteHeader> {
        self.by_id
            .values()
            .map(|e| EmoteHeader {
                id: e.id.clone(),
                name: e.name.clone(),
                author: e.author.clone(),
                is_loop: e.is_loop,
                duration_ticks: e.stop_tick,
            })
            .collect()
    }

    /// Load custom emotes from a directory.
    pub fn load_directory<P: AsRef<Path>>(&mut self, dir_path: P) -> usize {
        let dir = dir_path.as_ref();
        if !dir.exists() || !dir.is_dir() {
            return 0;
        }

        let mut loaded = 0;
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext.eq_ignore_ascii_case("json") || ext.eq_ignore_ascii_case("emotecraft") {
                        match EmoteFile::from_file(&path) {
                            Ok(file) => match file.compile() {
                                Ok(emote) => {
                                    log::info!("Loaded emote: {} ({})", emote.name, emote.id);
                                    self.register(Arc::new(emote));
                                    loaded += 1;
                                }
                                Err(err) => {
                                    log::warn!("Failed to compile emote {}: {}", path.display(), err);
                                }
                            },
                            Err(err) => {
                                log::warn!("Failed to parse emote file {}: {}", path.display(), err);
                            }
                        }
                    }
                }
            }
        }
        loaded
    }
}
