@group(0) @binding(0) var<storage, read_write> out_counter: atomic<u32>;
@group(0) @binding(1) var<storage, read_write> light_ids: array<u32>;

@group(0) @binding(2)
var<storage, read> globals: Globals;

@group(0) @binding(3)
var<storage, read> camera: Camera;

@group(0) @binding(4)
var<storage, read> raw_lights: array<RawLight>;

@group(0) @binding(5)
var<storage, read_write> lights: array<Light>;

@group(0) @binding(6)
var<uniform> light_count: u32;

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
    var light: RawLight;

    if idx < light_count {
        light = raw_lights[idx];

        if frustum_culling(light) {
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
        lights[idx] = Light(
            mix(light.old_position, light.position, globals.t),
            mix(light.old_rotation, light.rotation, globals.t),

            mix(light.old_color, light.color, globals.t),
            mix(light.old_attributes, light.attributes, globals.t),
        );

        let slot = index + local_slot;
        light_ids[slot] = idx;
    }
}

fn frustum_culling(light: RawLight) -> bool
{
    let radius = mix(light.old_attributes[1], light.attributes[1], globals.t);
    let center = mix(light.old_position, light.position, globals.t).xyz;

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