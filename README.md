# LetmeDab

> **Emotecraft animation plugin and native mod for [Horizon (Minecraft Rust Edition)](https://playhorizon.dev).**

---

## ✨ Features

- **🎮 Full Emotecraft Format Parity**:
  - Native parser and serializer for `.emotecraft` and `.json` animation files.
  - Supports all 28+ Emotecraft easing functions (`Linear`, `EaseInOutSine`, `EaseOutCubic`, `EaseOutBounce`, `EaseInBack`, `Step`, etc.).
  - Full humanoid body part transforms: `head`, `torso`, `rightArm`, `leftArm`, `rightLeg`, `leftLeg`.
  - Supports rotations (degrees or radians), positions (x, y, z), and limb bending angles.
  - Supports looping emotes (`isLoop`, `returnToTick`, `beginTick`, `endTick`, `stopTick`).

- **⌨️ Horizon Keybinds & Radial Emote Wheel**:
  - Default keybindings registered in [`horizon-mod.toml`](file:///c:/Users/Sarvadnya/LetmeDAB/horizon-mod.toml):
    - **`B`**: Open / Toggle Radial Emote Wheel (8 customizable quick slots)
    - **`X`**: Stop active emote (with smooth fade-out)
    - **`G`**: Quick Dab!
    - **`H`**: Quick Wave!
  - Fully remappable in Horizon's Controls menu.

- **🕺 32 Built-in Emotes Included**:
  - **Classics**: `dab`, `wave`, `bow`, `cheer`, `salute`, `facepalm`, `tpose`
  - **Social & Expression**: `shrug`, `clap`, `cry`, `laugh`, `point`, `think`, `peace`, `heart`, `beckon`, `slow_clap`, `chuckle`, `surrender`
  - **Posture & Roleplay**: `sit`, `sleep`, `zombie`, `ninja`, `flex`
  - **Dances & Fitness**: `floss`, `gangnam`, `spin`, `guitar`, `disco`, `robot`, `pushup`, `jumping_jack`

- **🎨 Horizon Shader API Integration (WGSL)**:
  - Compatible with [Horizon Shader API](https://playhorizon.dev/docs/api/shaders).
  - Uses native **WebGPU Shading Language (WGSL)** compiled via `wgpu` across Vulkan, Metal, and DirectX 12.
  - Bundled [`shaders/horizon-shaders.toml`](file:///c:/Users/Sarvadnya/LetmeDAB/shaders/horizon-shaders.toml) and [`shaders/emote_fx.wgsl`](file:///c:/Users/Sarvadnya/LetmeDAB/shaders/emote_fx.wgsl) adding dynamic party lighting and cinematic emote vignette post-processing passes.

- **🌐 Multiplayer & Network Synchronization**:
  - Uses Minecraft Custom Payload packets over the standard `emotecraft:main` plugin channel (Protocol v8).
  - Full packet handling: Handshake (`0x00`), Play (`0x01`), Stop (`0x02`), SyncList (`0x03`).
  - Server-side rate limiting, anti-cheat validation, and spatial broadcasting.

- **🔌 Horizon Plugin Integration**:
  - Native `Horizon.toml` plugin manifest.
  - Dynamic library (`cdylib`) ready for Horizon's native mod loader.
  - Direct hook into Horizon's player model renderer (`apply_model_pose`).

---

## ⌨️ Controls & Keybindings

| Key | Action | Description |
|---|---|---|
| **`B`** | `OpenWheel` | Opens/closes the 8-slot Radial Emote Wheel |
| **`X`** | `StopEmote` | Stops playing the current animation |
| **`G`** | `PlayEmote("dab")` | Instantly performs the iconic Dab |
| **`H`** | `PlayEmote("wave")` | Instantly waves hello |

### Radial Emote Wheel Default Slots:
1. `dab` • 2. `wave` • 3. `cheer` • 4. `bow` • 5. `floss` • 6. `shrug` • 7. `point` • 8. `clap`

---

## 🛠️ How to Build the Mod for Horizon

You can build the mod using either the Rust CLI or the PowerShell packaging script:

### Method A: Automated Packaging Command
```bash
cargo run -- package
```
This automatically:
1. Compiles the optimized release binary (`target/release/letmedab.dll`).
2. Generates the `dist/letmedab/` mod bundle containing `Horizon.toml`, the binary, and all 32 emote definitions.

### Method B: PowerShell Script (Builds `.hmod` file)
```powershell
.\build-mod.ps1
```
Creates `dist/letmedab-0.1.0.hmod` (a compressed Horizon mod archive ready to drop into your `mods/` directory).

---

## 🚀 Testing & CLI Tools

- **View & test keybindings**:
  ```bash
  cargo run -- keybinds B
  cargo run -- keybinds G
  ```
- **List all 32 emotes**:
  ```bash
  cargo run -- list
  ```
- **Simulate an emote in the terminal (e.g. Dab)**:
  ```bash
  cargo run -- play dab
  ```
- **Export all 32 emotes**:
  ```bash
  cargo run -- export ./emotes
  ```
- **Run multiplayer network broadcast simulation**:
  ```bash
  cargo run -- demo-network
  ```
- **Run all unit & integration tests**:
  ```bash
  cargo test
  ```

---

## 📁 Project Architecture

```
LetmeDAB/
├── Cargo.toml            # Package configuration (lib + cdylib + bin)
├── horizon-mod.toml      # Official Horizon Mod manifest & keybinds
├── Horizon.toml          # Legacy manifest alias
├── build-mod.ps1         # One-click mod building script (.hmod)
├── dist/                 # Packaged mod output (.hmod & directory)
├── src/
│   ├── lib.rs            # Library entry point & exports
│   ├── main.rs           # CLI application & mod packager
│   ├── keybind.rs        # Keybind manager & 8-slot radial wheel
│   ├── model.rs          # Humanoid skeleton, body parts & transforms
│   ├── easing.rs         # 28+ Emotecraft easing curves
│   ├── format.rs         # JSON/.emotecraft parser & compiler
│   ├── animation.rs      # High-performance tick player & lerp engine
│   ├── network.rs        # Minecraft VarInt, buffer & packet codecs
│   ├── registry.rs       # Emote catalog & directory loader
│   ├── server.rs         # Server-side relay & anti-cheat validator
│   ├── client.rs         # Client-side player pose injector
│   ├── builtin.rs        # 32 built-in animations (Dab, Wave, Floss, etc.)
│   └── plugin.rs         # Horizon lifecycle, keybind & command hooks
├── emotes/               # Exported standalone .json emote files (32 emotes)
└── tests/
    └── integration_tests.rs # Full test suite (13 tests passing)
```

---

## 📜 In-Game Commands

| Command | Description |
|---|---|
| `/emote play <name>` | Play an emote by name or ID (e.g., `/emote play dab`) |
| `/emote stop` | Stop playing the current emote |
| `/emote list` | List all installed and server-synced emotes |
| `/emote help` | Show help and keybind hints |

---

## 📄 License

Dual-licensed under MIT or Apache-2.0. Emotecraft protocol compatibility modeled after the original work by KosmX.
