mod cell;
mod vec2d;

use cell::CellMap;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowButtons, WindowId};

/// The simulation / base width.
const FRAME_WIDTH: u32 = 128;
/// The simulation / base height.
const FRAME_HEIGHT: u32 = 128;
/// The scale to apply to the width / height to get the final window size.
const FRAME_SCALE: u32 = 4;
/// The simulation rate, in hertz (fps).
const SIM_RATE: u32 = 20;
/// The random seed to use for the initial state.
///
/// Keeping this the same value will deterministically set the initial cell states.
const RANDOM_SEED: u64 = 1337;
/// Window Logical Size for use with DPI scaling.
const WINDOW_SIZE: LogicalSize<u32> =
    LogicalSize::new(FRAME_WIDTH * FRAME_SCALE, FRAME_HEIGHT * FRAME_SCALE);

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    cells: CellMap,
    should_tick: bool,
    delta: f32,
    last_frame: f64,
    sim_rate: f32,
    sim_timer: f64,
}

/// Get the time since epoch in milliseconds.
fn epoch_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
        * 1000f64
}

impl App {
    pub fn new() -> Self {
        let cells = CellMap::new(FRAME_WIDTH, FRAME_HEIGHT, RANDOM_SEED);
        let now = epoch_ms();
        let sim_rate = 1000f32 / SIM_RATE as f32;
        App {
            window: None,
            pixels: None,
            cells,
            should_tick: false,
            delta: 0f32,
            last_frame: now,
            sim_rate,
            sim_timer: sim_rate as f64,
        }
    }
    /// Render the simulation state to the SurfaceTexture.
    fn render_sim(&mut self) {
        let pixels = self.pixels.as_mut().unwrap();
        let buffer = pixels.frame_mut();
        let cbuf = self.cells.to_slice();
        for i in 0..self.cells.count() {
            let idx = i * 4;
            if cbuf[i] {
                // Alive
                buffer[idx] = 0;
                buffer[idx + 1] = 0;
                buffer[idx + 2] = 0;
                buffer[idx + 3] = 0xff;
            } else {
                // Dead
                buffer[idx] = 0xff;
                buffer[idx + 1] = 0xff;
                buffer[idx + 2] = 0xff;
                buffer[idx + 3] = 0xff;
            }
        }
        pixels.render().unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("ConwayRS")
                        .with_min_inner_size(WINDOW_SIZE)
                        .with_inner_size(WINDOW_SIZE)
                        .with_resizable(false)
                        .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE),
                )
                .unwrap(),
        );
        self.window = Some(window.clone());
        self.pixels = Some(
            PixelsBuilder::new(
                FRAME_WIDTH,
                FRAME_HEIGHT,
                SurfaceTexture::new(WINDOW_SIZE.width, WINDOW_SIZE.height, window.clone()),
            )
            .build()
            .unwrap(),
        );
        self.render_sim();
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Goodbye!");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let now = epoch_ms();
                // Set the delta since the last frame.
                self.delta = (now - self.last_frame) as f32;
                self.last_frame = now;

                if self.should_tick {
                    // Use the sim timer field as a countdown, subtracting the frame delta.
                    self.sim_timer -= self.delta as f64;
                    if self.sim_timer <= 0f64 {
                        // When we hit zero, run the simulation tick and set the timer back to the rate.
                        self.sim_timer = self.sim_rate as f64;
                        self.cells.tick();
                        self.render_sim();
                    }
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                // Handle Spacebar
                if let Key::Named(NamedKey::Space) = event.logical_key
                    && event.state == ElementState::Released
                {
                    // Start or stop the simulation.
                    self.should_tick = !self.should_tick;
                }
                // Handle Backspace
                else if let Key::Named(NamedKey::Backspace) = event.logical_key
                    && event.state == ElementState::Released
                {
                    // Reset the cell states using the random seed.
                    // Note: this pauses the simulation.
                    self.should_tick = false;
                    self.cells.reset();
                    self.render_sim();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
