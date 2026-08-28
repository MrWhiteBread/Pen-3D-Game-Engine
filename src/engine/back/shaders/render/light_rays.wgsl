@group(0) @binding(0)
var rays_texture: texture_2d<f32>;

@fragment
fn fs_main_x(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = vec4<f32>(0.0);
    var color_count = 0.0;
    let offset = 10.0;

    for (var i = -offset; i <= offset; i += 1) {
        let x = in.position.x + i;
        if x < 0 || x >= f32 (textureDimensions(rays_texture).x) {
            continue;
        }

        let pixel_position = vec2<i32>(i32 (x), i32 (in.position.y));
        color += textureLoad(rays_texture, pixel_position, 0);
        color_count += 1;
    }

    return vec4<f32>(clamp((color.xyz / color_count), vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}

@fragment
fn fs_main_y(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = vec4<f32>(0.0);
    var color_count = 0.0;
    let offset = 10.0;

    for (var i = -offset; i <= offset; i += 1) {
        let y = in.position.y + i;
        if y < 0 || y >= f32 (textureDimensions(rays_texture).y) {
            continue;
        }

        let pixel_position = vec2<i32>(i32 (in.position.x), i32 (y));
        color += textureLoad(rays_texture, pixel_position, 0);
        color_count += 1;
    }

    return vec4<f32>(clamp((color.xyz / color_count), vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}