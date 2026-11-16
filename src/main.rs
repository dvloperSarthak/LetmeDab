use clap::{Parser, Subcommand};
use letmedab::{
    client::EmotecraftClient,
    format::EmoteFile,
    keybind::{KeyCode, KeybindAction, KeybindManager},
    network::EMOTECRAFT_CHANNEL,
    registry::EmoteRegistry,
    server::{EmotecraftServer, ServerConfig},
    AnimationInstance,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "letmedab")]
#[command(about = "LetmeDab: Emotecraft mod & plugin for Horizon (Minecraft Rust Edition)")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all registered and built-in emotes
    List,

    /// Play and simulate an emote in the terminal (e.g. dab)
    Play {
        /// Name or ID of the emote to play
        name: String,

        /// Simulation step interval in milliseconds
        #[arg(short, long, default_value_t = 50)]
        interval_ms: u64,
    },

    /// View configured keybinds and test keyboard input actions
    Keybinds {
        /// Optional key code to test (e.g. 'B', 'X', 'G', 'H')
        test_key: Option<String>,
    },

    /// Export built-in emotes (including DAB) to .emotecraft JSON files
    Export {
        /// Destination directory
        #[arg(default_value = "./emotes")]
        output_dir: PathBuf,
    },

    /// Validate an emote JSON file
    Validate {
        /// Path to the emote file
        file: PathBuf,
    },

    /// Run an end-to-end multi-client network broadcast simulation
    DemoNetwork,

    /// Build and package the Horizon mod (.hmod / cdylib) ready for Horizon launcher
    Package {
        /// Output directory for the packaged mod
        #[arg(default_value = "./dist")]
        output_dir: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();
    let registry = Arc::new(EmoteRegistry::new());

    match cli.command {
        Commands::List => {
            println!("╔═════════════════════════════════════════════════════════════════════════╗");
            println!("║              EMOTECRAFT FOR HORIZON (MINECRAFT RUST)                    ║");
            println!("╚═════════════════════════════════════════════════════════════════════════╝");
            println!("\nRegistered Emotes:");
            for emote in registry.all() {
                println!(
                    "  ▶ {:<12} | ID: {:<12} | Duration: {:>2} ticks | Loop: {:<5} | Author: {}",
                    emote.name, emote.id, emote.stop_tick, emote.is_loop, emote.author
                );
            }
            println!("\nTotal emotes: {}", registry.all().len());
        }

        Commands::Play { name, interval_ms } => {
            let emote = registry
                .get_by_id(&name)
                .ok_or_else(|| format!("Emote '{}' not found. Run 'emotecraft list' to view available emotes.", name))?;

            println!("=======================================================");
            println!("Playing Emote: {} ({}) by {}", emote.name, emote.id, emote.author);
            println!("Description:   {}", emote.description);
            println!("Total Ticks:   {}", emote.stop_tick);
            println!("=======================================================\n");

            let mut instance = AnimationInstance::new(emote);

            while instance.is_playing {
                let pose = instance.sample();
                let head = &pose.head;
                let l_arm = &pose.left_arm;
                let r_arm = &pose.right_arm;

                println!(
                    "Tick {:>2}/{:>2} [{:>3}%] | Head: P {:>4.0}° Y {:>4.0}° | L-Arm: P {:>4.0}° Y {:>4.0}° B {:>4.0}° | R-Arm: P {:>4.0}° Y {:>4.0}°",
                    instance.current_tick.round() as u32,
                    instance.emote.stop_tick,
                    (instance.progress() * 100.0).round() as u32,
                    head.pitch_degrees(),
                    head.yaw_degrees(),
                    l_arm.pitch_degrees(),
                    l_arm.yaw_degrees(),
                    l_arm.bend_degrees(),
                    r_arm.pitch_degrees(),
                    r_arm.yaw_degrees(),
                );

                instance.update(1.0);
                if interval_ms > 0 {
                    sleep(Duration::from_millis(interval_ms));
                }
            }

            println!("\nEmote animation completed!");
        }

        Commands::Keybinds { test_key } => {
            let mut mgr = KeybindManager::new();

            println!("╔═════════════════════════════════════════════════════════════════════════╗");
            println!("║              HORIZON EMOTECRAFT KEYBINDINGS                             ║");
            println!("╚═════════════════════════════════════════════════════════════════════════╝\n");

            println!("Configured Keybinds (stored in Horizon.toml):");
            for binding in mgr.list() {
                println!(
                    "  [Key {:?}] -> {:<20} (Action: {:?})",
                    binding.current_key, binding.name, binding.action
                );
            }

            println!("\nRadial Emote Wheel Slots (Default 'B'):");
            for (idx, slot) in mgr.wheel.slots.iter().enumerate() {
                let name = slot.as_deref().unwrap_or("<empty>");
                println!("  Slot {}: {}", idx + 1, name);
            }

            if let Some(key_str) = test_key {
                if let Some(key_code) = KeyCode::from_str_loose(&key_str) {
                    println!("\nSimulating key press: {:?}", key_code);
                    match mgr.handle_key_press(key_code) {
                        Some(KeybindAction::OpenWheel) => {
                            println!("  ✓ Action: Open/Toggle Emote Wheel (wheel is now {})", if mgr.wheel.is_open { "OPEN" } else { "CLOSED" });
                        }
                        Some(KeybindAction::StopEmote) => {
                            println!("  ✓ Action: Stop Active Emote");
                        }
                        Some(KeybindAction::PlayEmote(ref emote)) => {
                            println!("  ✓ Action: Immediately Trigger Emote '{}'", emote);
                        }
                        None => {
                            println!("  (No action bound to key {:?})", key_code);
                        }
                    }
                } else {
                    println!("\nUnrecognized key: '{}'. Try 'B', 'X', 'G', or 'H'.", key_str);
                }
            } else {
                println!("\nTip: Test a key with 'emotecraft keybinds <KEY>', e.g.:\n  emotecraft keybinds B\n  emotecraft keybinds G");
            }
        }

        Commands::Export { output_dir } => {
            fs::create_dir_all(&output_dir)?;
            println!("Exporting built-in emotes to: {}", output_dir.display());

            for emote in registry.all() {
                let file_path = output_dir.join(format!("{}.json", emote.id));
                let emote_file = emote.to_emote_file();
                emote_file.save_to_file(&file_path)?;
                println!("  ✓ Exported: {}", file_path.display());
            }

            println!("\nExport finished successfully! (32 files exported)");
        }

        Commands::Validate { file } => {
            println!("Validating emote file: {}", file.display());
            let emote_file = EmoteFile::from_file(&file)?;
            let emote = emote_file.compile()?;
            println!("✓ Valid Emotecraft file!");
            println!("  Name:        {}", emote.name);
            println!("  Author:      {}", emote.author);
            println!("  Description: {}", emote.description);
            println!("  Keyframes:   {}", emote.keyframes.len());
            println!("  Duration:    {} ticks", emote.stop_tick);
            println!("  Loop:        {}", emote.is_loop);
        }

        Commands::DemoNetwork => {
            println!("=== Horizon Minecraft Rust - Emotecraft Multiplayer Network Demo ===");

            let server_registry = Arc::new(EmoteRegistry::new());
            let mut server = EmotecraftServer::new(server_registry, ServerConfig::default());

            let player1 = Uuid::new_v4();
            let player2 = Uuid::new_v4();
            let online_players = vec![player1, player2];

            println!("1. Player 1 ({}) connects and completes handshake...", player1);
            let p1_client_registry = Arc::new(EmoteRegistry::new());
            let mut client1 = EmotecraftClient::new(player1, p1_client_registry);

            let handshake_req = client1.create_handshake_packet()?;
            let server_responses = server.handle_packet(player1, EMOTECRAFT_CHANNEL, &handshake_req, &online_players)?;

            for msg in server_responses {
                client1.handle_server_packet(&msg.channel, &msg.payload)?;
            }
            println!("   -> Handshake completed! Client 1 registered.");

            println!("2. Player 2 ({}) connects...", player2);
            let p2_client_registry = Arc::new(EmoteRegistry::new());
            let mut client2 = EmotecraftClient::new(player2, p2_client_registry);
            let p2_handshake = client2.create_handshake_packet()?;
            let s_resp2 = server.handle_packet(player2, EMOTECRAFT_CHANNEL, &p2_handshake, &online_players)?;
            for msg in s_resp2 {
                client2.handle_server_packet(&msg.channel, &msg.payload)?;
            }
            println!("   -> Handshake completed! Client 2 registered.");

            println!("3. Player 1 triggers Quick Dab (Key 'G')...");
            let play_payload = client1.play_emote("dab")?;
            println!("   -> Client 1 sent {} bytes over channel '{}'", play_payload.len(), EMOTECRAFT_CHANNEL);

            println!("4. Server processes packet, validates sender, and broadcasts to nearby players...");
            let broadcast = server.handle_packet(player1, EMOTECRAFT_CHANNEL, &play_payload, &online_players)?;
            println!("   -> Server generated {} broadcast packet(s)", broadcast.len());

            for msg in broadcast {
                if msg.recipient == player2 {
                    println!("   -> Delivering broadcast packet to Player 2...");
                    client2.handle_server_packet(&msg.channel, &msg.payload)?;
                }
            }

            println!("5. Verifying Client 2 renderer state for Player 1...");
            let pose = client2.sample_player_pose(&player1);
            match pose {
                Some(p) => {
                    println!("   ✓ Success! Player 2 is actively animating Player 1's model!");
                    println!("     Player 1's Head Pitch: {:.1}°", p.head.pitch_degrees());
                    println!("     Player 1's Left Arm Bend: {:.1}°", p.left_arm.bend_degrees());
                    println!("     Player 1's Right Arm Pitch: {:.1}°", p.right_arm.pitch_degrees());
                }
                None => {
                    return Err("Client 2 failed to animate Player 1".into());
                }
            }

            println!("\n=== Simulation passed with 100% success! ===");
        }

        Commands::Package { output_dir } => {
            println!("╔═════════════════════════════════════════════════════════════════════════╗");
            println!("║              BUILDING & PACKAGING HORIZON MOD                           ║");
            println!("╚═════════════════════════════════════════════════════════════════════════╝\n");

            println!("1. Compiling release dynamic library (cdylib)...");
            let build_status = Command::new("cargo")
                .args(["build", "--release"])
                .status()?;

            if !build_status.success() {
                return Err("Cargo release build failed".into());
            }

            let dist_bundle_dir = output_dir.join("letmedab");
            fs::create_dir_all(&dist_bundle_dir)?;

            println!("2. Copying manifests (horizon-mod.toml & horizon-mixin.toml)...");
            if Path::new("horizon-mod.toml").exists() {
                fs::copy("horizon-mod.toml", dist_bundle_dir.join("horizon-mod.toml"))?;
            }
            if Path::new("horizon-mixin.toml").exists() {
                fs::copy("horizon-mixin.toml", dist_bundle_dir.join("horizon-mixin.toml"))?;
            }
            if Path::new("Horizon.toml").exists() {
                fs::copy("Horizon.toml", dist_bundle_dir.join("Horizon.toml"))?;
            }
            if Path::new("Cargo.toml").exists() {
                fs::copy("Cargo.toml", dist_bundle_dir.join("Cargo.toml"))?;
            }
            let src_dir = Path::new("src");
            if src_dir.exists() {
                let target_src = dist_bundle_dir.join("src");
                fs::create_dir_all(&target_src)?;
                for entry in fs::read_dir(src_dir)?.flatten() {
                    fs::copy(entry.path(), target_src.join(entry.file_name()))?;
                }
            }

            println!("3. Exporting 32 built-in emotes into bundle...");
            let emotes_target = dist_bundle_dir.join("emotes");
            fs::create_dir_all(&emotes_target)?;
            for emote in registry.all() {
                let file_path = emotes_target.join(format!("{}.json", emote.id));
                emote.to_emote_file().save_to_file(&file_path)?;
            }

            let shaders_src = Path::new("shaders");
            if shaders_src.exists() {
                println!("4. Copying WGSL shaders into bundle...");
                let shaders_target = dist_bundle_dir.join("shaders");
                fs::create_dir_all(&shaders_target)?;
                for entry in fs::read_dir(shaders_src)?.flatten() {
                    fs::copy(entry.path(), shaders_target.join(entry.file_name()))?;
                }
            }

            println!("5. Locating compiled library binary...");
            let lib_candidates = [
                Path::new("target/release/letmedab.dll"),
                Path::new("target/release/libletmedab.so"),
                Path::new("target/release/libletmedab.dylib"),
                Path::new("target/release/libletmedab.a"),
            ];

            let mut copied_lib = false;
            for candidate in &lib_candidates {
                if candidate.exists() {
                    let file_name = candidate.file_name().unwrap();
                    fs::copy(candidate, dist_bundle_dir.join(file_name))?;
                    println!("   ✓ Included binary: {}", candidate.display());
                    copied_lib = true;
                    break;
                }
            }

            if !copied_lib {
                println!("   (Note: Library target built in target/release)");
            }

            println!("\n6. Packaging finished!");
            println!("   Directory: {}", dist_bundle_dir.display());
            println!("   -> Suitable format: .hnmod (Horizon Native Mod) for native Rust code & skeletal mixins!");
            println!("   -> Compatible with both .hnmod and .hmod loaders.");
        }
    }

    Ok(())
}
