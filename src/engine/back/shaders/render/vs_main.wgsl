@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let object = objects[in.object_id];

    let rot_mat = mat3x3<f32>(
        object.rot_mat1.xyz,
        object.rot_mat2.xyz,
        object.rot_mat3.xyz
    );


    let rotated_pos = rot_mat * (in.pos * object.size.xyz);
    let world_pos = rotated_pos + object.center.xyz;

    let normals = vec3<f32>(
        f32 (in.normals & 1023),
        f32 (in.normals >> 10 & 1023),
        f32 (in.normals >> 20 & 1023)
    );

    out.uv = in.uv;
    out.material = vec2<f32>(object.center.w, object.size.w);

    out.color = vec3<f32>(mix(
        unpack4x8unorm(bitcast<u32>(object.rot_mat1.w)),
        unpack4x8unorm(bitcast<u32>(object.rot_mat2.w)),
        globals.t
    ).xyz);

    out.world_normal = normalize(rot_mat * normalize((normals / 1023.0) * 2.0 - 1.0));
    out.world_position = world_pos;
    out.position = camera.view_proj * vec4<f32>(world_pos, 1.0);

    out.id = bitcast<u32>(object.rot_mat3.w);

    return out;
}