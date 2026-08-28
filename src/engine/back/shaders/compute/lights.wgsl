@group(0) @binding(0) var<storage, read_write> counter: atomic<u32>;
@group(0) @binding(1) var<storage, read_write> light_ids: array<u32>;

@group(0) @binding(2)
var<storage, read> globals: Globals;

@group(0) @binding(3)
var<storage, read> camera: Camera;

@group(0) @binding(4)
var<storage, read_write> lights: array<Light>;

@group(0) @binding(5)
var<uniform> light_count: u32;

@compute @workgroup_size(64, 1, 1)
fn compute_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;

    if (idx >= light_count) {
        return;
    }

    var light = lights[idx];

    if (!frustum_culling(light)) {
        return;
    }
    let slot = atomicAdd(&counter, 1);

    light.lerp_position = mix(light.old_position, light.position, globals.t);
    light.lerp_rotation = mix(light.old_rotation, light.rotation, globals.t);
    light.lerp_color = mix(light.old_color, light.color, globals.t);
    light.lerp_attributes = mix(light.old_attributes, light.attributes, globals.t);
    light.lerp_attributes2 = mix(light.old_attributes2, light.attributes2, globals.t);
    light.lerp_attributes3 = mix(light.old_attributes3, light.attributes3, globals.t);
    light.lerp_attributes4 = mix(light.old_attributes4, light.attributes4, globals.t);
    light.screen_position = camera.view_proj * light.lerp_position;

    lights[idx] = light;

    light_ids[slot] = idx;
}

fn frustum_culling(light: Light) -> bool
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