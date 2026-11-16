use crate::animation::AnimationInstance;
use crate::format::EmoteFile;
use crate::network::{EmotePacket, EMOTECRAFT_CHANNEL, PROTOCOL_VERSION};
use crate::registry::EmoteRegistry;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

/// Configuration options for the Emotecraft server mod.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub stop_on_move: bool,
    pub stop_on_attack: bool,
    pub stop_on_damage: bool,
    pub allow_custom_emotes: bool,
    pub max_packet_rate_per_sec: u32,
    pub broadcast_radius_blocks: f64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            stop_on_move: true,
            stop_on_attack: true,
            stop_on_damage: true,
            allow_custom_emotes: true,
            max_packet_rate_per_sec: 5,
            broadcast_radius_blocks: 64.0,
        }
    }
}

/// Outgoing packet message to be transmitted by the Horizon server networking layer.
#[derive(Debug, Clone)]
pub struct OutgoingMessage {
    pub recipient: Uuid,
    pub channel: String,
    pub payload: Vec<u8>,
}

/// Server-side Emotecraft manager for Horizon Minecraft.
pub struct EmotecraftServer {
    pub registry: Arc<EmoteRegistry>,
    pub config: ServerConfig,
    active_emotes: HashMap<Uuid, AnimationInstance>,
    player_packet_timestamps: HashMap<Uuid, Vec<Instant>>,
}

impl EmotecraftServer {
    pub fn new(registry: Arc<EmoteRegistry>, config: ServerConfig) -> Self {
        Self {
            registry,
            config,
            active_emotes: HashMap::new(),
            player_packet_timestamps: HashMap::new(),
        }
    }

    /// Check rate limit anti-cheat for a player.
    fn check_rate_limit(&mut self, player: &Uuid) -> bool {
        let now = Instant::now();
        let timestamps = self.player_packet_timestamps.entry(*player).or_default();
        timestamps.retain(|t| now.duration_since(*t).as_secs_f32() < 1.0);
        if timestamps.len() >= self.config.max_packet_rate_per_sec as usize {
            return false;
        }
        timestamps.push(now);
        true
    }

