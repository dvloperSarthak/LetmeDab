use crate::easing::Easing;
use crate::format::{Emote, ProcessedKeyframe};
use crate::model::{PartTransform, Pose};
use std::sync::Arc;
use uuid::Uuid;

/// Returns all standard built-in emotes (32 total).
pub fn get_builtin_emotes() -> Vec<Arc<Emote>> {
    vec![
        // Core classics
        Arc::new(create_dab_emote()),
        Arc::new(create_wave_emote()),
        Arc::new(create_bow_emote()),
        Arc::new(create_cheer_emote()),
        Arc::new(create_salute_emote()),
        Arc::new(create_facepalm_emote()),
        Arc::new(create_tpose_emote()),
        // Social & Expressive
        Arc::new(create_shrug_emote()),
        Arc::new(create_clap_emote()),
        Arc::new(create_cry_emote()),
        Arc::new(create_laugh_emote()),
        Arc::new(create_point_emote()),
        Arc::new(create_think_emote()),
        Arc::new(create_peace_emote()),
        Arc::new(create_heart_emote()),
        Arc::new(create_beckon_emote()),
        Arc::new(create_slow_clap_emote()),
        Arc::new(create_chuckle_emote()),
        Arc::new(create_surrender_emote()),
        // Posture & Roleplay
        Arc::new(create_sit_emote()),
        Arc::new(create_sleep_emote()),
        Arc::new(create_zombie_emote()),
        Arc::new(create_ninja_emote()),
        Arc::new(create_flex_emote()),
        // Dances & Action
        Arc::new(create_floss_emote()),
        Arc::new(create_gangnam_emote()),
        Arc::new(create_spin_emote()),
        Arc::new(create_guitar_emote()),
        Arc::new(create_disco_emote()),
        Arc::new(create_robot_emote()),
        Arc::new(create_pushup_emote()),
        Arc::new(create_jumping_jack_emote()),
    ]
}

// ============================================================================
// 1. DAB
// ============================================================================
pub fn create_dab_emote() -> Emote {
    let kf0 = ProcessedKeyframe {
        tick: 0,
        easing: Easing::EaseOutCubic,
        pose: Pose::default(),
    };

    let mut pose_dab = Pose::default();
    pose_dab.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 30.0, 45.0, 10.0, 0.0);
    pose_dab.torso = PartTransform::from_degrees(0.0, 0.0, 0.0, 15.0, 15.0, 10.0, 0.0);
    pose_dab.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -75.0, 70.0, 15.0, 80.0);
    pose_dab.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -70.0, -60.0, -45.0, 0.0);

    let kf_peak = ProcessedKeyframe {
        tick: 12,
        easing: Easing::EaseOutBack,
        pose: pose_dab,
    };
    let kf_hold = ProcessedKeyframe {
        tick: 25,
        easing: Easing::EaseInOutSine,
        pose: pose_dab,
    };
    let kf_end = ProcessedKeyframe {
        tick: 35,
        easing: Easing::EaseOutSine,
        pose: Pose::default(),
    };

    Emote {
        id: "dab".to_string(),
        name: "Dab".to_string(),
        author: "LetmeDAB Team".to_string(),
        description: "The classic iconic Dab gesture".to_string(),
        uuid: Uuid::from_u128(0x0001_dabd_ab00_0000_0000_0000_0000_0001),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 35,
        stop_tick: 38,
        keyframes: vec![kf0, kf_peak, kf_hold, kf_end],
    }
}

// ============================================================================
// 2. WAVE
// ============================================================================
pub fn create_wave_emote() -> Emote {
    let kf0 = ProcessedKeyframe {
        tick: 0,
        easing: Easing::EaseOutCubic,
        pose: Pose::default(),
    };

    let mut pose_up = Pose::default();
    pose_up.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -140.0, 0.0, 25.0, 30.0);

    let kf_up = ProcessedKeyframe {
        tick: 8,
        easing: Easing::EaseInOutSine,
        pose: pose_up,
    };

    let mut pose_w1 = pose_up;
    pose_w1.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -140.0, 0.0, 45.0, 40.0);
    let kf_w1 = ProcessedKeyframe {
        tick: 14,
        easing: Easing::EaseInOutSine,
        pose: pose_w1,
    };

    let mut pose_w2 = pose_up;
    pose_w2.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -140.0, 0.0, 10.0, 20.0);
    let kf_w2 = ProcessedKeyframe {
        tick: 20,
        easing: Easing::EaseInOutSine,
        pose: pose_w2,
    };

    let kf_w3 = ProcessedKeyframe {
        tick: 26,
        easing: Easing::EaseInOutSine,
        pose: pose_w1,
    };

    let kf_end = ProcessedKeyframe {
        tick: 36,
        easing: Easing::EaseOutSine,
        pose: Pose::default(),
    };

    Emote {
        id: "wave".to_string(),
        name: "Wave".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Wave hello or goodbye to friends".to_string(),
        uuid: Uuid::from_u128(0x0002_0000_0000_0000_0000_0000_0000_0002),
        is_loop: false,
        return_to_tick: 8,
        begin_tick: 0,
        end_tick: 36,
        stop_tick: 40,
        keyframes: vec![kf0, kf_up, kf_w1, kf_w2, kf_w3, kf_end],
    }
}

