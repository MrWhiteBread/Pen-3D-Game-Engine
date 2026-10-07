use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use pollster::block_on;
use tokio::runtime::{Runtime};
use tokio::time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::engine::back::settings::Settings;
use crate::engine::back::state::State;
use crate::engine::front::run::{run, RunVar};

pub struct AppVariables {
    pub fps_atomic: Arc<AtomicU32>,

    pub arc_cursor_locked: Arc<AtomicBool>,
    pub cursor_locked: bool,

    pub last_time: Instant,
    pub frame_count: u32,
    pub sh_up_mouse_lk: bool,
    pub mouse_x: f64,
    pub mouse_y: f64,
}

pub struct App<'a> {
    pub window: Option<Arc<Window>>,
    pub state: Option<State<'a>>,
    pub window_name: &'a str,
    pub window_size: PhysicalSize<u32>,
    pub tokio_runtime: Option<Runtime>,

    pub app_variables: AppVariables,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("title")
                .with_transparent(true)
                .with_title(format!("{}", self.window_name))
                .with_inner_size(self.window_size);

            let window = event_loop.create_window(window_attributes).expect("Could not create window");

            let ticks_per_second = 60.0;
            let time_per_update = 1.0 / ticks_per_second;

            let settings = Settings {
                time_per_update,
                deferred_rendering: true,

                size: self.window_size,
                old_size: PhysicalSize::new(0, 0),
            };

            let window = Some(Arc::new(window));
            let window_clone = window.as_ref().expect("works man").clone();

            let tokio_runtime = Runtime::new().expect("Could not create tokio runtime");

            let (mut state, staging_buffers) = tokio_runtime.block_on(
                async {
                    State::new(window_clone, settings).await
                }
            );

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

            self.app_variables.fps_atomic = fps_atomic;
            self.app_variables.arc_cursor_locked = cursor_locked.clone();

            tokio_runtime.spawn_blocking( move || {
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

            self.tokio_runtime = Some(tokio_runtime);

            self.window = window;
            self.state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let Some(window_ref) = self.window.as_ref() else { return };
        let Some(state) = self.state.as_mut() else { return };
        if window_ref.id() != window_id { return };

        let (window_position, window_size, changed, minimised): ([i32; 2], [u32; 2], bool, bool) = **state.window_bounds.load();

        if changed {
            window_ref.set_outer_position(PhysicalPosition::new(window_position[0], window_position[1]));
            let _ = window_ref.request_inner_size(PhysicalSize::new(window_size[0], window_size[1]));
            window_ref.set_minimized(minimised);
            state.window_bounds.store(Arc::new((window_position, window_size, false, minimised)));
        }

        let mut focused = true;

        if !state.input(&event) {
            match event {
                WindowEvent::Focused(is_focused) => {
                    focused = is_focused;
                }

                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                    state.settings.old_size = physical_size;
                }


                WindowEvent::CursorMoved { position, .. } => {
                    let size = window_ref.inner_size();

                    if !self.app_variables.cursor_locked {
                        self.app_variables.mouse_x = position.x / size.width as f64;
                        self.app_variables.mouse_y = position.y / size.height as f64;
                    }
                }
                WindowEvent::CloseRequested => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }

        let cursor_locked = state.cursor_locked.load(Ordering::Relaxed) && focused;
        state.cursor_locked.store(cursor_locked, Ordering::Relaxed);
        self.app_variables.cursor_locked = cursor_locked;
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, window_id: DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta: (x, y) } = event {
            if self.app_variables.cursor_locked {
                self.app_variables.mouse_x += x;
                self.app_variables.mouse_y += y;
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let Some(window_ref) = self.window.as_ref() else { return };
        let Some(state) = self.state.as_mut() else { return };

        if state.globals_updated.load(Ordering::Relaxed) {
            state.globals_updated.store(false, Ordering::Relaxed);

            self.app_variables.frame_count += 1;

            if self.app_variables.last_time.elapsed().as_secs_f32() >= 1.0 {
                self.app_variables.fps_atomic.store(self.app_variables.frame_count, Ordering::Relaxed);

                self.app_variables.frame_count = 0;
                self.app_variables.last_time = Instant::now();
            }


            let cursor_x = state.cursor_x.load(Ordering::Relaxed);
            let cursor_y = state.cursor_y.load(Ordering::Relaxed);

            state.cursor_x.store(cursor_x + self.app_variables.mouse_x as f32, Ordering::Relaxed);
            state.cursor_y.store(cursor_y + self.app_variables.mouse_y as f32, Ordering::Relaxed);

            self.app_variables.mouse_x = 0.0;
            self.app_variables.mouse_y = 0.0;

            if self.app_variables.cursor_locked {
                self.app_variables.sh_up_mouse_lk = true;

                let size = window_ref.inner_size();
                let center = (size.width as f32 / 2.0, size.height as f32 / 2.0);

                window_ref.set_cursor_position(PhysicalPosition::new(center.0, center.1)).ok();

                window_ref.set_cursor_visible(false);
            } else if self.app_variables.sh_up_mouse_lk {
                window_ref.set_cursor_visible(true);
                self.app_variables.sh_up_mouse_lk = false;
            }

            state.render();

            window_ref.request_redraw();
        }
    }
}
