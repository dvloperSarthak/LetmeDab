use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// Easing functions supported by Emotecraft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Easing {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
    Step,
}

impl Default for Easing {
    fn default() -> Self {
        Easing::Linear
    }
}

impl Easing {
    /// Parse easing from a case-insensitive string.
    pub fn from_str_loose(s: &str) -> Self {
        let clean = s.to_lowercase().replace('_', "").replace('-', "");
        match clean.as_str() {
            "linear" => Easing::Linear,
            "easeinquad" | "inquad" => Easing::EaseInQuad,
            "easeoutquad" | "outquad" => Easing::EaseOutQuad,
            "easeinoutquad" | "inoutquad" => Easing::EaseInOutQuad,
            "easeincubic" | "incubic" => Easing::EaseInCubic,
            "easeoutcubic" | "outcubic" => Easing::EaseOutCubic,
            "easeinoutcubic" | "inoutcubic" => Easing::EaseInOutCubic,
            "easeinquart" | "inquart" => Easing::EaseInQuart,
            "easeoutquart" | "outquart" => Easing::EaseOutQuart,
            "easeinoutquart" | "inoutquart" => Easing::EaseInOutQuart,
            "easeinquint" | "inquint" => Easing::EaseInQuint,
            "easeoutquint" | "outquint" => Easing::EaseOutQuint,
            "easeinoutquint" | "inoutquint" => Easing::EaseInOutQuint,
            "easeinsine" | "insine" => Easing::EaseInSine,
            "easeoutsine" | "outsine" => Easing::EaseOutSine,
            "easeinoutsine" | "inoutsine" => Easing::EaseInOutSine,
            "easeinexpo" | "inexpo" => Easing::EaseInExpo,
            "easeoutexpo" | "outexpo" => Easing::EaseOutExpo,
            "easeinoutexpo" | "inoutexpo" => Easing::EaseInOutExpo,
            "easeincirc" | "incirc" => Easing::EaseInCirc,
            "easeoutcirc" | "outcirc" => Easing::EaseOutCirc,
            "easeinoutcirc" | "inoutcirc" => Easing::EaseInOutCirc,
            "easeinback" | "inback" => Easing::EaseInBack,
            "easeoutback" | "outback" => Easing::EaseOutBack,
            "easeinoutback" | "inoutback" => Easing::EaseInOutBack,
            "easeinelastic" | "inelastic" => Easing::EaseInElastic,
            "easeoutelastic" | "outelastic" => Easing::EaseOutElastic,
            "easeinoutelastic" | "inoutelastic" => Easing::EaseInOutElastic,
            "easeinbounce" | "inbounce" => Easing::EaseInBounce,
            "easeoutbounce" | "outbounce" => Easing::EaseOutBounce,
            "easeinoutbounce" | "inoutbounce" => Easing::EaseInOutBounce,
            "step" => Easing::Step,
            _ => Easing::Linear,
        }
    }

    /// Evaluates the easing function for progress `t` in range [0.0, 1.0].
    pub fn ease(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => t * (2.0 - t),
            Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::EaseInCubic => t * t * t,
            Easing::EaseOutCubic => {
                let p = t - 1.0;
                p * p * p + 1.0
            }
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let f = 2.0 * t - 2.0;
                    0.5 * f * f * f + 1.0
                }
            }
            Easing::EaseInQuart => t * t * t * t,
            Easing::EaseOutQuart => {
                let p = t - 1.0;
                1.0 - p * p * p * p
            }
            Easing::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    let p = t - 1.0;
                    1.0 - 8.0 * p * p * p * p
                }
            }
            Easing::EaseInQuint => t * t * t * t * t,
            Easing::EaseOutQuint => {
                let p = t - 1.0;
                p * p * p * p * p + 1.0
            }
            Easing::EaseInOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    let p = 2.0 * t - 2.0;
                    0.5 * p * p * p * p * p + 1.0
                }
            }
            Easing::EaseInSine => 1.0 - (t * PI / 2.0).cos(),
            Easing::EaseOutSine => (t * PI / 2.0).sin(),
            Easing::EaseInOutSine => -0.5 * ((PI * t).cos() - 1.0),
            Easing::EaseInExpo => {
                if t == 0.0 {
                    0.0
                } else {
                    (2.0f32).powf(10.0 * (t - 1.0))
                }
            }
            Easing::EaseOutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - (2.0f32).powf(-10.0 * t)
                }
            }
            Easing::EaseInOutExpo => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    0.5 * (2.0f32).powf(20.0 * t - 10.0)
                } else {
                    1.0 - 0.5 * (2.0f32).powf(-20.0 * t + 10.0)
                }
            }
            Easing::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            Easing::EaseOutCirc => (1.0 - (t - 1.0).powi(2)).sqrt(),
            Easing::EaseInOutCirc => {
                if t < 0.5 {
                    0.5 * (1.0 - (1.0 - 4.0 * t * t).sqrt())
                } else {
                    0.5 * ((1.0 - (2.0 * t - 2.0).powi(2)).sqrt() + 1.0)
                }
            }
            Easing::EaseInBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            Easing::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                let p = t - 1.0;
                1.0 + c3 * p * p * p + c1 * p * p
            }
            Easing::EaseInOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }
            Easing::EaseInElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    let c4 = (2.0 * PI) / 3.0;
                    -(2.0f32).powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin()
                }
            }
            Easing::EaseOutElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    let c4 = (2.0 * PI) / 3.0;
                    (2.0f32).powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
            Easing::EaseInOutElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    let c5 = (2.0 * PI) / 4.5;
                    if t < 0.5 {
                        -0.5 * (2.0f32).powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()
                    } else {
                        0.5 * (2.0f32).powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin() + 1.0
                    }
                }
            }
            Easing::EaseOutBounce => Self::ease_out_bounce(t),
            Easing::EaseInBounce => 1.0 - Self::ease_out_bounce(1.0 - t),
            Easing::EaseInOutBounce => {
                if t < 0.5 {
                    (1.0 - Self::ease_out_bounce(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + Self::ease_out_bounce(2.0 * t - 1.0)) / 2.0
                }
            }
            Easing::Step => {
                if t >= 1.0 {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }

    fn ease_out_bounce(t: f32) -> f32 {
        let n1 = 7.5625;
        let d1 = 2.75;
        if t < 1.0 / d1 {
            n1 * t * t
        } else if t < 2.0 / d1 {
            let t = t - 1.5 / d1;
            n1 * t * t + 0.75
        } else if t < 2.5 / d1 {
            let t = t - 2.25 / d1;
            n1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / d1;
            n1 * t * t + 0.984375
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_bounds() {
        let easings = [
            Easing::Linear,
            Easing::EaseInQuad,
            Easing::EaseOutQuad,
            Easing::EaseInOutSine,
            Easing::EaseInOutCubic,
            Easing::EaseOutBounce,
        ];
        for easing in &easings {
            assert!((easing.ease(0.0) - 0.0).abs() < 1e-4);
            assert!((easing.ease(1.0) - 1.0).abs() < 1e-4);
            let mid = easing.ease(0.5);
            assert!(mid >= -0.5 && mid <= 1.5);
        }
    }

    #[test]
    fn test_loose_parsing() {
        assert_eq!(Easing::from_str_loose("ease_in_out_sine"), Easing::EaseInOutSine);
        assert_eq!(Easing::from_str_loose("easeInOutCubic"), Easing::EaseInOutCubic);
        assert_eq!(Easing::from_str_loose("LINEAR"), Easing::Linear);
        assert_eq!(Easing::from_str_loose("OutBounce"), Easing::EaseOutBounce);
    }
}