// ============================================================================
// 3. BOW
// ============================================================================
pub fn create_bow_emote() -> Emote {
    let kf0 = ProcessedKeyframe {
        tick: 0,
        easing: Easing::EaseInOutSine,
        pose: Pose::default(),
    };

    let mut bow_pose = Pose::default();
    bow_pose.torso = PartTransform::from_degrees(0.0, 2.0, 0.0, 40.0, 0.0, 0.0, 0.0);
    bow_pose.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 0.0);
    bow_pose.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 10.0, 0.0, 10.0, 0.0);
    bow_pose.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 10.0, 0.0, -10.0, 0.0);

    let kf_bow = ProcessedKeyframe {
        tick: 15,
        easing: Easing::EaseInOutSine,
        pose: bow_pose,
    };
    let kf_hold = ProcessedKeyframe {
        tick: 30,
        easing: Easing::EaseInOutSine,
        pose: bow_pose,
    };
    let kf_end = ProcessedKeyframe {
        tick: 45,
        easing: Easing::EaseOutCubic,
        pose: Pose::default(),
    };

    Emote {
        id: "bow".to_string(),
        name: "Bow".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "A respectful bow".to_string(),
        uuid: Uuid::from_u128(0x0003_0000_0000_0000_0000_0000_0000_0003),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 45,
        stop_tick: 48,
        keyframes: vec![kf0, kf_bow, kf_hold, kf_end],
    }
}

// ============================================================================
// 4. CHEER
// ============================================================================
pub fn create_cheer_emote() -> Emote {
    let kf0 = ProcessedKeyframe {
        tick: 0,
        easing: Easing::EaseOutBack,
        pose: Pose::default(),
    };

    let mut cheer_pose = Pose::default();
    cheer_pose.torso = PartTransform::from_degrees(0.0, -2.0, 0.0, -10.0, 0.0, 0.0, 0.0);
    cheer_pose.head = PartTransform::from_degrees(0.0, 0.0, 0.0, -25.0, 0.0, 0.0, 0.0);
    cheer_pose.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -170.0, 0.0, -20.0, 25.0);
    cheer_pose.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -170.0, 0.0, 20.0, 25.0);

    let kf_cheer = ProcessedKeyframe {
        tick: 12,
        easing: Easing::EaseInOutSine,
        pose: cheer_pose,
    };
    let kf_hold = ProcessedKeyframe {
        tick: 28,
        easing: Easing::EaseOutCubic,
        pose: cheer_pose,
    };
    let kf_end = ProcessedKeyframe {
        tick: 40,
        easing: Easing::EaseOutSine,
        pose: Pose::default(),
    };

    Emote {
        id: "cheer".to_string(),
        name: "Cheer".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Celebrate victory with a two-arm cheer!".to_string(),
        uuid: Uuid::from_u128(0x0004_0000_0000_0000_0000_0000_0000_0004),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 40,
        stop_tick: 44,
        keyframes: vec![kf0, kf_cheer, kf_hold, kf_end],
    }
}

// ============================================================================
// 5. SALUTE
// ============================================================================
pub fn create_salute_emote() -> Emote {
    let kf0 = ProcessedKeyframe {
        tick: 0,
        easing: Easing::EaseOutBack,
        pose: Pose::default(),
    };

    let mut salute_pose = Pose::default();
    salute_pose.head = PartTransform::from_degrees(0.0, 0.0, 0.0, -5.0, 0.0, 0.0, 0.0);
    salute_pose.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -115.0, 45.0, 20.0, 90.0);

    let kf_salute = ProcessedKeyframe {
        tick: 10,
        easing: Easing::Linear,
        pose: salute_pose,
    };
    let kf_hold = ProcessedKeyframe {
        tick: 26,
        easing: Easing::EaseOutSine,
        pose: salute_pose,
    };
    let kf_end = ProcessedKeyframe {
        tick: 34,
        easing: Easing::EaseOutSine,
        pose: Pose::default(),
    };

    Emote {
        id: "salute".to_string(),
        name: "Salute".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "A disciplined crisp military salute".to_string(),
        uuid: Uuid::from_u128(0x0005_0000_0000_0000_0000_0000_0000_0005),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 34,
        stop_tick: 36,
        keyframes: vec![kf0, kf_salute, kf_hold, kf_end],
    }
}

// ============================================================================
// 6. FACEPALM
// ============================================================================
pub fn create_facepalm_emote() -> Emote {
    let kf0 = ProcessedKeyframe {
        tick: 0,
        easing: Easing::EaseOutCubic,
        pose: Pose::default(),
    };

    let mut facepalm_pose = Pose::default();
    facepalm_pose.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 20.0, 10.0, 5.0, 0.0);
    facepalm_pose.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -100.0, -20.0, 15.0, 85.0);

    let kf_facepalm = ProcessedKeyframe {
        tick: 14,
        easing: Easing::EaseInOutSine,
        pose: facepalm_pose,
    };
    let kf_hold = ProcessedKeyframe {
        tick: 32,
        easing: Easing::EaseOutCubic,
        pose: facepalm_pose,
    };
    let kf_end = ProcessedKeyframe {
        tick: 44,
        easing: Easing::EaseOutSine,
        pose: Pose::default(),
    };

    Emote {
        id: "facepalm".to_string(),
        name: "Facepalm".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Express utter disbelief or exasperation".to_string(),
        uuid: Uuid::from_u128(0x0006_0000_0000_0000_0000_0000_0000_0006),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 44,
        stop_tick: 48,
        keyframes: vec![kf0, kf_facepalm, kf_hold, kf_end],
    }
}

// ============================================================================
// 7. T-POSE
// ============================================================================
pub fn create_tpose_emote() -> Emote {
    let kf0 = ProcessedKeyframe {
        tick: 0,
        easing: Easing::EaseOutBack,
        pose: Pose::default(),
    };

    let mut tpose = Pose::default();
    tpose.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, 90.0, 0.0);
    tpose.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, -90.0, 0.0);

    let kf_t = ProcessedKeyframe {
        tick: 6,
        easing: Easing::Linear,
        pose: tpose,
    };
    let kf_hold = ProcessedKeyframe {
        tick: 40,
        easing: Easing::EaseOutSine,
        pose: tpose,
    };
    let kf_end = ProcessedKeyframe {
        tick: 46,
        easing: Easing::EaseOutSine,
        pose: Pose::default(),
    };

    Emote {
        id: "tpose".to_string(),
        name: "T-Pose".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Assert pure dominance with the default T-Pose".to_string(),
        uuid: Uuid::from_u128(0x0007_0000_0000_0000_0000_0000_0000_0007),
        is_loop: true,
        return_to_tick: 6,
        begin_tick: 0,
        end_tick: 40,
        stop_tick: 46,
        keyframes: vec![kf0, kf_t, kf_hold, kf_end],
    }
}

