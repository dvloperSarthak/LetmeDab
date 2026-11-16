pub mod animation;
pub mod builtin;
pub mod client;
pub mod easing;
pub mod format;
pub mod keybind;
pub mod model;
pub mod network;
pub mod plugin;
pub mod registry;
pub mod server;

// Re-export common types
pub use animation::AnimationInstance;
pub use builtin::get_builtin_emotes;
pub use client::EmotecraftClient;
pub use easing::Easing;
pub use format::{Emote, EmoteFile, ProcessedKeyframe};
pub use keybind::{KeyCode, KeybindAction, KeybindManager, Keybinding};
pub use model::{BodyPart, PartTransform, Pose};
pub use network::{EmotePacket, EMOTECRAFT_CHANNEL, PROTOCOL_VERSION};
pub use plugin::HorizonEmotecraftPlugin;
pub use registry::EmoteRegistry;
pub use server::{EmotecraftServer, ServerConfig};
