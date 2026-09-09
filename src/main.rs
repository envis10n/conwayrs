mod cell;
mod vec2d;

use cell::CellMap;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

const FRAME_WIDTH: u32 = 128;
const FRAME_HEIGHT: u32 = 128;
const FRAME_SCALE: u32 = 4;
const RANDOM_SEED: u64 = 69;
const WINDOW_SIZE: LogicalSize<u32> =
    LogicalSize::new(FRAME_WIDTH * FRAME_SCALE, FRAME_HEIGHT * FRAME_SCALE);

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    cells: CellMap,
}

impl App {
    pub fn new() -> Self {
        let cells = CellMap::new(FRAME_WIDTH, FRAME_HEIGHT, RANDOM_SEED);
        App {
            window: None,
            pixels: None,
            cells,
        }
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
                        .with_resizable(false),
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
        )
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Goodbye!");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // TODO: Render
                self.cells.tick();
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
                std::thread::sleep(std::time::Duration::from_millis(1000 / 15));
                self.window.as_ref().unwrap().request_redraw();
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