// ============================================================================
// 8. SHRUG
// ============================================================================
pub fn create_shrug_emote() -> Emote {
    let kf0 = ProcessedKeyframe {
        tick: 0,
        easing: Easing::EaseOutBack,
        pose: Pose::default(),
    };

    let mut shrug = Pose::default();
    shrug.head = PartTransform::from_degrees(0.0, 0.0, 0.0, -10.0, 0.0, 15.0, 0.0);
    shrug.torso = PartTransform::from_degrees(0.0, -1.0, 0.0, -5.0, 0.0, 0.0, 0.0);
    shrug.left_arm = PartTransform::from_degrees(0.0, -1.0, 0.0, -35.0, -20.0, -45.0, 60.0);
    shrug.right_arm = PartTransform::from_degrees(0.0, -1.0, 0.0, -35.0, 20.0, 45.0, 60.0);

    let kf_shrug = ProcessedKeyframe {
        tick: 12,
        easing: Easing::EaseOutBack,
        pose: shrug,
    };
    let kf_hold = ProcessedKeyframe {
        tick: 28,
        easing: Easing::EaseInOutSine,
        pose: shrug,
    };
    let kf_end = ProcessedKeyframe {
        tick: 38,
        easing: Easing::EaseOutSine,
        pose: Pose::default(),
    };

    Emote {
        id: "shrug".to_string(),
        name: "Shrug".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Who knows? ¯\\_(ツ)_/¯".to_string(),
        uuid: Uuid::from_u128(0x0008_0000_0000_0000_0000_0000_0000_0008),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 38,
        stop_tick: 40,
        keyframes: vec![kf0, kf_shrug, kf_hold, kf_end],
    }
}

// ============================================================================
// 9. CLAP / APPLAUSE (Looping)
// ============================================================================
pub fn create_clap_emote() -> Emote {
    let kf0 = ProcessedKeyframe {
        tick: 0,
        easing: Easing::EaseOutCubic,
        pose: Pose::default(),
    };

    let mut clap_open = Pose::default();
    clap_open.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -60.0, 30.0, -15.0, 60.0);
    clap_open.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -60.0, -30.0, 15.0, 60.0);

    let mut clap_hit = Pose::default();
    clap_hit.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -65.0, 10.0, 0.0, 75.0);
    clap_hit.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -65.0, -10.0, 0.0, 75.0);

    let kf1 = ProcessedKeyframe { tick: 6, easing: Easing::EaseInOutSine, pose: clap_open };
    let kf2 = ProcessedKeyframe { tick: 10, easing: Easing::EaseInQuad, pose: clap_hit };
    let kf3 = ProcessedKeyframe { tick: 14, easing: Easing::EaseOutQuad, pose: clap_open };
    let kf4 = ProcessedKeyframe { tick: 18, easing: Easing::EaseInQuad, pose: clap_hit };
    let kf5 = ProcessedKeyframe { tick: 22, easing: Easing::EaseOutQuad, pose: clap_open };

    Emote {
        id: "clap".to_string(),
        name: "Clap".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Enthusiastic applause and clapping".to_string(),
        uuid: Uuid::from_u128(0x0009_0000_0000_0000_0000_0000_0000_0009),
        is_loop: true,
        return_to_tick: 6,
        begin_tick: 0,
        end_tick: 22,
        stop_tick: 26,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4, kf5],
    }
}

// ============================================================================
// 10. CRY
// ============================================================================
pub fn create_cry_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutCubic, pose: Pose::default() };

    let mut cry1 = Pose::default();
    cry1.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 35.0, 0.0, 0.0, 0.0);
    cry1.torso = PartTransform::from_degrees(0.0, 1.0, 0.0, 15.0, 0.0, 0.0, 0.0);
    cry1.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, 35.0, 10.0, 95.0);
    cry1.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, -35.0, -10.0, 95.0);

    let mut cry_sob = cry1;
    cry_sob.torso = PartTransform::from_degrees(0.0, 0.5, 0.0, 20.0, 0.0, 0.0, 0.0);

    let kf1 = ProcessedKeyframe { tick: 10, easing: Easing::EaseInOutSine, pose: cry1 };
    let kf2 = ProcessedKeyframe { tick: 16, easing: Easing::EaseInOutSine, pose: cry_sob };
    let kf3 = ProcessedKeyframe { tick: 22, easing: Easing::EaseInOutSine, pose: cry1 };
    let kf4 = ProcessedKeyframe { tick: 28, easing: Easing::EaseInOutSine, pose: cry_sob };
    let kf5 = ProcessedKeyframe { tick: 38, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "cry".to_string(),
        name: "Cry".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Sobbing and covering face in tears".to_string(),
        uuid: Uuid::from_u128(0x000a_0000_0000_0000_0000_0000_0000_000a),
        is_loop: false,
        return_to_tick: 10,
        begin_tick: 0,
        end_tick: 38,
        stop_tick: 42,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4, kf5],
    }
}

