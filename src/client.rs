use crate::animation::AnimationInstance;
use crate::format::EmoteFile;
use crate::model::Pose;
use crate::network::{EmoteHeader, EmotePacket, EMOTECRAFT_CHANNEL, PROTOCOL_VERSION};
use crate::registry::EmoteRegistry;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Client-side Emotecraft manager for Horizon Minecraft.
pub struct EmotecraftClient {
    pub local_player: Uuid,
    pub registry: Arc<EmoteRegistry>,
    pub active_animations: HashMap<Uuid, AnimationInstance>,
    pub server_emotes: Vec<EmoteHeader>,
    pub is_connected_to_emotecraft_server: bool,
}

impl EmotecraftClient {
    pub fn new(local_player: Uuid, registry: Arc<EmoteRegistry>) -> Self {
        Self {
            local_player,
            registry,
            active_animations: HashMap::new(),
            server_emotes: Vec::new(),
            is_connected_to_emotecraft_server: false,
        }
    }

    /// Create initial handshake payload to send to server on join.
    pub fn create_handshake_packet(&self) -> Result<Vec<u8>, String> {
        let packet = EmotePacket::Handshake {
            protocol_version: PROTOCOL_VERSION,
            mod_version: env!("CARGO_PKG_VERSION").to_string(),
        };
        packet.encode().map_err(|e| e.to_string())
    }

    /// Play an emote for the local player.
    /// Returns the raw custom payload to send to the server over `emotecraft:main`.
    pub fn play_emote(&mut self, emote_id: &str) -> Result<Vec<u8>, String> {
        let emote = self
            .registry
            .get_by_id(emote_id)
            .ok_or_else(|| format!("Emote '{}' not found in registry", emote_id))?;

        let instance = AnimationInstance::new(emote.clone());
        self.active_animations.insert(self.local_player, instance);

        // Build play packet
        let packet = EmotePacket::Play {
            player_uuid: self.local_player,
            emote_id: emote.id.clone(),
            is_loop: emote.is_loop,
            start_tick: emote.begin_tick,
            raw_emote_json: None,
        };

        packet.encode().map_err(|e| e.to_string())
    }

    /// Stop the local player's active emote.
    /// Returns the raw stop payload to send to the server.
    pub fn stop_emote(&mut self, fade_ticks: u32) -> Result<Vec<u8>, String> {
        if let Some(instance) = self.active_animations.get_mut(&self.local_player) {
            instance.stop(fade_ticks as f32);
        }

        let packet = EmotePacket::Stop {
            player_uuid: self.local_player,
            fade_ticks,
        };

        packet.encode().map_err(|e| e.to_string())
    }

    /// Process a packet received from the server.
    pub fn handle_server_packet(&mut self, channel: &str, data: &[u8]) -> Result<(), String> {
        if channel != EMOTECRAFT_CHANNEL {
            return Ok(());
        }

        let packet = EmotePacket::decode(data).map_err(|e| format!("Decode error: {}", e))?;

        match packet {
            EmotePacket::Handshake { protocol_version, .. } => {
                log::info!("Connected to Emotecraft server (protocol v{})", protocol_version);
                self.is_connected_to_emotecraft_server = true;
            }

            EmotePacket::SyncList { emotes } => {
                log::info!("Received {} emotes from server", emotes.len());
                self.server_emotes = emotes;
            }

            EmotePacket::Play {
                player_uuid,
                emote_id,
                is_loop,
                start_tick,
                raw_emote_json,
            } => {
                // If this is for another player, start animating them
                let emote_opt = if let Some(ref json) = raw_emote_json {
                    EmoteFile::from_str(json)
                        .ok()
                        .and_then(|ef| ef.compile().ok())
                        .map(Arc::new)
                } else {
                    self.registry.get_by_id(&emote_id)
                };

                if let Some(emote) = emote_opt {
                    let mut instance = AnimationInstance::new(emote);
                    instance.current_tick = start_tick as f32;
                    instance.is_looping = is_loop;
                    self.active_animations.insert(player_uuid, instance);
                } else {
                    log::warn!("Received unknown emote '{}' for player {}", emote_id, player_uuid);
                }
            }

            EmotePacket::Stop {
                player_uuid,
                fade_ticks,
            } => {
                if let Some(instance) = self.active_animations.get_mut(&player_uuid) {
                    instance.stop(fade_ticks as f32);
                }
            }
        }

        Ok(())
    }

    /// Advance client-side animations.
    /// Can be called every render frame (with fractional delta_ticks) or every game tick.
    pub fn update(&mut self, delta_ticks: f32) {
        self.active_animations.retain(|_, instance| {
            instance.update(delta_ticks);
            instance.is_playing
        });
    }

    /// Sample the current emote pose for a player to inject into the Horizon model renderer.
    /// Returns `Some(Pose)` if the player is emoting, or `None` if they are in normal vanilla pose.
    pub fn sample_player_pose(&self, player: &Uuid) -> Option<Pose> {
        self.active_animations.get(player).map(|instance| instance.sample())
    }

    /// Check if the local player is currently emoting.
    pub fn is_local_emoting(&self) -> bool {
        self.active_animations
            .get(&self.local_player)
            .map(|i| i.is_playing)
            .unwrap_or(false)
    }
}
