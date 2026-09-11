mod cell;
mod vec2d;

use cell::CellMap;
use clap::Parser;
use fastrand::Rng;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowButtons, WindowId};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Simulation bounds width.
    #[arg(short, long, default_value_t = 128)]
    width: u32,
    /// Simulation bounds height.
    #[arg(short = 'H', long, default_value_t = 128)]
    height: u32,
    /// Simulation scale (window size).
    #[arg(short, long, default_value_t = 4)]
    scale: u32,
    /// Simulation rate in hertz.
    #[arg(short, long, default_value_t = 20)]
    rate: u32,
    /// Seed to use for RNG. Omitting this will use a random seed.
    #[arg(short = 'S', long)]
    seed: Option<u64>,
}

struct App {
    args: Cli,
    window_size: LogicalSize<u32>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    cells: CellMap,
    should_tick: bool,
    delta: f32,
    last_frame: f64,
    sim_rate: f32,
    sim_timer: f64,
    window_title: String,
    sim_seed: u64,
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
    pub fn new(args: Cli) -> Self {
        let sim_seed: u64 = if let Some(seed) = args.seed {
            seed
        } else {
            Rng::new().get_seed()
        };
        let cells = CellMap::new(args.width, args.height, sim_seed);
        let window_size = LogicalSize::new(args.width * args.scale, args.height * args.scale);
        let now = epoch_ms();
        let sim_rate = 1000f32 / args.rate as f32;
        App {
            args,
            window_size,
            window: None,
            pixels: None,
            cells,
            should_tick: false,
            delta: 0f32,
            last_frame: now,
            sim_rate,
            sim_timer: sim_rate as f64,
            window_title: format!("ConwayRS | Seed: {} (Paused)", sim_seed),
            sim_seed,
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
                        .with_title(&self.window_title)
                        .with_min_inner_size(self.window_size)
                        .with_inner_size(self.window_size)
                        .with_resizable(false)
                        .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE),
                )
                .unwrap(),
        );
        self.window = Some(window.clone());
        self.pixels = Some(
            PixelsBuilder::new(
                self.args.width,
                self.args.height,
                SurfaceTexture::new(
                    self.window_size.width,
                    self.window_size.height,
                    window.clone(),
                ),
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
                    let state = if self.should_tick { "" } else { " (Paused)" };
                    self.window
                        .as_ref()
                        .unwrap()
                        .set_title(&format!("ConwayRS | Seed: {}{}", self.sim_seed, state));
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
                    self.window
                        .as_ref()
                        .unwrap()
                        .set_title(&format!("ConwayRS | Seed: {} (Paused)", self.sim_seed));
                }
                // Handle Esc
                else if let Key::Named(NamedKey::Escape) = event.logical_key
                    && event.state == ElementState::Released
                {
                    // Exit
                    println!("Goodbye!");
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::new(cli);
    event_loop.run_app(&mut app).unwrap();
}