// ============================================================================
// 11. LAUGH
// ============================================================================
pub fn create_laugh_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutBack, pose: Pose::default() };

    let mut laugh1 = Pose::default();
    laugh1.head = PartTransform::from_degrees(0.0, 0.0, 0.0, -30.0, 0.0, 0.0, 0.0);
    laugh1.torso = PartTransform::from_degrees(0.0, 0.0, 0.0, -15.0, 0.0, 0.0, 0.0);
    laugh1.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 20.0, -25.0, -30.0, 60.0);
    laugh1.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 20.0, 25.0, 30.0, 60.0);

    let mut laugh2 = laugh1;
    laugh2.head = PartTransform::from_degrees(0.0, 0.0, 0.0, -15.0, 0.0, 0.0, 0.0);
    laugh2.torso = PartTransform::from_degrees(0.0, 0.0, 0.0, -5.0, 0.0, 0.0, 0.0);

    let kf1 = ProcessedKeyframe { tick: 8, easing: Easing::EaseInOutSine, pose: laugh1 };
    let kf2 = ProcessedKeyframe { tick: 14, easing: Easing::EaseInOutSine, pose: laugh2 };
    let kf3 = ProcessedKeyframe { tick: 20, easing: Easing::EaseInOutSine, pose: laugh1 };
    let kf4 = ProcessedKeyframe { tick: 26, easing: Easing::EaseInOutSine, pose: laugh2 };
    let kf5 = ProcessedKeyframe { tick: 36, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "laugh".to_string(),
        name: "Laugh".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Hearty belly laughter".to_string(),
        uuid: Uuid::from_u128(0x000b_0000_0000_0000_0000_0000_0000_000b),
        is_loop: false,
        return_to_tick: 8,
        begin_tick: 0,
        end_tick: 36,
        stop_tick: 40,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4, kf5],
    }
}

// ============================================================================
// 12. POINT
// ============================================================================
pub fn create_point_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutBack, pose: Pose::default() };

    let mut point = Pose::default();
    point.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, -10.0, 0.0, 0.0);
    point.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, -5.0, 0.0, 0.0);

    let kf_pt = ProcessedKeyframe { tick: 8, easing: Easing::EaseOutBack, pose: point };
    let kf_hold = ProcessedKeyframe { tick: 26, easing: Easing::EaseInOutSine, pose: point };
    let kf_end = ProcessedKeyframe { tick: 34, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "point".to_string(),
        name: "Point".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Point directly at someone or something".to_string(),
        uuid: Uuid::from_u128(0x000c_0000_0000_0000_0000_0000_0000_000c),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 34,
        stop_tick: 36,
        keyframes: vec![kf0, kf_pt, kf_hold, kf_end],
    }
}

// ============================================================================
// 13. THINK
// ============================================================================
pub fn create_think_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutCubic, pose: Pose::default() };

    let mut think = Pose::default();
    think.head = PartTransform::from_degrees(0.0, 0.0, 0.0, -15.0, 20.0, 10.0, 0.0);
    think.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -110.0, -25.0, 15.0, 100.0);
    think.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -40.0, 40.0, -20.0, 75.0);

    let kf_th = ProcessedKeyframe { tick: 12, easing: Easing::EaseOutCubic, pose: think };
    let kf_hold = ProcessedKeyframe { tick: 32, easing: Easing::EaseInOutSine, pose: think };
    let kf_end = ProcessedKeyframe { tick: 42, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "think".to_string(),
        name: "Think".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Deep thinking with hand on chin 🤔".to_string(),
        uuid: Uuid::from_u128(0x000d_0000_0000_0000_0000_0000_0000_000d),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 42,
        stop_tick: 45,
        keyframes: vec![kf0, kf_th, kf_hold, kf_end],
    }
}

// ============================================================================
// 14. PEACE
// ============================================================================
pub fn create_peace_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutBack, pose: Pose::default() };

    let mut peace = Pose::default();
    peace.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, -12.0, 0.0);
    peace.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -110.0, -15.0, 25.0, 90.0);

    let kf_p = ProcessedKeyframe { tick: 10, easing: Easing::EaseOutBack, pose: peace };
    let kf_hold = ProcessedKeyframe { tick: 28, easing: Easing::EaseInOutSine, pose: peace };
    let kf_end = ProcessedKeyframe { tick: 36, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "peace".to_string(),
        name: "Peace".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Flash a peace sign ✌️".to_string(),
        uuid: Uuid::from_u128(0x000e_0000_0000_0000_0000_0000_0000_000e),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 36,
        stop_tick: 38,
        keyframes: vec![kf0, kf_p, kf_hold, kf_end],
    }
}

// ============================================================================
// 15. HEART
// ============================================================================
pub fn create_heart_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutCubic, pose: Pose::default() };

    let mut heart = Pose::default();
    heart.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 15.0, 0.0, 0.0, 0.0);
    heart.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -65.0, 45.0, 25.0, 80.0);
    heart.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -65.0, -45.0, -25.0, 80.0);

    let kf_h = ProcessedKeyframe { tick: 12, easing: Easing::EaseOutBack, pose: heart };
    let kf_hold = ProcessedKeyframe { tick: 32, easing: Easing::EaseInOutSine, pose: heart };
    let kf_end = ProcessedKeyframe { tick: 42, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "heart".to_string(),
        name: "Heart".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Form a heart shape with both hands 💖".to_string(),
        uuid: Uuid::from_u128(0x000f_0000_0000_0000_0000_0000_0000_000f),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 42,
        stop_tick: 46,
        keyframes: vec![kf0, kf_h, kf_hold, kf_end],
    }
}

// ============================================================================
// 16. BECKON / COME HERE
// ============================================================================
pub fn create_beckon_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutCubic, pose: Pose::default() };

    let mut b1 = Pose::default();
    b1.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -60.0, 0.0, 10.0, 45.0);

    let mut b2 = Pose::default();
    b2.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -75.0, 0.0, 10.0, 80.0);

    let kf1 = ProcessedKeyframe { tick: 8, easing: Easing::EaseInOutSine, pose: b1 };
    let kf2 = ProcessedKeyframe { tick: 14, easing: Easing::EaseInOutSine, pose: b2 };
    let kf3 = ProcessedKeyframe { tick: 20, easing: Easing::EaseInOutSine, pose: b1 };
    let kf4 = ProcessedKeyframe { tick: 26, easing: Easing::EaseInOutSine, pose: b2 };
    let kf5 = ProcessedKeyframe { tick: 36, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "beckon".to_string(),
        name: "Beckon".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Wave towards self to say 'come here!'".to_string(),
        uuid: Uuid::from_u128(0x0010_0000_0000_0000_0000_0000_0000_0010),
        is_loop: false,
        return_to_tick: 8,
        begin_tick: 0,
        end_tick: 36,
        stop_tick: 40,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4, kf5],
    }
}

