//! LetmeDab Native Runtime for Horizon Minecraft.
use std::sync::atomic::{AtomicBool, Ordering};

pub static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn init() {
    if !INITIALIZED.swap(true, Ordering::SeqCst) {
        log::info!("LetmeDab native runtime initialized for Horizon");
    }
}

pub fn handle_tick() {
    // Native animation and keybind tick step
}

pub fn handle_command(cmd: &str) -> Option<String> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }
    match parts[0] {
        "emote" | "letmedab" => {
            if parts.len() < 2 {
                return Some("Usage: /emote <play|stop|list|help>".to_string());
            }
            match parts[1] {
                "play" => {
                    let emote = parts.get(2).unwrap_or(&"dab");
                    Some(format!("Playing emote '{}'", emote))
                }
                "stop" => Some("Stopped active emote".to_string()),
                "list" => Some("32 built-in emotes available: dab, wave, bow, cheer, salute, facepalm, tpose, shrug, clap, cry, laugh, point, think, peace, heart, beckon, slow_clap, chuckle, surrender, sit, sleep, zombie, ninja, flex, floss, gangnam, spin, guitar, disco, robot, pushup, jumping_jack".to_string()),
                "help" => Some("LetmeDab commands: /emote play <id>, /emote stop, /emote list. Default keybinds: 'B' = Wheel, 'X' = Stop, 'G' = Quick Dab, 'H' = Quick Wave".to_string()),
                _ => Some("Unknown subcommand. Try /emote help".to_string()),
            }
        }
        _ => None,
    }
}
