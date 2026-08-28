@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = mix(in.old_color, in.color, globals.t);

    let object = objects[in.object_id];

    let rotated_pos = object.rot_mat * (mix(in.old_pos, in.pos, globals.t) * object.lerp_size.xyz);
    let world_pos = rotated_pos + object.lerp_center.xyz;

    out.texture_id = object.texture_id;
    out.uv = mix(in.old_uv, in.uv, globals.t);
    out.material = mix(in.old_material, in.material, globals.t);
    out.world_normal = normalize(object.rot_mat * mix(in.old_normal, in.normal, globals.t));
    out.world_position = world_pos;
    out.position = camera.view_proj * vec4<f32>(world_pos, 1.0);

    return out;
}