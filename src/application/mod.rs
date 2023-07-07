mod context;
mod texture;
mod camera;
mod instance;

use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use crate::application::context::Context;

pub struct Application {
    event_loop: EventLoop<()>,
    context: Context
}

impl Application {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("Hex World").with_inner_size(LogicalSize::new(1920, 1080)).build(&event_loop).unwrap();

        let context = pollster::block_on(Context::new(window));

        Self {
            event_loop,
            context
        }
    }

    pub fn run(mut self) -> ! {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id
                } if window_id == self.context.window().id() => if !self.context.input(event) {
                    match event {
                        WindowEvent::CloseRequested => control_flow.set_exit(),
                        WindowEvent::Resized(size) => {
                            self.context.resize(*size);
                        }
                        WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {
                            self.context.resize(**new_inner_size);
                        },
                        _ => ()
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.context.window().id() => {
                    self.context.update();
                    match self.context.render() {
                        Ok(_) => {},
                        Err(wgpu::SurfaceError::Lost) => self.context.resize(self.context.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => control_flow.set_exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    self.context.window().request_redraw();
                }
                _ => (),
            }
        })
    }
}