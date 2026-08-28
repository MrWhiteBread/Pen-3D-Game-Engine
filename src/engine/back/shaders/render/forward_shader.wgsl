@group(0) @binding(0)
var<storage, read> globals: Globals;

@group(0) @binding(1)
var<storage, read> camera: Camera;

@group(0) @binding(2)
var<storage, read> objects: array<Object>;

@group(0) @binding(3)
var<storage, read> lights: array<Light>;

@group(0) @binding(4)
var<storage, read> light_counter: u32;

@group(0) @binding(5)
var<storage, read> light_ids: array<u32>;

@group(1) @binding(0)
var rays_texture: texture_storage_2d<rgba16float, write>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var colour = in.color.rgb;

    var vertex: Vertex;
    vertex.position = in.position.xyz;
    vertex.color = in.color.xyz;
    vertex.world_normal = in.world_normal;
    vertex.world_position = in.world_position;
    vertex.material = in.material;
    vertex.uv = in.uv;
    vertex.texture_id = in.texture_id;
    calculate_light_rays(in.position.xy, in.position.z, vertex.world_normal, vertex.world_position);

    return vec4<f32>(get_colour(vertex), 1.0);
}