use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use arc_swap::{ArcSwap};
use atomic_float::{AtomicF32, AtomicF64};
use tokio::time::Instant;
use winit::keyboard::{Key, NamedKey};
use crate::engine::back::key_handler::InputHandler;
use crate::engine::back::render::render::Render;
use crate::engine::back::staging_buffers::StagingBuffers;
use crate::engine::front::components::camera::Camera;
use crate::src::game::Game;

pub async fn run(run_var: RunVar) {
    let time_per_update = run_var.time_per_update;
    let staging_buffers_arc = run_var.staging_buffers.clone();
    let globals_updated = run_var.globals_updated.clone(); // tells state to render
    let ticks_update = run_var.ticks_updated.clone(); // tells state to update buffers
    let del_timer = run_var.del_timer.clone();

    let staging_buffers_cloned = (**staging_buffers_arc.load()).clone();
    let render = Render::new(staging_buffers_cloned);

    let mut game = Game::new(render);

    let mut update_var = UpdateVar { // variable that game.update() uses
        cursor_x: run_var.cursor_x.clone(),
        cursor_y: run_var.cursor_y.clone(),
        cursor_locked: run_var.cursor_locked.clone(),
        window_bounds: (**ArcSwap::<([i32; 2], [u32; 2], bool, bool)>::load(&run_var.window_bounds)).clone(),
        window_bounds_arc_swap: run_var.window_bounds.clone(),

        fps: run_var.fps.load(Ordering::Relaxed),
        fps_atomic: run_var.fps.clone(),

        tps: 0,
        tick_count: 0,

        key_handler: (**ArcSwap::<InputHandler>::load(&run_var.key_handler)).clone(),
        f11: false,

        key_handler_arc_swap: run_var.key_handler.clone(),
    };

    let mut prev_time = Instant::now();

    let mut last_time = Instant::now();
    let mut ticks_count = 0;
    let mut tps = 0;
    let mut t = 0.0;

    loop {
        let cr_time = Instant::now();
        let elp_time = (cr_time - prev_time).as_secs_f64();
        prev_time = cr_time;
        t += elp_time;

        while t >= time_per_update {
            ticks_count += 1;

            { // update variables for game.update();

                update_var.cursor_x = run_var.cursor_x.clone();
                update_var.cursor_y = run_var.cursor_y.clone();
                update_var.window_bounds = (**ArcSwap::<([i32; 2], [u32; 2], bool, bool)>::load(&update_var.window_bounds_arc_swap)).clone();
                update_var.key_handler = (**ArcSwap::<InputHandler>::load(&run_var.key_handler)).clone();
                update_var.fps = update_var.fps_atomic.load(Ordering::Relaxed);
                update_var.tps = tps;
                update_var.tick_count = ticks_count;
            }

            { // call game.update();
                game.update(&mut update_var);

                update_var.key_handler_arc_swap.store(Arc::new(update_var.key_handler.clone()));
                update_var.window_bounds_arc_swap.store(Arc::new(update_var.window_bounds.clone()));
            }

            { // update buffers
                let render = game.get_render();
                render.setup_staging_buffers();
                let staging_buffers = render.get_staging_buffers();

                if let Some(staging_buffers) = staging_buffers {
                    staging_buffers_arc.store(Arc::new(staging_buffers));
                    render.setup_offsets();
                }

                render.update_camera();
                ticks_update.store(true, Ordering::Relaxed); // tells state to updated buffers
            }

            t -= time_per_update;
        }

        del_timer.store(t, Ordering::Relaxed);
        globals_updated.store(true, Ordering::Relaxed); // tells state to render

        if last_time.elapsed().as_secs_f64() > 1.0 {
            //println!("TPS: {}", ticks_count.load(Ordering::Relaxed));
            tps = ticks_count;
            ticks_count = 0;
            last_time = Instant::now();
        }
    }
}

pub struct RunVar {
    pub time_per_update: f64,
    pub staging_buffers: Arc<ArcSwap<StagingBuffers>>,
    pub key_handler: Arc<ArcSwap<InputHandler>>,
    
    pub cursor_x: Arc<AtomicF32>,
    pub cursor_y: Arc<AtomicF32>,
    pub cursor_locked: Arc<AtomicBool>,
    pub window_bounds: Arc<ArcSwap<([i32; 2], [u32; 2], bool, bool)>>,
    
    pub globals_updated: Arc<AtomicBool>,
    pub ticks_updated: Arc<AtomicBool>,
    pub fps: Arc<AtomicU32>,
    pub del_timer: Arc<AtomicF64>,
}

pub struct UpdateVar {
    pub cursor_x: Arc<AtomicF32>,
    pub cursor_y: Arc<AtomicF32>,
    pub cursor_locked: Arc<AtomicBool>,
    pub window_bounds: ([i32; 2], [u32; 2], bool, bool),
    pub window_bounds_arc_swap:  Arc<ArcSwap<([i32; 2], [u32; 2], bool, bool)>>,

    pub fps: u32,
    pub fps_atomic: Arc<AtomicU32>,

    pub tps: u32,
    pub tick_count: u32,

    pub key_handler: InputHandler,
    pub f11: bool,

    pub key_handler_arc_swap: Arc<ArcSwap<InputHandler>>,
}

#[allow(dead_code)]
impl UpdateVar {
    pub fn fullscreen(&mut self) {
        let lock_mouse = self.cursor_locked.load(Ordering::Relaxed);

        let fullscreen = if self.key_handler.pressed_keys.contains(&Key::Named(NamedKey::F11)) && !self.f11{
            self.f11 = true;

            !lock_mouse
        } else if !self.key_handler.pressed_keys.contains(&Key::Named(NamedKey::F11)) {
            self.f11 = false;
            lock_mouse
        } else {
            lock_mouse
        };

        if fullscreen {
            self.cursor_locked.store(true, Ordering::Relaxed);
        } else {
            self.cursor_locked.store(false, Ordering::Relaxed);
        }
    }

    pub fn is_fullscreen(&self) -> bool {
        self.cursor_locked.load(Ordering::Relaxed)
    }

    pub fn add_camera_to_cursor(&self, camera: &mut Camera, dpi: f32) {
        if self.is_fullscreen() {
            let dpi = dpi / 1000.0;

            let cursor_y = self.cursor_y.load(Ordering::Relaxed);
            let cursor_x = self.cursor_x.load(Ordering::Relaxed);

            self.cursor_y.store(0.0, Ordering::Relaxed);
            self.cursor_x.store(0.0, Ordering::Relaxed);

            let y = -cursor_y * dpi;
            let x = -cursor_x * dpi;

            let rot: [f32; 2] = [y, x];
            camera.change_rotation(&rot, &true);
        }
    }

    pub fn set_window_x(&mut self, x: i32) {
        self.window_bounds.0[0] = x;
        self.window_bounds.2 = true;
    }

    pub fn set_window_y(&mut self, y: i32) {
        self.window_bounds.0[1] = y;
        self.window_bounds.2 = true;
    }

    pub fn set_window_width(&mut self, width: u32) {
        self.window_bounds.1[0] = width;
        self.window_bounds.2 = true;
    }

    pub fn set_minimised(&mut self, state: bool) {
        self.window_bounds.3 = state;
        self.window_bounds.2 = true;
    }

    pub fn set_window_height(&mut self, height: u32) {
        self.window_bounds.1[1] = height;
        self.window_bounds.2 = true;
    }

    pub fn get_window_bounds(&self) -> ([i32; 2], [u32; 2]) {
        (self.window_bounds.0.clone(), self.window_bounds.1.clone())
    }
}