    /// Handle an incoming custom payload packet from a client.
    /// Returns a list of outgoing messages to send to other clients.
    pub fn handle_packet(
        &mut self,
        sender: Uuid,
        channel: &str,
        data: &[u8],
        all_online_players: &[Uuid],
    ) -> Result<Vec<OutgoingMessage>, String> {
        if channel != EMOTECRAFT_CHANNEL {
            return Ok(Vec::new());
        }

        if !self.check_rate_limit(&sender) {
            log::warn!("Rate limit exceeded for player {}", sender);
            return Err("Rate limit exceeded".to_string());
        }

        let packet = EmotePacket::decode(data).map_err(|e| format!("Decode error: {}", e))?;
        let mut outgoing = Vec::new();

        match packet {
            EmotePacket::Handshake {
                protocol_version,
                mod_version,
            } => {
                log::info!(
                    "Emotecraft handshake from player {}: v{} (mod: {})",
                    sender,
                    protocol_version,
                    mod_version
                );

                // Send back server handshake and available emotes
                let response_handshake = EmotePacket::Handshake {
                    protocol_version: PROTOCOL_VERSION,
                    mod_version: env!("CARGO_PKG_VERSION").to_string(),
                };
                let sync_list = EmotePacket::SyncList {
                    emotes: self.registry.get_headers(),
                };

                outgoing.push(OutgoingMessage {
                    recipient: sender,
                    channel: EMOTECRAFT_CHANNEL.to_string(),
                    payload: response_handshake.encode().map_err(|e| e.to_string())?,
                });
                outgoing.push(OutgoingMessage {
                    recipient: sender,
                    channel: EMOTECRAFT_CHANNEL.to_string(),
                    payload: sync_list.encode().map_err(|e| e.to_string())?,
                });
            }

            EmotePacket::Play {
                player_uuid,
                emote_id,
                is_loop,
                start_tick,
                raw_emote_json,
            } => {
                // Ensure sender can only trigger emotes for themselves
                if player_uuid != sender {
                    return Err("Sender UUID mismatch in play packet".to_string());
                }

                // Look up emote in registry or parse custom payload
                let emote_opt = if let Some(ref json) = raw_emote_json {
                    if self.config.allow_custom_emotes {
                        EmoteFile::from_str(json)
                            .ok()
                            .and_then(|ef| ef.compile().ok())
                            .map(Arc::new)
                    } else {
                        None
                    }
                } else {
                    self.registry.get_by_id(&emote_id)
                };

                if let Some(emote) = emote_opt {
                    let mut instance = AnimationInstance::new(emote.clone());
                    instance.current_tick = start_tick as f32;
                    instance.is_looping = is_loop;
                    self.active_emotes.insert(sender, instance);

                    // Broadcast play packet to other connected players
                    let broadcast_packet = EmotePacket::Play {
                        player_uuid: sender,
                        emote_id: emote.id.clone(),
                        is_loop,
                        start_tick,
                        raw_emote_json,
                    };
                    let encoded = broadcast_packet.encode().map_err(|e| e.to_string())?;

                    for &other in all_online_players {
                        if other != sender {
                            outgoing.push(OutgoingMessage {
                                recipient: other,
                                channel: EMOTECRAFT_CHANNEL.to_string(),
                                payload: encoded.clone(),
                            });
                        }
                    }
                } else {
                    log::warn!("Player {} requested unknown emote: {}", sender, emote_id);
                }
            }

            EmotePacket::Stop {
                player_uuid,
                fade_ticks,
            } => {
                if player_uuid != sender {
                    return Err("Sender UUID mismatch in stop packet".to_string());
                }

                if let Some(instance) = self.active_emotes.get_mut(&sender) {
                    instance.stop(fade_ticks as f32);
                }

                let broadcast_packet = EmotePacket::Stop {
                    player_uuid: sender,
                    fade_ticks,
                };
                let encoded = broadcast_packet.encode().map_err(|e| e.to_string())?;

                for &other in all_online_players {
                    if other != sender {
                        outgoing.push(OutgoingMessage {
                            recipient: other,
                            channel: EMOTECRAFT_CHANNEL.to_string(),
                            payload: encoded.clone(),
                        });
                    }
                }
            }

            EmotePacket::SyncList { .. } => {
                // Client does not send SyncList to server
            }
        }

        Ok(outgoing)
    }

    /// Called on player movement. If `stop_on_move` is set, cancels active emote.
    pub fn on_player_move(&mut self, player: Uuid, all_online_players: &[Uuid]) -> Vec<OutgoingMessage> {
        if self.config.stop_on_move && self.active_emotes.contains_key(&player) {
            return self.stop_player_emote(player, 5, all_online_players);
        }
        Vec::new()
    }

    /// Force stop an emote on a player.
    pub fn stop_player_emote(
        &mut self,
        player: Uuid,
        fade_ticks: u32,
        all_online_players: &[Uuid],
    ) -> Vec<OutgoingMessage> {
        self.active_emotes.remove(&player);

        let stop_packet = EmotePacket::Stop {
            player_uuid: player,
            fade_ticks,
        };

        if let Ok(encoded) = stop_packet.encode() {
            all_online_players
                .iter()
                .filter(|&&p| p != player)
                .map(|&p| OutgoingMessage {
                    recipient: p,
                    channel: EMOTECRAFT_CHANNEL.to_string(),
                    payload: encoded.clone(),
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Server game tick update.
    pub fn tick(&mut self, delta_ticks: f32) {
        self.active_emotes.retain(|_, instance| {
            instance.update(delta_ticks);
            instance.is_playing
        });
    }

    /// Handle player disconnect.
    pub fn on_player_disconnect(&mut self, player: &Uuid) {
        self.active_emotes.remove(player);
        self.player_packet_timestamps.remove(player);
    }

    /// Get current active emote instance for a player.
    pub fn get_active_emote(&self, player: &Uuid) -> Option<&AnimationInstance> {
        self.active_emotes.get(player)
    }
}
