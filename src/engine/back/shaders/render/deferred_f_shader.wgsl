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
var depth_texture: texture_depth_2d;

@group(1) @binding(1)
var data_texture: texture_2d<f32>;

@group(1) @binding(2)
var color_texture: texture_2d<f32>;

@group(1) @binding(3)
var uv_texture: texture_2d<f32>;

@group(1) @binding(4)
var rays_texture: texture_storage_2d<rgba16float, write>;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var vertex: Vertex;
    let pixel_coords = vec2<i32>(floor(in.position.xy));
    let depth = textureLoad(depth_texture, pixel_coords, 0);

    if depth >= 1.0 {
        calculate_light_rays(in.position.xy, depth, vec3<f32>(0.0), vec3<f32>(0.0));
        return vec4<f32>(0.0);
    }

    vertex.world_position = get_world_pos(pixel_coords, depth);

    let data2 = textureLoad(color_texture, pixel_coords, 0);
    let z_direction = fract(data2.a) > 0.0;
    vertex.texture_id = u32 (floor(data2.a));

    let data1 = textureLoad(data_texture, pixel_coords, 0);
    vertex.material = data1.zw;

    let xy_normals = data1.xy;
    let z_normal = select(-1.0, 1.0, z_direction) * sqrt(max(0.0, 1.0 - dot(xy_normals, xy_normals)));
    vertex.world_normal = vec3<f32>(xy_normals, z_normal);

    vertex.color = data2.rgb;

    vertex.uv = textureLoad(uv_texture, pixel_coords, 0).xy;
    calculate_light_rays(in.position.xy, depth, vertex.world_normal, vertex.world_position);

    return vec4<f32>(get_colour(vertex), 1.0);
}

fn get_world_pos(pixel_coords: vec2<i32>, depth: f32) -> vec3<f32> {
    let texture_size = vec2<f32>(textureDimensions(depth_texture, 0));
    let coords = vec2<f32>(pixel_coords) / texture_size;

    let position = vec2<f32>(coords.x * 2.0 - 1.0, 1.0 - coords.y * 2.0);
    let full_pos = vec4<f32>(position, depth, 1.0);

    let world_pos_4 = camera.inv_view_proj * full_pos;
    let world_pos = world_pos_4.xyz / world_pos_4.w;
    return world_pos;
}