// ============================================================================
// 17. SLOW CLAP
// ============================================================================
pub fn create_slow_clap_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutCubic, pose: Pose::default() };

    let mut open = Pose::default();
    open.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -60.0, 35.0, -20.0, 60.0);
    open.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -60.0, -35.0, 20.0, 60.0);

    let mut hit = Pose::default();
    hit.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -65.0, 10.0, 0.0, 75.0);
    hit.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -65.0, -10.0, 0.0, 75.0);

    let kf1 = ProcessedKeyframe { tick: 8, easing: Easing::EaseInOutSine, pose: open };
    let kf2 = ProcessedKeyframe { tick: 16, easing: Easing::EaseInCubic, pose: hit };
    let kf3 = ProcessedKeyframe { tick: 24, easing: Easing::EaseOutSine, pose: open };
    let kf4 = ProcessedKeyframe { tick: 34, easing: Easing::EaseInCubic, pose: hit };
    let kf5 = ProcessedKeyframe { tick: 44, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "slow_clap".to_string(),
        name: "Slow Clap".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Sarcastic theatrical slow clapping 👏 ... 👏".to_string(),
        uuid: Uuid::from_u128(0x0011_0000_0000_0000_0000_0000_0000_0011),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 44,
        stop_tick: 48,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4, kf5],
    }
}

// ============================================================================
// 18. CHUCKLE
// ============================================================================
pub fn create_chuckle_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutCubic, pose: Pose::default() };

    let mut ch1 = Pose::default();
    ch1.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 10.0, 10.0, 5.0, 0.0);
    ch1.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -95.0, -25.0, 10.0, 90.0);

    let mut ch2 = ch1;
    ch2.torso = PartTransform::from_degrees(0.0, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0);

    let kf1 = ProcessedKeyframe { tick: 10, easing: Easing::EaseInOutSine, pose: ch1 };
    let kf2 = ProcessedKeyframe { tick: 15, easing: Easing::EaseInOutSine, pose: ch2 };
    let kf3 = ProcessedKeyframe { tick: 20, easing: Easing::EaseInOutSine, pose: ch1 };
    let kf4 = ProcessedKeyframe { tick: 30, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "chuckle".to_string(),
        name: "Chuckle".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Polite hand-over-mouth chuckle 🤭".to_string(),
        uuid: Uuid::from_u128(0x0012_0000_0000_0000_0000_0000_0000_0012),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 30,
        stop_tick: 34,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4],
    }
}

// ============================================================================
// 19. SURRENDER
// ============================================================================
pub fn create_surrender_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutBack, pose: Pose::default() };

    let mut surr = Pose::default();
    surr.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
    surr.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -165.0, 0.0, -20.0, 15.0);
    surr.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -165.0, 0.0, 20.0, 15.0);

    let kf1 = ProcessedKeyframe { tick: 10, easing: Easing::EaseOutBack, pose: surr };
    let kf_hold = ProcessedKeyframe { tick: 30, easing: Easing::EaseInOutSine, pose: surr };
    let kf_end = ProcessedKeyframe { tick: 40, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "surrender".to_string(),
        name: "Surrender".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Hands up in surrender 🙌".to_string(),
        uuid: Uuid::from_u128(0x0013_0000_0000_0000_0000_0000_0000_0013),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 40,
        stop_tick: 42,
        keyframes: vec![kf0, kf1, kf_hold, kf_end],
    }
}

// ============================================================================
// 20. SIT (Looping)
// ============================================================================
pub fn create_sit_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseInOutSine, pose: Pose::default() };

    let mut sit = Pose::default();
    sit.torso = PartTransform::from_degrees(0.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    sit.left_leg = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, 0.0, 0.0, 0.0);
    sit.right_leg = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, 0.0, 0.0, 0.0);
    sit.left_arm = PartTransform::from_degrees(0.0, 9.0, 0.0, -45.0, 0.0, 0.0, 30.0);
    sit.right_arm = PartTransform::from_degrees(0.0, 9.0, 0.0, -45.0, 0.0, 0.0, 30.0);

    let kf1 = ProcessedKeyframe { tick: 12, easing: Easing::EaseInOutSine, pose: sit };
    let kf2 = ProcessedKeyframe { tick: 40, easing: Easing::Linear, pose: sit };

    Emote {
        id: "sit".to_string(),
        name: "Sit".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Sit down comfortably on the ground".to_string(),
        uuid: Uuid::from_u128(0x0014_0000_0000_0000_0000_0000_0000_0014),
        is_loop: true,
        return_to_tick: 12,
        begin_tick: 0,
        end_tick: 40,
        stop_tick: 44,
        keyframes: vec![kf0, kf1, kf2],
    }
}

// ============================================================================
// 21. SLEEP / LAY (Looping)
// ============================================================================
pub fn create_sleep_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseInOutSine, pose: Pose::default() };

    let mut sleep = Pose::default();
    sleep.torso = PartTransform::from_degrees(0.0, 11.0, 0.0, 90.0, 0.0, 0.0, 0.0);
    sleep.head = PartTransform::from_degrees(0.0, 11.0, 0.0, 90.0, 0.0, 30.0, 0.0);
    sleep.left_arm = PartTransform::from_degrees(0.0, 11.0, 0.0, 90.0, 0.0, 0.0, 0.0);
    sleep.right_arm = PartTransform::from_degrees(0.0, 11.0, 0.0, 90.0, 0.0, 0.0, 0.0);
    sleep.left_leg = PartTransform::from_degrees(0.0, 11.0, 0.0, 90.0, 0.0, 0.0, 0.0);
    sleep.right_leg = PartTransform::from_degrees(0.0, 11.0, 0.0, 90.0, 0.0, 0.0, 0.0);

    let kf1 = ProcessedKeyframe { tick: 15, easing: Easing::EaseInOutSine, pose: sleep };
    let kf2 = ProcessedKeyframe { tick: 45, easing: Easing::Linear, pose: sleep };

    Emote {
        id: "sleep".to_string(),
        name: "Sleep".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Lie down and sleep 💤".to_string(),
        uuid: Uuid::from_u128(0x0015_0000_0000_0000_0000_0000_0000_0015),
        is_loop: true,
        return_to_tick: 15,
        begin_tick: 0,
        end_tick: 45,
        stop_tick: 50,
        keyframes: vec![kf0, kf1, kf2],
    }
}

