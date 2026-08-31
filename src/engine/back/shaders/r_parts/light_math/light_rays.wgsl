fn calculate_light_rays(position: vec2<f32>, depth: f32, normals: vec3<f32>, world_position: vec3<f32>) {
    var ray_colours = vec3<f32>(0.0);

    for (var i: u32 = 0; i < light_counter; i++) {
        let light = lights[light_ids[i]];
        let ray_value = calculate_ray(light, position, depth, normals, world_position);

        ray_colours += ray_value;
    }
    ray_colours = clamp(ray_colours, vec3<f32>(0.0), vec3<f32>(1.0));

    textureStore(rays_texture, vec2<i32>(position.xy), vec4<f32>(ray_colours, 1.0));
}

fn calculate_ray(light: Light, position: vec2<f32>, depth: f32, normals: vec3<f32>, world_position: vec3<f32>) -> vec3<f32> {
    let light_depth = light.screen_position.z / light.screen_position.w;
    let light_ndc = light.screen_position.xy / light.screen_position.w;

    if light_ndc.x > 1.0 || light_ndc.x < -1.0
        || light_ndc.y > 1.0 || light_ndc.y < -1.0 || light.screen_position.z < 0.0 {
        return vec3<f32>(0.0);
    }

    let light_vec = light.lerp_position.xyz - world_position;
    let norm_light = normalize(light_vec);

    if light.lerp_attributes2[3] >= 0 && depth < 1.0 && ((light.lerp_attributes2[0] >= 0.0 && depth < light_depth) || (light.lerp_attributes2[0] < 0.0 && dot(normalize(normals), norm_light) <= 0.0)) {
        return vec3<f32>(0.0);
    }

    let pix_pos = position / globals.screen_size;
    let pixel_ndc = vec2<f32>(pix_pos.x * 2.0 - 1.0, (1.0 - pix_pos.y) * 2.0 - 1.0);

    let dist = distance(light_ndc, pixel_ndc);

    var range = abs(light.lerp_attributes2[3]);
    var intensity = light.lerp_attributes3[2];
    var softness = light.lerp_attributes3[1];
    var w = light.screen_position.w;

    let sec_attenuation = pow(clamp(1.0 - dist / range * 2.0 * w, 0.0, 1.0), softness) * intensity;

    if light.lerp_attributes[3] != 0.0 && light.lerp_attributes2[0] != 0.0 && light.lerp_attributes2[2] != 1.0 {
        let PI = 3.14159;
        let norm = normalize(light_ndc - pixel_ndc);
        var angle = atan2(norm.x, norm.y);

        let seed = light.lerp_attributes3[3];
        var r_intensity = 1.0;
        let angle_roughness = light.lerp_attributes4[0];
        let empty_delta = light.lerp_attributes4[1];
        let empty_intensity = light.lerp_attributes[3];
        var r: f32;

        if seed > 0.0 {
            let as_val = pow(10.0, angle_roughness);
            angle = floor((angle + PI) / (2.0 * PI) * as_val) / as_val;

            r = f32 (u32 (angle * seed) * 1664525u + 1013904223u) % light.lerp_attributes4[2] / light.lerp_attributes4[3];
            r_intensity = f32 (u32 (r * seed) * 1664525u + 1013904223u) % 1000 / 100.0;
        } else {
            r = (angle + PI) / (2.0 * PI);
        }

        let section = floor(r * floor(light.lerp_attributes2[1]));

        if u32 (section) % u32 (floor(light.lerp_attributes2[2])) == 0 {
            range = abs(light.lerp_attributes2[0]);
            intensity = light.lerp_attributes[3] * r_intensity;
            softness = light.lerp_attributes3[0];
            w = 1.0;
        } else if seed > 0.0 {
            range = abs(light.lerp_attributes2[0]) * mix(fract(section / floor(light.lerp_attributes2[2])), 1.0, empty_delta);
            intensity = empty_intensity * mix(fract(section / floor(light.lerp_attributes2[2])), 1.0, empty_delta);
            softness = light.lerp_attributes3[0];
            w = 1.0;
        }
    }

    let attenuation = pow(clamp(1.0 - dist / range * 2.0 * w, 0.0, 1.0), softness) * intensity;
    let max_attenuation = max(attenuation, sec_attenuation);

    return light.lerp_color.rgb * max_attenuation;
}