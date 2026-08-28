@group(0) @binding(0)
var<storage, read_write> globals: Globals;

@group(0) @binding(1)
var<storage, read_write> camera: Camera;

@compute @workgroup_size(1, 1, 1)
fn compute_main(@builtin(global_invocation_id) id: vec3<u32>) {
    calculate_planes();
    globals.lerp_scene_light = mix(globals.old_scene_light, globals.scene_light, globals.t);
}

fn calculate_planes() {
    var planes: array<Plane, 6>;

    let tr = transpose(camera.view_proj);
    let row1 = tr[0];
    let row2 = tr[1];
    let row3 = tr[2];
    let row4 = tr[3];

    let left = row4 + row1;
    planes[0].normal = normalize(left.xyz);
    planes[0].d = left.w / length(left.xyz);

    let right = row4 - row1;
    planes[1].normal = normalize(right.xyz);
    planes[1].d = right.w / length(right.xyz);

    let bottom = row4 + row2;
    planes[2].normal = normalize(bottom.xyz);
    planes[2].d = bottom.w / length(bottom.xyz);

    let top = row4 - row2;
    planes[3].normal = normalize(top.xyz);
    planes[3].d = top.w / length(top.xyz);

    let near = row4 + row3;
    planes[4].normal = normalize(near.xyz);
    planes[4].d = near.w / length(near.xyz);

    let far = row4 - row3;
    planes[5].normal = normalize(far.xyz);
    planes[5].d = far.w / length(far.xyz);

    camera.planes = planes;
}