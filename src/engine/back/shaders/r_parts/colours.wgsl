fn get_colour(vertex: Vertex) -> vec3<f32> {
    var color = vertex.color;
    var light_colours = vec3<f32>(0.0);

    for (var i: u32 = 0; i < light_counter; i++) {
        let light = lights[light_ids[i]];
        let light_value = calculate_light(light, vertex, color);

        light_colours += light_value;
    }
    color *= globals.lerp_scene_light;
    color += light_colours;
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));

    return color;
}