use crate::format::Emote;
use crate::model::Pose;
use std::sync::Arc;

/// An active playing instance of an Emote on a specific player model.
#[derive(Debug, Clone)]
pub struct AnimationInstance {
    pub emote: Arc<Emote>,
    pub current_tick: f32,
    pub is_playing: bool,
    pub is_looping: bool,
    pub fade_out_duration: f32,
    pub fade_out_progress: f32,
    pub is_fading_out: bool,
    pub last_pose_before_fade: Option<Pose>,
}

impl AnimationInstance {
    /// Create a new playing animation instance from an emote.
    pub fn new(emote: Arc<Emote>) -> Self {
        let is_looping = emote.is_loop;
        let start_tick = emote.begin_tick as f32;
        Self {
            emote,
            current_tick: start_tick,
            is_playing: true,
            is_looping,
            fade_out_duration: 5.0, // Default 5 ticks fade-out for silky smooth transitions
            fade_out_progress: 0.0,
            is_fading_out: false,
            last_pose_before_fade: None,
        }
    }

    /// Advance animation playback by `delta_ticks`.
    /// In Minecraft, 1 tick = 0.05 seconds (20 ticks/sec).
    pub fn update(&mut self, delta_ticks: f32) {
        if !self.is_playing {
            return;
        }

        if self.is_fading_out {
            self.fade_out_progress += delta_ticks;
            if self.fade_out_progress >= self.fade_out_duration {
                self.is_playing = false;
            }
            return;
        }

        self.current_tick += delta_ticks;

        let end_tick = self.emote.end_tick as f32;
        let stop_tick = self.emote.stop_tick as f32;

        if self.is_looping {
            if self.current_tick >= end_tick {
                let loop_start = self.emote.return_to_tick as f32;
                let loop_length = (end_tick - loop_start).max(1.0);
                let overflow = self.current_tick - end_tick;
                self.current_tick = loop_start + (overflow % loop_length);
            }
        } else {
            if self.current_tick >= stop_tick {
                self.is_playing = false;
            }
        }
    }

    /// Stop the emote with a graceful fade-out.
    pub fn stop(&mut self, fade_ticks: f32) {
        if !self.is_playing || self.is_fading_out {
            return;
        }
        if fade_ticks <= 0.0 {
            self.is_playing = false;
            return;
        }
        self.last_pose_before_fade = Some(self.sample_raw());
        self.fade_out_duration = fade_ticks;
        self.fade_out_progress = 0.0;
        self.is_fading_out = true;
    }

    /// Sample raw pose from keyframes without fade-out blending.
    fn sample_raw(&self) -> Pose {
        let kfs = &self.emote.keyframes;
        if kfs.is_empty() {
            return Pose::default();
        }

        if kfs.len() == 1 {
            return kfs[0].pose;
        }

        let tick = self.current_tick;

        // Before first keyframe
        if tick <= kfs[0].tick as f32 {
            return kfs[0].pose;
        }

        // After last keyframe
        let last_idx = kfs.len() - 1;
        if tick >= kfs[last_idx].tick as f32 {
            return kfs[last_idx].pose;
        }

        // Find the bounding keyframe span [prev, next]
        let mut prev_idx = 0;
        for i in 0..last_idx {
            if (kfs[i].tick as f32) <= tick && tick <= (kfs[i + 1].tick as f32) {
                prev_idx = i;
                break;
            }
        }

        let prev = &kfs[prev_idx];
        let next = &kfs[prev_idx + 1];

        let span = (next.tick - prev.tick) as f32;
        if span <= 0.0 {
            return next.pose;
        }

        let progress = (tick - prev.tick as f32) / span;
        let eased_t = next.easing.ease(progress);

        prev.pose.lerp(&next.pose, eased_t)
    }

    /// Sample the final player model pose at the current playback moment.
    pub fn sample(&self) -> Pose {
        if !self.is_playing {
            return Pose::default();
        }

        if self.is_fading_out {
            let initial_pose = self.last_pose_before_fade.unwrap_or_else(|| Pose::default());
            let t = (self.fade_out_progress / self.fade_out_duration).clamp(0.0, 1.0);
            return initial_pose.lerp(&Pose::default(), t);
        }

        self.sample_raw()
    }

    /// Current progress percentage (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        let stop_tick = self.emote.stop_tick as f32;
        if stop_tick <= 0.0 {
            0.0
        } else {
            (self.current_tick / stop_tick).clamp(0.0, 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::easing::Easing;
    use crate::format::ProcessedKeyframe;
    use crate::model::PartTransform;
    use uuid::Uuid;

    #[test]
    fn test_animation_interpolation() {
        let kf1 = ProcessedKeyframe {
            tick: 0,
            easing: Easing::Linear,
            pose: Pose::default(),
        };
        let mut kf2 = ProcessedKeyframe {
            tick: 20,
            easing: Easing::Linear,
            pose: Pose::default(),
        };
        kf2.pose.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 90.0, 0.0, 0.0, 0.0);

        let emote = Arc::new(Emote {
            id: "test".to_string(),
            name: "Test".to_string(),
            author: "Tester".to_string(),
            description: "Testing".to_string(),
            uuid: Uuid::new_v4(),
            is_loop: false,
            return_to_tick: 0,
            begin_tick: 0,
            end_tick: 20,
            stop_tick: 25,
            keyframes: vec![kf1, kf2],
        });

        let mut instance = AnimationInstance::new(emote);
        assert_eq!(instance.sample().head.pitch_degrees().round(), 0.0);

        instance.update(10.0); // halfway
        assert_eq!(instance.sample().head.pitch_degrees().round(), 45.0);

        instance.update(10.0); // at tick 20
        assert_eq!(instance.sample().head.pitch_degrees().round(), 90.0);

        instance.update(5.0); // at tick 25 -> finishes
        assert!(!instance.is_playing);
        assert_eq!(instance.sample().head.pitch_degrees().round(), 0.0);
    }
}
