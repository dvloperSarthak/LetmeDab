use crate::easing::Easing;
use crate::model::{PartTransform, Pose};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum EmoteFormatError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid animation: {0}")]
    Invalid(String),
}

/// Raw representation of a body part's transforms in the JSON file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawPartTransform {
    #[serde(default)]
    pub x: Option<f32>,
    #[serde(default)]
    pub y: Option<f32>,
    #[serde(default)]
    pub z: Option<f32>,
    #[serde(default)]
    pub pitch: Option<f32>,
    #[serde(default)]
    pub yaw: Option<f32>,
    #[serde(default)]
    pub roll: Option<f32>,
    #[serde(default)]
    pub bend: Option<f32>,
}

impl RawPartTransform {
    pub fn to_part_transform(&self, degrees: bool) -> PartTransform {
        let conv = if degrees {
            |val: Option<f32>| val.unwrap_or(0.0).to_radians()
        } else {
            |val: Option<f32>| val.unwrap_or(0.0)
        };

        PartTransform {
            x: self.x.unwrap_or(0.0),
            y: self.y.unwrap_or(0.0),
            z: self.z.unwrap_or(0.0),
            pitch: conv(self.pitch),
            yaw: conv(self.yaw),
            roll: conv(self.roll),
            bend: conv(self.bend),
        }
    }

    pub fn from_part_transform(pt: &PartTransform, degrees: bool) -> Self {
        let conv = if degrees {
            |val: f32| if val.abs() > 1e-5 { Some(val.to_degrees()) } else { None }
        } else {
            |val: f32| if val.abs() > 1e-5 { Some(val) } else { None }
        };

        let conv_pos = |val: f32| if val.abs() > 1e-5 { Some(val) } else { None };

        Self {
            x: conv_pos(pt.x),
            y: conv_pos(pt.y),
            z: conv_pos(pt.z),
            pitch: conv(pt.pitch),
            yaw: conv(pt.yaw),
            roll: conv(pt.roll),
            bend: conv(pt.bend),
        }
    }
}

/// A keyframe entry inside the JSON `moves` list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawMove {
    pub tick: u32,
    #[serde(default)]
    pub easing: Option<String>,
    #[serde(default)]
    pub turn: Option<f32>,
    #[serde(default)]
    pub head: Option<RawPartTransform>,
    #[serde(default)]
    pub torso: Option<RawPartTransform>,
    #[serde(default)]
    pub right_arm: Option<RawPartTransform>,
    #[serde(default)]
    pub left_arm: Option<RawPartTransform>,
    #[serde(default)]
    pub right_leg: Option<RawPartTransform>,
    #[serde(default)]
    pub left_leg: Option<RawPartTransform>,
}

/// Emote timing and motion configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawEmoteData {
    #[serde(default)]
    pub is_loop: Option<bool>,
    #[serde(default)]
    pub return_to_tick: Option<u32>,
    #[serde(default)]
    pub begin_tick: Option<u32>,
    pub end_tick: u32,
    pub stop_tick: u32,
    #[serde(default = "default_true")]
    pub degrees: Option<bool>,
    pub moves: Vec<RawMove>,
}

fn default_true() -> Option<bool> {
    Some(true)
}

/// Top-level Emotecraft file structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmoteFile {
    pub name: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub uuid: Option<Uuid>,
    pub emote: RawEmoteData,
}

/// Normalized Keyframe ready for high performance playback.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcessedKeyframe {
    pub tick: u32,
    pub easing: Easing,
    pub pose: Pose,
}

/// Complete compiled Emote ready for playback.
#[derive(Debug, Clone, PartialEq)]
pub struct Emote {
    pub id: String,
    pub name: String,
    pub author: String,
    pub description: String,
    pub uuid: Uuid,
    pub is_loop: bool,
    pub return_to_tick: u32,
    pub begin_tick: u32,
    pub end_tick: u32,
    pub stop_tick: u32,
    pub keyframes: Vec<ProcessedKeyframe>,
}

