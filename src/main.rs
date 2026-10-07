use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32};
use tokio::time::Instant;
use winit::dpi::{PhysicalSize};
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::{App, AppVariables};


mod src;
mod engine;
mod app;

fn main() -> Result<(), EventLoopError> {
    env_logger::init();
    
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    
    
    let window_name = "Pen 3D Engine";
    let window_size = PhysicalSize::new(1600, 900);
    
    let app_variables = AppVariables {
        fps_atomic: Arc::new(AtomicU32::new(0)),

        arc_cursor_locked: Arc::new(AtomicBool::new(false)),
        cursor_locked: false,

        last_time: Instant::now(),
        frame_count: 0,
        sh_up_mouse_lk: false,
        mouse_x: 0.0,
        mouse_y: 0.0,
    };

    let mut app = App {
        window: None,
        state: None,
        window_name,
        window_size,
        tokio_runtime: None,
        
        app_variables,
    };

    event_loop.run_app(&mut app)
}
