struct Vertex {
    position: vec3<f32>,
    color: vec3<f32>,
    world_normal: vec3<f32>,
    world_position: vec3<f32>,
    material: vec2<f32>,
    uv: vec2<f32>,
    texture_id: u32,
}

fn calculate_light(light: Light, in: Vertex, color: vec3<f32>) -> vec3<f32> {
    let normal = normalize(in.world_normal);
    let camera_normal = normalize(camera.position.xyz - in.world_position);

    let light_vec = light.lerp_position.xyz - in.world_position;
    let norm_light = normalize(light_vec);

    let PI: f32 = 3.14159265358979323846;
    var intensity: f32;

    if light.lerp_attributes[2] > 180.0 {
        let fov = (360.0 - light.lerp_attributes[2]) * (PI / 180.0);
        let cone_edge = cos(fov * 0.55);
        let soft_edge = cos(fov * 0.5);
        let cos_angle = dot(norm_light, normalize(light.lerp_rotation.xyz));

        intensity = 1.0 - smoothstep(cone_edge, soft_edge, cos_angle);
    } else {
        let fov = light.lerp_attributes[2] * (PI / 180.0);
        let cone_edge = cos(fov * 0.5);
        let soft_edge = cos(fov * 0.45);

        let cos_angle = dot(norm_light, -normalize(light.lerp_rotation.xyz));
        intensity = smoothstep(cone_edge, soft_edge, cos_angle);
    }

    let radius = light.lerp_attributes[1];
    let dist = length(light_vec);


    if intensity == 0.0 || dist > radius {
        return vec3<f32>(0.0);
    }

    let something = normalize(norm_light + camera_normal);
    let diff = max(dot(normal, norm_light), 0.0);

    let shininess = in.material[0];
    let spec = pow(max(dot(normal, something), 0.0), shininess);

    let metallic = in.material[1];
    let spec_colour = mix(vec3<f32>(0.04), color * light.lerp_color.rgb, metallic);

    let softness = light.lerp_attributes[0];

    let attenuation = light.lerp_color.a * (pow(radius / dist, softness) - 1.0);
    let diffuse = color * diff * (1.0 - metallic);
    let specular = spec_colour * spec;
    return (diffuse + specular) * intensity * attenuation;
}