impl EmoteFile {
    /// Load an EmoteFile from a JSON or .emotecraft file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, EmoteFormatError> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Self::from_str(&content)
    }

    /// Load an EmoteFile from a JSON string.
    pub fn from_str(json: &str) -> Result<Self, EmoteFormatError> {
        let emote_file: EmoteFile = serde_json::from_str(json)?;
        Ok(emote_file)
    }

    /// Compile into an active `Emote` instance.
    pub fn compile(&self) -> Result<Emote, EmoteFormatError> {
        let degrees = self.emote.degrees.unwrap_or(true);
        let mut raw_moves = self.emote.moves.clone();
        raw_moves.sort_by_key(|m| m.tick);

        if raw_moves.is_empty() {
            return Err(EmoteFormatError::Invalid("Emote has no keyframes".to_string()));
        }

        let mut keyframes = Vec::new();
        let mut last_pose = Pose::default();

        for m in raw_moves {
            let easing = m
                .easing
                .as_ref()
                .map(|e| Easing::from_str_loose(e))
                .unwrap_or(Easing::Linear);

            // Update running pose state with specified transforms
            if let Some(ref h) = m.head {
                last_pose.head = h.to_part_transform(degrees);
            }
            if let Some(ref t) = m.torso {
                last_pose.torso = t.to_part_transform(degrees);
            }
            if let Some(ref ra) = m.right_arm {
                last_pose.right_arm = ra.to_part_transform(degrees);
            }
            if let Some(ref la) = m.left_arm {
                last_pose.left_arm = la.to_part_transform(degrees);
            }
            if let Some(ref rl) = m.right_leg {
                last_pose.right_leg = rl.to_part_transform(degrees);
            }
            if let Some(ref ll) = m.left_leg {
                last_pose.left_leg = ll.to_part_transform(degrees);
            }

            keyframes.push(ProcessedKeyframe {
                tick: m.tick,
                easing,
                pose: last_pose,
            });
        }

        // Generate id from name
        let id = self
            .name
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "_");

        Ok(Emote {
            id,
            name: self.name.clone(),
            author: self.author.clone().unwrap_or_else(|| "Unknown".to_string()),
            description: self.description.clone().unwrap_or_default(),
            uuid: self.uuid.unwrap_or_else(Uuid::new_v4),
            is_loop: self.emote.is_loop.unwrap_or(false),
            return_to_tick: self.emote.return_to_tick.unwrap_or(0),
            begin_tick: self.emote.begin_tick.unwrap_or(0),
            end_tick: self.emote.end_tick,
            stop_tick: self.emote.stop_tick,
            keyframes,
        })
    }

    /// Save to a formatted JSON string.
    pub fn to_json(&self) -> Result<String, EmoteFormatError> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }

    /// Save to file.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), EmoteFormatError> {
        let json = self.to_json()?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

impl Emote {
    /// Decompile an Emote back into an EmoteFile for export/JSON saving.
    pub fn to_emote_file(&self) -> EmoteFile {
        let moves = self
            .keyframes
            .iter()
            .map(|kf| RawMove {
                tick: kf.tick,
                easing: Some(format!("{:?}", kf.easing)),
                turn: None,
                head: Some(RawPartTransform::from_part_transform(&kf.pose.head, true)),
                torso: Some(RawPartTransform::from_part_transform(&kf.pose.torso, true)),
                right_arm: Some(RawPartTransform::from_part_transform(&kf.pose.right_arm, true)),
                left_arm: Some(RawPartTransform::from_part_transform(&kf.pose.left_arm, true)),
                right_leg: Some(RawPartTransform::from_part_transform(&kf.pose.right_leg, true)),
                left_leg: Some(RawPartTransform::from_part_transform(&kf.pose.left_leg, true)),
            })
            .collect();

        EmoteFile {
            name: self.name.clone(),
            author: Some(self.author.clone()),
            description: Some(self.description.clone()),
            icon: None,
            uuid: Some(self.uuid),
            emote: RawEmoteData {
                is_loop: Some(self.is_loop),
                return_to_tick: Some(self.return_to_tick),
                begin_tick: Some(self.begin_tick),
                end_tick: self.end_tick,
                stop_tick: self.stop_tick,
                degrees: Some(true),
                moves,
            },
        }
    }
}
