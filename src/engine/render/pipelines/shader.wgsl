struct Uniforms {
    view_proj: mat4x4<f32>,
    light_view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
    fog: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var shadow_texture: texture_depth_2d;
@group(1) @binding(1)
var shadow_sampler: sampler_comparison;

@group(2) @binding(0)
var block_texture: texture_2d_array<f32>;
@group(2) @binding(1)
var block_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) tex_layer: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) @interpolate(flat) tex_layer: i32,
    @location(3) shadow_pos: vec4<f32>,
    @location(4) world_pos: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    out.uv = model.uv;
    out.tex_layer = i32(model.tex_layer);
    out.shadow_pos = uniforms.light_view_proj * vec4<f32>(model.position, 1.0);
    out.world_pos = model.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.tex_layer < 0) {
        if (in.tex_layer == -2) { return vec4<f32>(1.0, 0.25, 0.25, 1.0); }
        if (in.tex_layer == -3) { return vec4<f32>(1.0, 1.0, 0.0, 1.0); }
        if (in.tex_layer == -4) { return vec4<f32>(0.0, 0.61, 0.61, 1.0); }
        if (in.tex_layer == -5) { return vec4<f32>(0.5, 0.0, 0.0, 1.0); }
        return vec4<f32>(1.0, 0.85, 0.1, 1.0);
    }
    let tex_color = textureSample(block_texture, block_sampler, in.uv, in.tex_layer);
    if (tex_color.a < 0.1) { discard; }

    let light_dir = normalize(vec3<f32>(0.5, 1.2, 0.6));
    let diff = max(dot(in.normal, light_dir), 0.0);
    let proj_coords = in.shadow_pos.xyz / in.shadow_pos.w;
    let shadow_uv = vec2<f32>(proj_coords.x * 0.5 + 0.5, -proj_coords.y * 0.5 + 0.5);
    let bias = max(0.002 * (1.0 - diff), 0.0005);
    let current_depth = proj_coords.z - bias;

    var shadow: f32 = 0.0;
    if (shadow_uv.x >= 0.0 && shadow_uv.x <= 1.0 && 
        shadow_uv.y >= 0.0 && shadow_uv.y <= 1.0 && 
        proj_coords.z >= 0.0 && proj_coords.z <= 1.0) {
        let texel_size = 1.0 / 2048.0;
        for (var x = -1; x <= 1; x++) {
            for (var y = -1; y <= 1; y++) {
                let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
                shadow += textureSampleCompare(
                    shadow_texture, shadow_sampler, shadow_uv + offset, current_depth
                );
            }
        }
        shadow = shadow / 9.0;
    } else {
        shadow = 1.0;
    }

    let ambient = 0.35;
    let direct = diff * shadow * 0.65;
    var alpha: f32 = tex_color.a;
    if (in.tex_layer == 4 || in.tex_layer == 5) {
        alpha = 0.80;
    } else if (in.tex_layer >= 18 && in.tex_layer <= 27) {
        alpha = 0.75;
    }
    let lit = tex_color.rgb * (ambient + direct);
    let fogf = clamp((distance(in.world_pos, uniforms.camera_pos.xyz) - uniforms.fog.x) / max(uniforms.fog.y - uniforms.fog.x, 1e-4), 0.0, 1.0);
    let fog_color = vec3<f32>(0.1, 0.12, 0.16);
    return vec4<f32>(mix(lit, fog_color, fogf), alpha);
}

@vertex
fn vs_shadow(model: VertexInput) -> @builtin(position) vec4<f32> {
    return uniforms.light_view_proj * vec4<f32>(model.position, 1.0);
}