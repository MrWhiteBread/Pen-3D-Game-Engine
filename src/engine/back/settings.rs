use winit::dpi::PhysicalSize;

pub struct Settings {
    pub time_per_update: f64,
    pub deferred_rendering: bool,
    
    pub size: PhysicalSize<u32>,
    pub old_size: PhysicalSize<u32>,
}