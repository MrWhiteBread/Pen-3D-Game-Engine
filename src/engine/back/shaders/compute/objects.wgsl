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
var<storage, read> raw_objects: array<RawObject>;

@group(0) @binding(6)
var<storage, read_write> objects: array<Object>;

@group(0) @binding(7)
var<uniform> object_count: u32;

var<workgroup> counter: atomic<u32>;
var<workgroup> index: u32;

@compute @workgroup_size(64, 1, 1)
fn compute_main(@builtin(local_invocation_id) local_id: vec3<u32>, @builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;

    if local_id.x == 0 {
        atomicStore(&counter, 0);
    }

    workgroupBarrier();

    var valid = false;
    var raw_obj: RawObject;

    if idx < object_count {
        raw_obj = raw_objects[idx];

        if unpack4x8unorm(raw_obj.color).w != 0.0 && frustum_culling(raw_obj) && raw_obj.index_count > 0 {
            valid = true;
        }
    }

    var local_slot = 0u;
    if valid {
        local_slot = atomicAdd(&counter, 1);
    }

    workgroupBarrier();

    if local_id.x == 0 {
        let ctr = atomicLoad(&counter);

        if ctr > 0 {
            index = atomicAdd(&out_counter, ctr);
        }
    }

    workgroupBarrier();

    if valid {
        var obj = objects[idx];

        let q = slerp(raw_obj.old_rot, raw_obj.rot, globals.t);
        let rot_mat = quat_to_mat3(q);

        obj.rot_mat1 = vec4<f32>(rot_mat[0], bitcast<f32>(raw_obj.color));
        obj.rot_mat2 = vec4<f32>(rot_mat[1], bitcast<f32>(raw_obj.old_color));
        obj.rot_mat3 = vec4<f32>(rot_mat[2], bitcast<f32>(raw_obj.id));

        obj.center = mix(raw_obj.old_center, raw_obj.center, globals.t);
        obj.size = mix(raw_obj.old_size, raw_obj.size, globals.t);

        objects[idx] = obj;

        let slot = index + local_slot;

        out_commands[slot].index_count = raw_obj.index_count;
        out_commands[slot].instance_count = 1;
        out_commands[slot].first_index = raw_obj.first_index;
        out_commands[slot].base_vertex = raw_obj.base_vertex;
        out_commands[slot].first_instance = idx;
        object_ids[idx] = idx;
    }
}

fn frustum_culling(obj: RawObject) -> bool
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