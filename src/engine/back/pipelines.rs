mod forward;
pub mod pass_state;
mod compute_objects;
mod deferred_v;
mod deferred_f;
mod compute_lights;
mod compute_once;
mod per_pixel_forward;
mod light_rays;
mod mipmap;

use wgpu::{SurfaceConfiguration};

use crate::engine::back::buffers::Buffers;
use crate::engine::back::screen_textures::ScreenTextures;
use crate::engine::back::pipelines::compute_lights::compute_lights;
use crate::engine::back::pipelines::compute_objects::compute_objects;
use crate::engine::back::pipelines::compute_once::compute_once;
use crate::engine::back::pipelines::deferred_f::deferred_f;
use crate::engine::back::pipelines::deferred_v::deferred_v;
use crate::engine::back::pipelines::forward::{forward};
use crate::engine::back::pipelines::light_rays::light_rays;
use crate::engine::back::pipelines::mipmap::mipmap;
use crate::engine::back::pipelines::pass_state::{ComputePassState, DoublePassState, PassState, ScreenPassState, SimplePassState};
use crate::engine::back::pipelines::per_pixel_forward::per_pixel_forward;
use crate::engine::back::shaders::Shaders;

pub struct Pipelines {
    pub compute_once: ComputePassState,
    pub compute_lights: ComputePassState,
    pub compute_objects: ComputePassState,
    
    pub deferred_v: PassState,
    pub deferred_f: ScreenPassState,

    pub forward_pass: ScreenPassState,
    pub per_pixel_forward_pass: ScreenPassState,
    pub light_rays_pass: DoublePassState,
    pub mipmap: SimplePassState,
}

impl Pipelines {
    pub fn new(device: &wgpu::Device, buffers: &Buffers, config: &SurfaceConfiguration, screen_textures: &ScreenTextures) -> Pipelines {
        let shaders = Shaders::new();
        
        let compute_once = compute_once(device, buffers, &shaders);
        let compute_lights = compute_lights(device, buffers, &shaders);
        let compute_objects = compute_objects(device, buffers, &shaders);
        
        let deferred_v = deferred_v(device, buffers, &shaders);
        let deferred_f = deferred_f(device, buffers, config, &shaders, &screen_textures);

        let forward_pass = forward(device, buffers, config, &shaders, &screen_textures);
        let per_pixel_forward_pass = per_pixel_forward(device, buffers, config, &shaders, &screen_textures);

        let light_rays_pass = light_rays(device, config, &shaders, buffers, &screen_textures);
        let mipmap = mipmap(device, &shaders);

        Self {
            compute_once,
            compute_lights,
            compute_objects,

            deferred_v,
            deferred_f,
            forward_pass,
            per_pixel_forward_pass,
            light_rays_pass,
            mipmap,
        }
    }
}