// ============================================================================
// 22. ZOMBIE
// ============================================================================
pub fn create_zombie_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseInOutSine, pose: Pose::default() };

    let mut zomb = Pose::default();
    zomb.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 5.0, 0.0, 15.0, 0.0);
    zomb.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, -5.0, 0.0, 0.0);
    zomb.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, 5.0, 0.0, 0.0);

    let kf1 = ProcessedKeyframe { tick: 10, easing: Easing::EaseInOutSine, pose: zomb };
    let kf2 = ProcessedKeyframe { tick: 35, easing: Easing::EaseInOutSine, pose: zomb };
    let kf3 = ProcessedKeyframe { tick: 42, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "zombie".to_string(),
        name: "Zombie".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Classic arms-outstretched zombie walk 🧟".to_string(),
        uuid: Uuid::from_u128(0x0016_0000_0000_0000_0000_0000_0000_0016),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 42,
        stop_tick: 45,
        keyframes: vec![kf0, kf1, kf2, kf3],
    }
}

// ============================================================================
// 23. NINJA / MARTIAL ARTS
// ============================================================================
pub fn create_ninja_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutBack, pose: Pose::default() };

    let mut ninja = Pose::default();
    ninja.torso = PartTransform::from_degrees(0.0, 2.0, 0.0, 10.0, -25.0, 0.0, 0.0);
    ninja.head = PartTransform::from_degrees(0.0, 0.0, 0.0, -5.0, 25.0, 0.0, 0.0);
    ninja.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -55.0, 40.0, 15.0, 60.0);
    ninja.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 15.0, -30.0, -10.0, 90.0);
    ninja.left_leg = PartTransform::from_degrees(0.0, 0.0, 0.0, -15.0, 0.0, -10.0, 25.0);
    ninja.right_leg = PartTransform::from_degrees(0.0, 0.0, 0.0, 15.0, 0.0, 10.0, 25.0);

    let kf1 = ProcessedKeyframe { tick: 10, easing: Easing::EaseOutBack, pose: ninja };
    let kf2 = ProcessedKeyframe { tick: 32, easing: Easing::EaseInOutSine, pose: ninja };
    let kf3 = ProcessedKeyframe { tick: 40, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "ninja".to_string(),
        name: "Ninja".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Dynamic martial arts ninja guard stance 🥷".to_string(),
        uuid: Uuid::from_u128(0x0017_0000_0000_0000_0000_0000_0000_0017),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 40,
        stop_tick: 44,
        keyframes: vec![kf0, kf1, kf2, kf3],
    }
}

// ============================================================================
// 24. FLEX
// ============================================================================
pub fn create_flex_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutBack, pose: Pose::default() };

    let mut flex = Pose::default();
    flex.torso = PartTransform::from_degrees(0.0, -1.0, 0.0, -5.0, 0.0, 0.0, 0.0);
    flex.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, -15.0, 0.0, 0.0);
    flex.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, 0.0, -85.0, 90.0);
    flex.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, 0.0, 85.0, 90.0);

    let kf1 = ProcessedKeyframe { tick: 12, easing: Easing::EaseOutBack, pose: flex };
    let kf2 = ProcessedKeyframe { tick: 30, easing: Easing::EaseInOutSine, pose: flex };
    let kf3 = ProcessedKeyframe { tick: 40, easing: Easing::EaseOutSine, pose: Pose::default() };

    Emote {
        id: "flex".to_string(),
        name: "Flex".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Show off those gains with a double bicep flex 💪".to_string(),
        uuid: Uuid::from_u128(0x0018_0000_0000_0000_0000_0000_0000_0018),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 40,
        stop_tick: 44,
        keyframes: vec![kf0, kf1, kf2, kf3],
    }
}

// ============================================================================
// 25. FLOSS (Looping Dance)
// ============================================================================
pub fn create_floss_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseInOutSine, pose: Pose::default() };

    // Left swing: hips right, arms left
    let mut f_left = Pose::default();
    f_left.torso = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 15.0, 10.0, 0.0);
    f_left.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 20.0, -35.0, -25.0, 20.0);
    f_left.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 25.0, -30.0, -25.0, 20.0);

    // Right swing: hips left, arms right
    let mut f_right = Pose::default();
    f_right.torso = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, -15.0, -10.0, 0.0);
    f_right.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 25.0, 30.0, 25.0, 20.0);
    f_right.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 20.0, 35.0, 25.0, 20.0);

    let kf1 = ProcessedKeyframe { tick: 6, easing: Easing::EaseInOutSine, pose: f_left };
    let kf2 = ProcessedKeyframe { tick: 12, easing: Easing::EaseInOutSine, pose: f_right };
    let kf3 = ProcessedKeyframe { tick: 18, easing: Easing::EaseInOutSine, pose: f_left };
    let kf4 = ProcessedKeyframe { tick: 24, easing: Easing::EaseInOutSine, pose: f_right };

    Emote {
        id: "floss".to_string(),
        name: "Floss".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "The viral floss dance moves!".to_string(),
        uuid: Uuid::from_u128(0x0019_0000_0000_0000_0000_0000_0000_0019),
        is_loop: true,
        return_to_tick: 6,
        begin_tick: 0,
        end_tick: 24,
        stop_tick: 28,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4],
    }
}

