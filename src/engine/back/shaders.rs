pub struct Shaders<'a> {
    pub once: &'a str,
    pub objects: &'a str,
    pub lights: &'a str,

    pub forward_rendering: &'a str,
    pub per_pixel_forward_rendering: &'a str,

    pub deferred_vertex: &'a str,
    pub deferred_fragment: &'a str,

    pub light_rays: &'a str,
    pub mipmap: &'a str,
}

impl<'a> Shaders<'a> {
    pub fn new() -> Shaders<'a> {
        let once = concat!(
            include_str!("shaders\\structs\\globals.wgsl"), '\n',
            include_str!("shaders\\structs\\camera.wgsl"), '\n',

            include_str!("shaders\\compute\\once.wgsl"),
        );

        let objects = concat!(
            include_str!("shaders\\structs\\globals.wgsl"), '\n',
            include_str!("shaders\\structs\\camera.wgsl"), '\n',
            include_str!("shaders\\structs\\object.wgsl"), '\n',

            include_str!("shaders\\r_parts\\slerp.wgsl"), '\n',
            include_str!("shaders\\r_parts\\quad_to_mat3.wgsl"), '\n',

            include_str!("shaders\\compute\\objects.wgsl"),
        );

        let lights = concat!(
            include_str!("shaders\\structs\\globals.wgsl"), '\n',
            include_str!("shaders\\structs\\camera.wgsl"), '\n',
            include_str!("shaders\\structs\\light.wgsl"), '\n',
    
            include_str!("shaders\\compute\\lights.wgsl"),
        );


        let forward_rendering = concat!(
            include_str!("shaders\\structs\\vertex_input.wgsl"), '\n',
            include_str!("shaders\\structs\\vertex_output.wgsl"), '\n',
            include_str!("shaders\\structs\\globals.wgsl"), '\n',
            include_str!("shaders\\structs\\camera.wgsl"), '\n',
            include_str!("shaders\\structs\\object.wgsl"), '\n',
            include_str!("shaders\\structs\\light.wgsl"), '\n',
        
            include_str!("shaders\\r_parts\\light_math\\light.wgsl"), '\n',
            include_str!("shaders\\r_parts\\light_math\\light_rays.wgsl"), '\n',
            include_str!("shaders\\r_parts\\colours.wgsl"), '\n',

            include_str!("shaders\\render\\vs_main.wgsl"), '\n',
            include_str!("shaders\\render\\forward_shader.wgsl"),
        );

        let per_pixel_forward_rendering = concat!(
            include_str!("shaders\\structs\\globals.wgsl"), '\n',
            include_str!("shaders\\structs\\camera.wgsl"), '\n',
            include_str!("shaders\\structs\\light.wgsl"), '\n',

            include_str!("shaders\\r_parts\\light_math\\light_rays.wgsl"), '\n',

            include_str!("shaders\\render\\vs_main_fill.wgsl"), '\n',
            include_str!("shaders\\render\\per_pixel_forward_shader.wgsl"),
        );


        let deferred_vertex = concat!(
            include_str!("shaders\\structs\\vertex_input.wgsl"), '\n',
            include_str!("shaders\\structs\\vertex_output.wgsl"), '\n',
            include_str!("shaders\\structs\\globals.wgsl"), '\n',
            include_str!("shaders\\structs\\camera.wgsl"), '\n',
            include_str!("shaders\\structs\\object.wgsl"), '\n',

            include_str!("shaders\\render\\vs_main.wgsl"), '\n',
            include_str!("shaders\\render\\deferred_v_shader.wgsl"),
        );

        let deferred_fragment = concat!(
            include_str!("shaders\\structs\\globals.wgsl"), '\n',
            include_str!("shaders\\structs\\camera.wgsl"), '\n',
            include_str!("shaders\\structs\\light.wgsl"), '\n',

            include_str!("shaders\\r_parts\\light_math\\light.wgsl"), '\n',
            include_str!("shaders\\r_parts\\light_math\\light_rays.wgsl"), '\n',
            include_str!("shaders\\r_parts\\colours.wgsl"), '\n',

            include_str!("shaders\\render\\vs_main_fill.wgsl"), '\n',
            include_str!("shaders\\render\\deferred_f_shader.wgsl"),
        );

        let light_rays = concat!(
            include_str!("shaders\\render\\vs_main_fill_uv.wgsl"), '\n',
            include_str!("shaders\\render\\light_rays.wgsl"),
        );

        let mipmap = concat!(
            include_str!("shaders\\render\\vs_main_fill_uv.wgsl"), '\n',
            include_str!("shaders\\render\\mipmap.wgsl"),
        );

        Shaders {
            once,
            objects,
            lights,

            forward_rendering,
            per_pixel_forward_rendering,
            
            deferred_vertex,
            deferred_fragment,

            light_rays,
            mipmap,
        }
    }
}