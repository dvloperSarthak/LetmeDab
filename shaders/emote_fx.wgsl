// LetmeDab - Horizon WGSL Post-Processing & Emote FX Shader
// Written in WebGPU Shading Language (WGSL) for Horizon Minecraft (Rust Edition)
// Compatible with Horizon Shader API (https://playhorizon.dev/docs/api/shaders)

struct FrameInput {
    time: f32,
    resolution: vec2<f32>,
    camera_fov: f32,
    emote_active: u32,
    emote_progress: f32,
};

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;
@group(0) @binding(2) var<uniform> frame: FrameInput;

override enable_party_lights: bool = true;
override enable_vignette: bool = true;
override intensity: f32 = 0.75;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32((in_vertex_index << 1u) & 2u);
    let y = f32(in_vertex_index & 2u);
    out.position = vec4<f32>(x * 2.0 - 1.0, y * -2.0 + 1.0, 0.0, 1.0);
    out.uv = vec2<f32>(x, y);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(screen_texture, screen_sampler, in.uv);

    // Subtle cinematic vignette when an emote is actively playing
    if (enable_vignette) {
        let dist = distance(in.uv, vec2<f32>(0.5, 0.5));
        let vig = smoothstep(0.8, 0.25, dist);
        color = vec4<f32>(color.rgb * mix(1.0, vig, intensity * 0.4), color.a);
    }

    // Party disco color shifting highlight during dance emotes
    if (enable_party_lights && frame.emote_active != 0u) {
        let t = frame.time * 2.0;
        let r_shift = sin(t) * 0.08 * intensity;
        let g_shift = sin(t + 2.094) * 0.08 * intensity;
        let b_shift = sin(t + 4.188) * 0.08 * intensity;

        color = vec4<f32>(
            clamp(color.r + r_shift, 0.0, 1.0),
            clamp(color.g + g_shift, 0.0, 1.0),
            clamp(color.b + b_shift, 0.0, 1.0),
            color.a
        );
    }

    return color;
}
