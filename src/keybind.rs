use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Keyboard keys supported for Emotecraft in Horizon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ,
    KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT,
    KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Space, Escape, Tab, LeftShift, RightShift, LeftControl,
}

impl KeyCode {
    pub fn from_str_loose(s: &str) -> Option<Self> {
        let clean = s.trim().to_uppercase();
        match clean.as_str() {
            "A" => Some(KeyCode::KeyA),
            "B" => Some(KeyCode::KeyB),
            "C" => Some(KeyCode::KeyC),
            "D" => Some(KeyCode::KeyD),
            "E" => Some(KeyCode::KeyE),
            "F" => Some(KeyCode::KeyF),
            "G" => Some(KeyCode::KeyG),
            "H" => Some(KeyCode::KeyH),
            "I" => Some(KeyCode::KeyI),
            "J" => Some(KeyCode::KeyJ),
            "K" => Some(KeyCode::KeyK),
            "L" => Some(KeyCode::KeyL),
            "M" => Some(KeyCode::KeyM),
            "N" => Some(KeyCode::KeyN),
            "O" => Some(KeyCode::KeyO),
            "P" => Some(KeyCode::KeyP),
            "Q" => Some(KeyCode::KeyQ),
            "R" => Some(KeyCode::KeyR),
            "S" => Some(KeyCode::KeyS),
            "T" => Some(KeyCode::KeyT),
            "U" => Some(KeyCode::KeyU),
            "V" => Some(KeyCode::KeyV),
            "W" => Some(KeyCode::KeyW),
            "X" => Some(KeyCode::KeyX),
            "Y" => Some(KeyCode::KeyY),
            "Z" => Some(KeyCode::KeyZ),
            "SPACE" => Some(KeyCode::Space),
            "ESCAPE" | "ESC" => Some(KeyCode::Escape),
            "TAB" => Some(KeyCode::Tab),
            _ => None,
        }
    }
}

/// Actions that can be triggered by keybinds in Horizon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeybindAction {
    /// Toggle or open the radial Emote Wheel (default: KeyB)
    OpenWheel,
    /// Stop currently active emote (default: KeyX)
    StopEmote,
    /// Quick trigger a specific emote directly (e.g. Dab on 'G')
    PlayEmote(String),
}

/// A configured keybinding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybinding {
    pub id: String,
    pub name: String,
    pub default_key: KeyCode,
    pub current_key: KeyCode,
    pub action: KeybindAction,
}

/// 8-slot Radial Emote Wheel state for fast selection in Horizon UI.
#[derive(Debug, Clone)]
pub struct EmoteWheel {
    pub is_open: bool,
    pub selected_index: usize,
    pub slots: [Option<String>; 8],
}

impl Default for EmoteWheel {
    fn default() -> Self {
        Self {
            is_open: false,
            selected_index: 0,
            slots: [
                Some("dab".to_string()),
                Some("wave".to_string()),
                Some("cheer".to_string()),
                Some("bow".to_string()),
                Some("floss".to_string()),
                Some("shrug".to_string()),
                Some("point".to_string()),
                Some("clap".to_string()),
            ],
        }
    }
}

impl EmoteWheel {
    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn select_slot(&mut self, slot: usize) {
        if slot < 8 {
            self.selected_index = slot;
        }
    }

    pub fn get_selected_emote(&self) -> Option<&str> {
        self.slots[self.selected_index].as_deref()
    }
}

/// Manages keybinds and handles keyboard inputs from Horizon client.
#[derive(Debug, Clone)]
pub struct KeybindManager {
    bindings: HashMap<KeyCode, Keybinding>,
    pub wheel: EmoteWheel,
}

impl Default for KeybindManager {
    fn default() -> Self {
        let mut mgr = Self {
            bindings: HashMap::new(),
            wheel: EmoteWheel::default(),
        };

        // Standard default Emotecraft keybinds:
        // 'B' -> Open/Toggle Emote Wheel
        mgr.register(Keybinding {
            id: "open_wheel".to_string(),
            name: "Open Emote Wheel".to_string(),
            default_key: KeyCode::KeyB,
            current_key: KeyCode::KeyB,
            action: KeybindAction::OpenWheel,
        });

        // 'X' -> Stop active emote
        mgr.register(Keybinding {
            id: "stop_emote".to_string(),
            name: "Stop Emote".to_string(),
            default_key: KeyCode::KeyX,
            current_key: KeyCode::KeyX,
            action: KeybindAction::StopEmote,
        });

        // 'G' -> Quick Dab
        mgr.register(Keybinding {
            id: "quick_dab".to_string(),
            name: "Quick Dab".to_string(),
            default_key: KeyCode::KeyG,
            current_key: KeyCode::KeyG,
            action: KeybindAction::PlayEmote("dab".to_string()),
        });

        // 'H' -> Quick Wave
        mgr.register(Keybinding {
            id: "quick_wave".to_string(),
            name: "Quick Wave".to_string(),
            default_key: KeyCode::KeyH,
            current_key: KeyCode::KeyH,
            action: KeybindAction::PlayEmote("wave".to_string()),
        });

        mgr
    }
}

impl KeybindManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, binding: Keybinding) {
        self.bindings.insert(binding.current_key, binding);
    }

    /// Handle key pressed in Horizon client.
    /// Returns the action to execute, if any.
    pub fn handle_key_press(&mut self, key: KeyCode) -> Option<KeybindAction> {
        if let Some(binding) = self.bindings.get(&key) {
            match &binding.action {
                KeybindAction::OpenWheel => {
                    self.wheel.toggle();
                    Some(KeybindAction::OpenWheel)
                }
                KeybindAction::StopEmote => {
                    if self.wheel.is_open {
                        self.wheel.close();
                    }
                    Some(KeybindAction::StopEmote)
                }
                KeybindAction::PlayEmote(id) => {
                    if self.wheel.is_open {
                        self.wheel.close();
                    }
                    Some(KeybindAction::PlayEmote(id.clone()))
                }
            }
        } else {
            None
        }
    }

    /// List all keybindings
    pub fn list(&self) -> Vec<&Keybinding> {
        self.bindings.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keybind_wheel_toggle() {
        let mut mgr = KeybindManager::new();
        assert!(!mgr.wheel.is_open);

        let action = mgr.handle_key_press(KeyCode::KeyB);
        assert_eq!(action, Some(KeybindAction::OpenWheel));
        assert!(mgr.wheel.is_open);

        let action2 = mgr.handle_key_press(KeyCode::KeyB);
        assert_eq!(action2, Some(KeybindAction::OpenWheel));
        assert!(!mgr.wheel.is_open);
    }

    #[test]
    fn test_quick_dab_keybind() {
        let mut mgr = KeybindManager::new();
        let action = mgr.handle_key_press(KeyCode::KeyG);
        assert_eq!(action, Some(KeybindAction::PlayEmote("dab".to_string())));
    }
}
