use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use pollster::block_on;
use tokio::time::Instant;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::error::EventLoopError;
use winit::event::{DeviceEvent, Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{WindowBuilder};

use crate::engine::back::settings::Settings;
use crate::engine::back::state::State;
use crate::engine::front::run::{run, RunVar};

mod src;
mod engine;

fn main() -> Result<(), EventLoopError> {
    let window_name = "Pen 3D Engine";
    let window_size = PhysicalSize::new(1600, 900);
    let ticks_per_second = 60.0;
    let time_per_update = 1.0 / ticks_per_second;

    //REMOVE WITH_WINDOW_LEVEL() FOR A NORMAL ENGINE AND WITH_RESIZABLE() AND WITH_DECORATION()
    env_logger::init();
    let event_loop = EventLoop::new().expect("Could not create event loop");
    let window = WindowBuilder::new()
        .with_transparent(true)
        .with_title(format!("{}", window_name))
        .with_inner_size(window_size)
        //.with_window_level(WindowLevel::AlwaysOnTop)
        //.with_resizable(false)
        //.with_decorations(false)
        .build(&event_loop)
        .expect("Failed to create window");
    
    let mut settings = Settings {
        time_per_update,
        deferred_rendering: true,

        size: window_size,
        old_size: PhysicalSize::new(0, 0),
    };

    let tokio_runtime = tokio::runtime::Runtime::new().expect("Could not create tokio runtime");

    let (mut state, staging_buffers) = tokio_runtime.block_on(async {State::new(&window, &mut settings).await});

    let _guard = tokio_runtime.enter();

    let key_handler = state.key_handler.clone();
    let cursor_x = state.cursor_x.clone();
    let cursor_y = state.cursor_y.clone();
    let cursor_locked = state.cursor_locked.clone();
    
    let staging_buffers = staging_buffers.clone();
    let globals_updated = state.globals_updated.clone();
    let ticks_updated = state.ticks_updated.clone();
    let del_timer = state.del_timer.clone();
    let window_bounds = state.window_bounds.clone();
    
    let fps_atomic = Arc::new(AtomicU32::new(0));
    let fps_atomic_clone = fps_atomic.clone();

    tokio::task::spawn_blocking( move || {
        let run_var = RunVar{
            time_per_update,
            staging_buffers,
            key_handler,
            cursor_x,
            cursor_y,
            cursor_locked,
            window_bounds,

            fps: fps_atomic_clone,
            globals_updated,
            ticks_updated,
            del_timer,
        };

        block_on(run(run_var));
    });
    
    let window_ref = &window;
    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut sh_up_mouse_lk = false;
    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;
    
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run(move |event: Event<()>, target | {
        let cursor_locked = state.cursor_locked.load(Ordering::Relaxed);
        let (window_position, window_size, changed, minimised): ([i32; 2], [u32; 2], bool, bool) = **state.window_bounds.load();

        if changed {
            window_ref.set_outer_position(PhysicalPosition::new(window_position[0], window_position[1]));
            let _ = window_ref.request_inner_size(PhysicalSize::new(window_size[0], window_size[1]));
            window_ref.set_minimized(minimised);
            state.window_bounds.store(Arc::new((window_position, window_size, false, minimised)));
        }

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window_ref.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                            state.settings.old_size = *physical_size;
                        }


                        WindowEvent::CursorMoved { position, .. } => {
                            let size = window_ref.inner_size();

                            if !cursor_locked {
                                mouse_x = position.x / size.width as f64;
                                mouse_y = position.y / size.height as f64;
                            }
                        }
                        WindowEvent::CloseRequested => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
            }

            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta: (x, y) }, .. } => {
                if cursor_locked {
                    mouse_x = x;
                    mouse_y = y;
                }
            }

            Event::AboutToWait => {
                if state.globals_updated.load(Ordering::Relaxed) {
                    state.globals_updated.store(false, Ordering::Relaxed);

                    frame_count += 1;
                    if last_time.elapsed().as_secs_f32() >= 1.0 {
                        //println!("FPS: {}", frame_count);
                        fps_atomic.store(frame_count, Ordering::Relaxed);
                        frame_count = 0;
                        last_time = Instant::now();
                    }

                    let cursor_x = state.cursor_x.load(Ordering::Relaxed);
                    let cursor_y = state.cursor_y.load(Ordering::Relaxed);

                    state
                        .cursor_x
                        .store(cursor_x + mouse_x as f32, Ordering::Relaxed);
                    state
                        .cursor_y
                        .store(cursor_y + mouse_y as f32, Ordering::Relaxed);

                    mouse_x = 0.0;
                    mouse_y = 0.0;

                    if cursor_locked {
                        sh_up_mouse_lk = true;
                        let size = window_ref.inner_size();
                        let center = (size.width as f32 / 2.0, size.height as f32 / 2.0);
                        window_ref.set_cursor_position(PhysicalPosition::new(center.0, center.1)).ok();

                        window_ref.set_cursor_visible(false);
                    } else if sh_up_mouse_lk {
                        //println!("fullscreen");
                        window_ref.set_cursor_visible(true);
                        sh_up_mouse_lk = false;
                    }

                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => state.resize(window_ref.inner_size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                        Err(_) => {}
                    }

                    window_ref.request_redraw();
                }
            }

            _ => {}
        }
    })
}