// ============================================================================
// 26. GANGNAM (Looping Dance)
// ============================================================================
pub fn create_gangnam_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseInOutSine, pose: Pose::default() };

    let mut bounce1 = Pose::default();
    bounce1.torso = PartTransform::from_degrees(0.0, -1.5, 0.0, 5.0, 0.0, 0.0, 0.0);
    bounce1.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -65.0, 30.0, 10.0, 80.0);
    bounce1.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -65.0, -30.0, -10.0, 80.0);
    bounce1.left_leg = PartTransform::from_degrees(0.0, 0.0, 0.0, -10.0, 0.0, 0.0, 20.0);
    bounce1.right_leg = PartTransform::from_degrees(0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 20.0);

    let mut bounce2 = bounce1;
    bounce2.torso = PartTransform::from_degrees(0.0, 1.5, 0.0, 5.0, 0.0, 0.0, 0.0);
    bounce2.left_leg = PartTransform::from_degrees(0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 20.0);
    bounce2.right_leg = PartTransform::from_degrees(0.0, 0.0, 0.0, -10.0, 0.0, 0.0, 20.0);

    let kf1 = ProcessedKeyframe { tick: 5, easing: Easing::EaseInOutSine, pose: bounce1 };
    let kf2 = ProcessedKeyframe { tick: 10, easing: Easing::EaseInOutSine, pose: bounce2 };
    let kf3 = ProcessedKeyframe { tick: 15, easing: Easing::EaseInOutSine, pose: bounce1 };
    let kf4 = ProcessedKeyframe { tick: 20, easing: Easing::EaseInOutSine, pose: bounce2 };

    Emote {
        id: "gangnam".to_string(),
        name: "Gangnam".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Oppa Gangnam Style horse dance 🐎".to_string(),
        uuid: Uuid::from_u128(0x001a_0000_0000_0000_0000_0000_0000_001a),
        is_loop: true,
        return_to_tick: 5,
        begin_tick: 0,
        end_tick: 20,
        stop_tick: 24,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4],
    }
}

// ============================================================================
// 27. SPIN
// ============================================================================
pub fn create_spin_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::Linear, pose: Pose::default() };

    let mut s1 = Pose::default();
    s1.torso = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 0.0);
    s1.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 0.0);
    s1.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, -45.0, 0.0);
    s1.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, 45.0, 0.0);

    let mut s2 = s1;
    s2.torso.yaw = 180.0f32.to_radians();
    s2.head.yaw = 180.0f32.to_radians();

    let mut s3 = s1;
    s3.torso.yaw = 270.0f32.to_radians();
    s3.head.yaw = 270.0f32.to_radians();

    let mut s4 = Pose::default();
    s4.torso.yaw = 360.0f32.to_radians();
    s4.head.yaw = 360.0f32.to_radians();

    let kf1 = ProcessedKeyframe { tick: 5, easing: Easing::Linear, pose: s1 };
    let kf2 = ProcessedKeyframe { tick: 10, easing: Easing::Linear, pose: s2 };
    let kf3 = ProcessedKeyframe { tick: 15, easing: Easing::Linear, pose: s3 };
    let kf4 = ProcessedKeyframe { tick: 20, easing: Easing::Linear, pose: s4 };

    Emote {
        id: "spin".to_string(),
        name: "Spin".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Full 360 degree spin pirouette 🌪️".to_string(),
        uuid: Uuid::from_u128(0x001b_0000_0000_0000_0000_0000_0000_001b),
        is_loop: false,
        return_to_tick: 0,
        begin_tick: 0,
        end_tick: 20,
        stop_tick: 22,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4],
    }
}

// ============================================================================
// 28. GUITAR (Air Guitar, Looping)
// ============================================================================
pub fn create_guitar_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseOutBack, pose: Pose::default() };

    let mut g1 = Pose::default();
    g1.torso = PartTransform::from_degrees(0.0, 1.0, 0.0, 10.0, 20.0, 5.0, 0.0);
    g1.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 15.0, -10.0, 0.0, 0.0);
    g1.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -60.0, 45.0, 20.0, 60.0);
    g1.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -45.0, -20.0, -15.0, 80.0);

    let mut g2 = g1;
    g2.head = PartTransform::from_degrees(0.0, 0.0, 0.0, -15.0, -10.0, 0.0, 0.0);
    g2.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -25.0, -15.0, -15.0, 70.0);

    let kf1 = ProcessedKeyframe { tick: 6, easing: Easing::EaseInOutSine, pose: g1 };
    let kf2 = ProcessedKeyframe { tick: 12, easing: Easing::EaseInOutSine, pose: g2 };
    let kf3 = ProcessedKeyframe { tick: 18, easing: Easing::EaseInOutSine, pose: g1 };
    let kf4 = ProcessedKeyframe { tick: 24, easing: Easing::EaseInOutSine, pose: g2 };

    Emote {
        id: "guitar".to_string(),
        name: "Guitar".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Rocking out on an invisible air guitar 🎸".to_string(),
        uuid: Uuid::from_u128(0x001c_0000_0000_0000_0000_0000_0000_001c),
        is_loop: true,
        return_to_tick: 6,
        begin_tick: 0,
        end_tick: 24,
        stop_tick: 28,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4],
    }
}

// ============================================================================
// 29. DISCO (Looping)
// ============================================================================
pub fn create_disco_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseInOutSine, pose: Pose::default() };

    let mut point_up = Pose::default();
    point_up.torso = PartTransform::from_degrees(0.0, 0.0, 0.0, -5.0, 15.0, 0.0, 0.0);
    point_up.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -150.0, 0.0, 35.0, 0.0);
    point_up.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 20.0, -20.0, -20.0, 45.0);

    let mut point_down = Pose::default();
    point_down.torso = PartTransform::from_degrees(0.0, 0.0, 0.0, 10.0, -15.0, 0.0, 0.0);
    point_down.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 30.0, 40.0, -25.0, 20.0);
    point_down.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 20.0, -20.0, -20.0, 45.0);

    let kf1 = ProcessedKeyframe { tick: 8, easing: Easing::EaseInOutSine, pose: point_up };
    let kf2 = ProcessedKeyframe { tick: 16, easing: Easing::EaseInOutSine, pose: point_down };
    let kf3 = ProcessedKeyframe { tick: 24, easing: Easing::EaseInOutSine, pose: point_up };
    let kf4 = ProcessedKeyframe { tick: 32, easing: Easing::EaseInOutSine, pose: point_down };

    Emote {
        id: "disco".to_string(),
        name: "Disco".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Classic Saturday night fever disco dance 🕺".to_string(),
        uuid: Uuid::from_u128(0x001d_0000_0000_0000_0000_0000_0000_001d),
        is_loop: true,
        return_to_tick: 8,
        begin_tick: 0,
        end_tick: 32,
        stop_tick: 36,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4],
    }
}

