use letmedab::{
    builtin::create_dab_emote,
    client::EmotecraftClient,
    easing::Easing,
    format::EmoteFile,
    network::{EmotePacket, EMOTECRAFT_CHANNEL},
    registry::EmoteRegistry,
    server::{EmotecraftServer, ServerConfig},
    AnimationInstance,
};
use std::sync::Arc;
use uuid::Uuid;

#[test]
fn test_dab_keyframe_geometry() {
    let dab = create_dab_emote();
    assert_eq!(dab.id, "dab");
    assert_eq!(dab.keyframes.len(), 4);

    let mut instance = AnimationInstance::new(Arc::new(dab));

    // Tick 0: Neutral
    let p0 = instance.sample();
    assert_eq!(p0.head.pitch_degrees().round(), 0.0);
    assert_eq!(p0.left_arm.bend_degrees().round(), 0.0);

    // Tick 12: Peak Dab Pose
    instance.current_tick = 12.0;
    let p_peak = instance.sample();

    // In a dab, head tilts down (pitch > 0) and left (yaw > 0) towards the folded left arm
    assert!(p_peak.head.pitch_degrees() > 20.0);
    assert!(p_peak.head.yaw_degrees() > 30.0);

    // Left arm is bent sharply across face
    assert!(p_peak.left_arm.bend_degrees() >= 75.0);
    assert!(p_peak.left_arm.pitch_degrees() < -50.0);

    // Right arm is outstretched back and up
    assert!(p_peak.right_arm.pitch_degrees() < -50.0);
    assert!(p_peak.right_arm.roll_degrees() < -30.0);
}

#[test]
fn test_json_roundtrip() {
    let dab = create_dab_emote();
    let file = dab.to_emote_file();
    let json = file.to_json().expect("Serialization failed");

    let parsed = EmoteFile::from_str(&json).expect("Deserialization failed");
    let compiled = parsed.compile().expect("Compilation failed");

    assert_eq!(compiled.id, dab.id);
    assert_eq!(compiled.keyframes.len(), dab.keyframes.len());
    assert_eq!(compiled.stop_tick, dab.stop_tick);
}

#[test]
fn test_easing_accuracy() {
    let easings = [
        Easing::Linear,
        Easing::EaseInSine,
        Easing::EaseOutSine,
        Easing::EaseInOutSine,
        Easing::EaseInCubic,
        Easing::EaseOutCubic,
        Easing::EaseInOutCubic,
        Easing::EaseOutBounce,
        Easing::Step,
    ];

    for easing in easings {
        let v0 = easing.ease(0.0);
        let v1 = easing.ease(1.0);
        assert!((v0 - 0.0).abs() < 1e-4, "{:?} at 0.0 was {}", easing, v0);
        assert!((v1 - 1.0).abs() < 1e-4, "{:?} at 1.0 was {}", easing, v1);
    }
}

#[test]
fn test_network_roundtrip() {
    let player = Uuid::new_v4();
    let packet = EmotePacket::Play {
        player_uuid: player,
        emote_id: "dab".to_string(),
        is_loop: false,
        start_tick: 0,
        raw_emote_json: None,
    };

    let encoded = packet.encode().expect("Packet encode failed");
    let decoded = EmotePacket::decode(&encoded).expect("Packet decode failed");

    assert_eq!(packet, decoded);
}

#[test]
fn test_server_client_handshake_and_broadcast() {
    let registry = Arc::new(EmoteRegistry::new());
    let mut server = EmotecraftServer::new(registry.clone(), ServerConfig::default());

    let p1 = Uuid::new_v4();
    let p2 = Uuid::new_v4();
    let players = vec![p1, p2];

    let mut c1 = EmotecraftClient::new(p1, registry.clone());
    let mut c2 = EmotecraftClient::new(p2, registry.clone());

    // C1 Handshake
    let req = c1.create_handshake_packet().unwrap();
    let resp = server.handle_packet(p1, EMOTECRAFT_CHANNEL, &req, &players).unwrap();
    for msg in resp {
        c1.handle_server_packet(&msg.channel, &msg.payload).unwrap();
    }
    assert!(c1.is_connected_to_emotecraft_server);

    // C1 plays Dab
    let play_bytes = c1.play_emote("dab").unwrap();
    let broadcast = server.handle_packet(p1, EMOTECRAFT_CHANNEL, &play_bytes, &players).unwrap();

    assert_eq!(broadcast.len(), 1);
    assert_eq!(broadcast[0].recipient, p2);

    // C2 receives packet
    c2.handle_server_packet(&broadcast[0].channel, &broadcast[0].payload).unwrap();

    // Check C2 is animating P1
    let pose = c2.sample_player_pose(&p1);
    assert!(pose.is_some());
}
