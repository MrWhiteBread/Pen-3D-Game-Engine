

@group(0) @binding(0)
var<storage, read> globals: Globals;

@group(0) @binding(1)
var<storage, read> camera: Camera;

@group(0) @binding(2)
var<storage, read> lights: array<Light>;

@group(0) @binding(3)
var<storage, read> light_counter: u32;

@group(0) @binding(4)
var<storage, read> light_ids: array<u32>;

@group(1) @binding(0)
var rays_texture: texture_storage_2d<rgba16float, write>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    calculate_light_rays(in.position.xy, 1.0, vec3<f32>(0.0), vec3<f32>(0.0));
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}