// ============================================================================
// 30. ROBOT (Step Easing)
// ============================================================================
pub fn create_robot_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::Step, pose: Pose::default() };

    let mut r1 = Pose::default();
    r1.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 45.0, 0.0, 0.0);
    r1.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, 0.0, 0.0, 90.0);

    let mut r2 = Pose::default();
    r2.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, -45.0, 0.0, 0.0);
    r2.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, 90.0, 90.0);
    r2.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, -90.0, 0.0, 0.0, 90.0);

    let mut r3 = Pose::default();
    r3.head = PartTransform::from_degrees(0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 0.0);
    r3.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, -90.0, 90.0);

    let kf1 = ProcessedKeyframe { tick: 8, easing: Easing::Step, pose: r1 };
    let kf2 = ProcessedKeyframe { tick: 16, easing: Easing::Step, pose: r2 };
    let kf3 = ProcessedKeyframe { tick: 24, easing: Easing::Step, pose: r3 };
    let kf4 = ProcessedKeyframe { tick: 32, easing: Easing::Step, pose: Pose::default() };

    Emote {
        id: "robot".to_string(),
        name: "Robot".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Mechanical snappy robotic movements 🤖".to_string(),
        uuid: Uuid::from_u128(0x001e_0000_0000_0000_0000_0000_0000_001e),
        is_loop: true,
        return_to_tick: 8,
        begin_tick: 0,
        end_tick: 32,
        stop_tick: 36,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4],
    }
}

// ============================================================================
// 31. PUSHUP (Looping Fitness)
// ============================================================================
pub fn create_pushup_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseInOutSine, pose: Pose::default() };

    // Up position
    let mut up = Pose::default();
    up.torso = PartTransform::from_degrees(0.0, 10.0, 0.0, 85.0, 0.0, 0.0, 0.0);
    up.head = PartTransform::from_degrees(0.0, 10.0, 0.0, 45.0, 0.0, 0.0, 0.0);
    up.left_arm = PartTransform::from_degrees(0.0, 10.0, 0.0, 90.0, 0.0, 0.0, 0.0);
    up.right_arm = PartTransform::from_degrees(0.0, 10.0, 0.0, 90.0, 0.0, 0.0, 0.0);
    up.left_leg = PartTransform::from_degrees(0.0, 10.0, 0.0, 85.0, 0.0, 0.0, 0.0);
    up.right_leg = PartTransform::from_degrees(0.0, 10.0, 0.0, 85.0, 0.0, 0.0, 0.0);

    // Down position
    let mut down = up;
    down.torso.y = 12.0;
    down.left_arm = PartTransform::from_degrees(0.0, 12.0, 0.0, 90.0, 0.0, 0.0, 90.0);
    down.right_arm = PartTransform::from_degrees(0.0, 12.0, 0.0, 90.0, 0.0, 0.0, 90.0);

    let kf1 = ProcessedKeyframe { tick: 8, easing: Easing::EaseInOutSine, pose: up };
    let kf2 = ProcessedKeyframe { tick: 16, easing: Easing::EaseInOutSine, pose: down };
    let kf3 = ProcessedKeyframe { tick: 24, easing: Easing::EaseInOutSine, pose: up };

    Emote {
        id: "pushup".to_string(),
        name: "Pushup".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Get those reps in with floor pushups!".to_string(),
        uuid: Uuid::from_u128(0x001f_0000_0000_0000_0000_0000_0000_001f),
        is_loop: true,
        return_to_tick: 8,
        begin_tick: 0,
        end_tick: 24,
        stop_tick: 28,
        keyframes: vec![kf0, kf1, kf2, kf3],
    }
}

// ============================================================================
// 32. JUMPING JACK (Looping Fitness)
// ============================================================================
pub fn create_jumping_jack_emote() -> Emote {
    let kf0 = ProcessedKeyframe { tick: 0, easing: Easing::EaseInOutSine, pose: Pose::default() };

    let mut open = Pose::default();
    open.torso = PartTransform::from_degrees(0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    open.left_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, -145.0, 0.0);
    open.right_arm = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, 145.0, 0.0);
    open.left_leg = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, -25.0, 0.0);
    open.right_leg = PartTransform::from_degrees(0.0, 0.0, 0.0, 0.0, 0.0, 25.0, 0.0);

    let kf1 = ProcessedKeyframe { tick: 6, easing: Easing::EaseInOutSine, pose: open };
    let kf2 = ProcessedKeyframe { tick: 12, easing: Easing::EaseInOutSine, pose: Pose::default() };
    let kf3 = ProcessedKeyframe { tick: 18, easing: Easing::EaseInOutSine, pose: open };
    let kf4 = ProcessedKeyframe { tick: 24, easing: Easing::EaseInOutSine, pose: Pose::default() };

    Emote {
        id: "jumping_jack".to_string(),
        name: "Jumping Jack".to_string(),
        author: "Horizon Emotecraft".to_string(),
        description: "Energetic jumping jack workout".to_string(),
        uuid: Uuid::from_u128(0x0020_0000_0000_0000_0000_0000_0000_0020),
        is_loop: true,
        return_to_tick: 6,
        begin_tick: 0,
        end_tick: 24,
        stop_tick: 28,
        keyframes: vec![kf0, kf1, kf2, kf3, kf4],
    }
}
