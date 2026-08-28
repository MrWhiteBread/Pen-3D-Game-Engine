struct DrawIndexedIndirectArgs {
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
}

@group(0) @binding(0) var<storage, read_write> out_commands: array<DrawIndexedIndirectArgs>;
@group(0) @binding(1) var<storage, read_write> out_counter: atomic<u32>;
@group(0) @binding(2) var<storage, read_write> object_ids: array<u32>;

@group(0) @binding(3)
var<storage, read> globals: Globals;

@group(0) @binding(4)
var<storage, read_write> camera: Camera;

@group(0) @binding(5)
var<storage, read_write> objects: array<Object>;

@group(0) @binding(6)
var<uniform> object_count: u32;

@compute @workgroup_size(64, 1, 1)
fn compute_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;

    if (idx >= object_count) || objects[idx].status == 0 {
        return;
    }

    var obj = objects[idx];

    if (!frustum_culling(obj)) {
        return;
    }
    let slot = atomicAdd(&out_counter, 1);

    obj.lerp_size = mix(obj.old_size, obj.size, globals.t);
    obj.lerp_center = mix(obj.old_center, obj.center, globals.t);

    let q = slerp(obj.old_rot, obj.rot, globals.t);
    obj.rot_mat = quat_to_mat3(q);
    objects[idx] = obj;

    out_commands[slot].index_count = obj.index_count;
    out_commands[slot].instance_count = 1;
    out_commands[slot].first_index = obj.first_index;
    out_commands[slot].base_vertex = obj.base_vertex;
    out_commands[slot].first_instance = idx;
    object_ids[idx] = idx;
}

fn frustum_culling(obj: Object) -> bool
{
    let radius = obj.radius * max(obj.size.x, max(obj.size.y, obj.size.z));
    let center = mix(obj.old_center, obj.center, globals.t).xyz;

    for (var i = 0; i < 6; i += 1)
    {
        let distance = dot(camera.planes[i].normal, center) + camera.planes[i].d;

        if (distance + radius < 0.0)
        {
            return false;
        }
    }
    return true;
}