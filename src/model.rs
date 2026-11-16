use serde::{Deserialize, Serialize};

/// Identified body parts in the Minecraft humanoid player model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BodyPart {
    Head,
    Torso,
    RightArm,
    LeftArm,
    RightLeg,
    LeftLeg,
}

impl BodyPart {
    pub const ALL: [BodyPart; 6] = [
        BodyPart::Head,
        BodyPart::Torso,
        BodyPart::RightArm,
        BodyPart::LeftArm,
        BodyPart::RightLeg,
        BodyPart::LeftLeg,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            BodyPart::Head => "head",
            BodyPart::Torso => "torso",
            BodyPart::RightArm => "rightArm",
            BodyPart::LeftArm => "leftArm",
            BodyPart::RightLeg => "rightLeg",
            BodyPart::LeftLeg => "leftLeg",
        }
    }
}

/// Transform for an individual model part.
/// Rotations are stored internally in radians.
/// Translations are stored in Minecraft pixel/model units.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PartTransform {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,

    #[serde(default)]
    pub pitch: f32,
    #[serde(default)]
    pub yaw: f32,
    #[serde(default)]
    pub roll: f32,

    #[serde(default)]
    pub bend: f32,
}

impl Default for PartTransform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
            bend: 0.0,
        }
    }
}

impl PartTransform {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        pitch: 0.0,
        yaw: 0.0,
        roll: 0.0,
        bend: 0.0,
    };

    /// Create from degrees instead of radians
    pub fn from_degrees(x: f32, y: f32, z: f32, pitch_deg: f32, yaw_deg: f32, roll_deg: f32, bend_deg: f32) -> Self {
        Self {
            x,
            y,
            z,
            pitch: pitch_deg.to_radians(),
            yaw: yaw_deg.to_radians(),
            roll: roll_deg.to_radians(),
            bend: bend_deg.to_radians(),
        }
    }

    /// Pitch in degrees
    pub fn pitch_degrees(&self) -> f32 {
        self.pitch.to_degrees()
    }

    /// Yaw in degrees
    pub fn yaw_degrees(&self) -> f32 {
        self.yaw.to_degrees()
    }

    /// Roll in degrees
    pub fn roll_degrees(&self) -> f32 {
        self.roll.to_degrees()
    }

    /// Bend in degrees
    pub fn bend_degrees(&self) -> f32 {
        self.bend.to_degrees()
    }

    /// Linear interpolation between two transforms
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
            pitch: self.pitch + (other.pitch - self.pitch) * t,
            yaw: self.yaw + (other.yaw - self.yaw) * t,
            roll: self.roll + (other.roll - self.roll) * t,
            bend: self.bend + (other.bend - self.bend) * t,
        }
    }

    /// Add two transforms together (useful for layering/blending on vanilla walk poses)
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            pitch: self.pitch + other.pitch,
            yaw: self.yaw + other.yaw,
            roll: self.roll + other.roll,
            bend: self.bend + other.bend,
        }
    }
}

/// Complete humanoid pose for all body parts.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    pub head: PartTransform,
    pub torso: PartTransform,
    pub right_arm: PartTransform,
    pub left_arm: PartTransform,
    pub right_leg: PartTransform,
    pub left_leg: PartTransform,
}

impl Default for Pose {
    fn default() -> Self {
        Self {
            head: PartTransform::ZERO,
            torso: PartTransform::ZERO,
            right_arm: PartTransform::ZERO,
            left_arm: PartTransform::ZERO,
            right_leg: PartTransform::ZERO,
            left_leg: PartTransform::ZERO,
        }
    }
}

impl Pose {
    pub fn get_part(&self, part: BodyPart) -> &PartTransform {
        match part {
            BodyPart::Head => &self.head,
            BodyPart::Torso => &self.torso,
            BodyPart::RightArm => &self.right_arm,
            BodyPart::LeftArm => &self.left_arm,
            BodyPart::RightLeg => &self.right_leg,
            BodyPart::LeftLeg => &self.left_leg,
        }
    }

    pub fn get_part_mut(&mut self, part: BodyPart) -> &mut PartTransform {
        match part {
            BodyPart::Head => &mut self.head,
            BodyPart::Torso => &mut self.torso,
            BodyPart::RightArm => &mut self.right_arm,
            BodyPart::LeftArm => &mut self.left_arm,
            BodyPart::RightLeg => &mut self.right_leg,
            BodyPart::LeftLeg => &mut self.left_leg,
        }
    }

    /// Linearly interpolate between two poses with progress `t` in [0.0, 1.0].
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            head: self.head.lerp(&other.head, t),
            torso: self.torso.lerp(&other.torso, t),
            right_arm: self.right_arm.lerp(&other.right_arm, t),
            left_arm: self.left_arm.lerp(&other.left_arm, t),
            right_leg: self.right_leg.lerp(&other.right_leg, t),
            left_leg: self.left_leg.lerp(&other.left_leg, t),
        }
    }

    /// Blend this pose with an additive pose weighted by weight
    pub fn blend_additive(&self, additive: &Self, weight: f32) -> Self {
        let weighted = Self {
            head: PartTransform::ZERO.lerp(&additive.head, weight),
            torso: PartTransform::ZERO.lerp(&additive.torso, weight),
            right_arm: PartTransform::ZERO.lerp(&additive.right_arm, weight),
            left_arm: PartTransform::ZERO.lerp(&additive.left_arm, weight),
            right_leg: PartTransform::ZERO.lerp(&additive.right_leg, weight),
            left_leg: PartTransform::ZERO.lerp(&additive.left_leg, weight),
        };
        Self {
            head: self.head.add(&weighted.head),
            torso: self.torso.add(&weighted.torso),
            right_arm: self.right_arm.add(&weighted.right_arm),
            left_arm: self.left_arm.add(&weighted.left_arm),
            right_leg: self.right_leg.add(&weighted.right_leg),
            left_leg: self.left_leg.add(&weighted.left_leg),
        }
    }
}
