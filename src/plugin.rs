use crate::client::EmotecraftClient;
use crate::keybind::{KeyCode, KeybindAction, KeybindManager};
use crate::model::Pose;
use crate::registry::EmoteRegistry;
use crate::server::{EmotecraftServer, ServerConfig};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Side/environment where the mod is executing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentSide {
    Client,
    Server,
    Both,
}

/// The Horizon Emotecraft Plugin instance.
pub struct HorizonEmotecraftPlugin {
    pub side: EnvironmentSide,
    pub registry: Arc<EmoteRegistry>,
    pub server: Option<Mutex<EmotecraftServer>>,
    pub client: Option<Mutex<EmotecraftClient>>,
    pub keybinds: Mutex<KeybindManager>,
}

impl HorizonEmotecraftPlugin {
    /// Initialize plugin for Horizon server.
    pub fn new_server(config: ServerConfig) -> Self {
        let registry = Arc::new(EmoteRegistry::new());
        let server = EmotecraftServer::new(registry.clone(), config);
        Self {
            side: EnvironmentSide::Server,
            registry,
            server: Some(Mutex::new(server)),
            client: None,
            keybinds: Mutex::new(KeybindManager::new()),
        }
    }

    /// Initialize plugin for Horizon client.
    pub fn new_client(local_player: Uuid) -> Self {
        let registry = Arc::new(EmoteRegistry::new());
        let client = EmotecraftClient::new(local_player, registry.clone());
        Self {
            side: EnvironmentSide::Client,
            registry,
            server: None,
            client: Some(Mutex::new(client)),
            keybinds: Mutex::new(KeybindManager::new()),
        }
    }

    /// Initialize unified plugin (for singleplayer/integrated server in Horizon).
    pub fn new_integrated(local_player: Uuid, config: ServerConfig) -> Self {
        let registry = Arc::new(EmoteRegistry::new());
        let server = EmotecraftServer::new(registry.clone(), config);
        let client = EmotecraftClient::new(local_player, registry.clone());
        Self {
            side: EnvironmentSide::Both,
            registry,
            server: Some(Mutex::new(server)),
            client: Some(Mutex::new(client)),
            keybinds: Mutex::new(KeybindManager::new()),
        }
    }

    /// Hook: Handle keyboard press from Horizon input loop.
    pub fn on_key_down(&self, key: KeyCode) -> Option<KeybindAction> {
        let mut keybinds = self.keybinds.lock().unwrap();
        let action = keybinds.handle_key_press(key)?;

        match &action {
            KeybindAction::OpenWheel => {
                log::info!("Emote wheel toggled (state: {})", keybinds.wheel.is_open);
            }
            KeybindAction::StopEmote => {
                if let Some(ref client_mutex) = self.client {
                    let mut client = client_mutex.lock().unwrap();
                    let _ = client.stop_emote(5);
                }
            }
            KeybindAction::PlayEmote(id) => {
                if let Some(ref client_mutex) = self.client {
                    let mut client = client_mutex.lock().unwrap();
                    let _ = client.play_emote(id);
                }
            }
        }

        Some(action)
    }

    /// Hook: Execute an in-game command such as `/emote play dab` or `/emote list`.
    pub fn handle_command(&self, args: &[&str]) -> Result<String, String> {
        if args.is_empty() {
            return Ok("Usage: /emote <play|stop|list|help> [args]".to_string());
        }

        match args[0].to_lowercase().as_str() {
            "play" => {
                if args.len() < 2 {
                    return Err("Usage: /emote play <name>".to_string());
                }
                let emote_id = args[1];
                if let Some(ref client_mutex) = self.client {
                    let mut client = client_mutex.lock().unwrap();
                    client.play_emote(emote_id)?;
                    Ok(format!("Playing emote: {}", emote_id))
                } else {
                    Err("Emote play command can only be executed on client".to_string())
                }
            }
            "stop" => {
                if let Some(ref client_mutex) = self.client {
                    let mut client = client_mutex.lock().unwrap();
                    client.stop_emote(5)?;
                    Ok("Stopped active emote".to_string())
                } else {
                    Err("Emote stop command can only be executed on client".to_string())
                }
            }
            "list" => {
                let emotes = self.registry.all();
                let mut output = format!("Available Emotes ({} total):\n", emotes.len());
                for e in emotes {
                    output.push_str(&format!("  - {} ({}) by {}\n", e.name, e.id, e.author));
                }
                Ok(output)
            }
            "help" => Ok("Emotecraft Horizon Commands:\n  /emote play <id> - Play an emote (e.g. dab)\n  /emote stop - Stop current emote\n  /emote list - List available emotes\nDefault Keybinds: 'B' = Wheel, 'X' = Stop, 'G' = Quick Dab, 'H' = Quick Wave".to_string()),
            other => Err(format!("Unknown emote subcommand '{}'. Try /emote help", other)),
        }
    }

    /// Hook: Inject into Horizon model renderer for a humanoid entity.
    /// Modifies the provided vanilla pose with the current emote transforms.
    pub fn apply_model_pose(&self, entity_uuid: &Uuid, vanilla_pose: &mut Pose) {
        if let Some(ref client_mutex) = self.client {
            let client = client_mutex.lock().unwrap();
            if let Some(emote_pose) = client.sample_player_pose(entity_uuid) {
                // Layer emote pose over vanilla model pose
                *vanilla_pose = vanilla_pose.blend_additive(&emote_pose, 1.0);
            }
        }
    }
}
