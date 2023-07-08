use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::renderer::Renderer;
use crate::world::World;

pub struct Application {
    event_loop: EventLoop<()>,
    renderer: Renderer,
    world: World,
    prev_frame: Instant,
    frame_counter: u32
}

impl Application {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("Hex World").with_inner_size(LogicalSize::new(1920, 1080)).build(&event_loop).unwrap();

        let world = World::new();

        let renderer = pollster::block_on(Renderer::new(window));

        let prev_frame = Instant::now();
        let frame_counter = 0;

        Self {
            event_loop,
            renderer,
            world,
            prev_frame,
            frame_counter
        }
    }

    pub fn run(mut self) -> ! {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id
                } if window_id == self.renderer.window().id() => if !self.world.handle_event(event) {
                    match event {
                        WindowEvent::CloseRequested => control_flow.set_exit(),
                        WindowEvent::Resized(size) => {
                            self.renderer.resize(*size);
                        }
                        WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {
                            self.renderer.resize(**new_inner_size);
                        },
                        _ => ()
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.renderer.window().id() => {
                    match self.renderer.render() {
                        Ok(_) => {},
                        Err(wgpu::SurfaceError::Lost) => self.renderer.recreate(),
                        Err(wgpu::SurfaceError::OutOfMemory) => control_flow.set_exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    self.frame_counter += 1;
                    if self.frame_counter >= 100 {
                        let framerate = 100.0 / self.prev_frame.elapsed().as_secs_f32();
                        self.renderer.set_window_title(format!("{} FPS", framerate as u32).as_str());

                        self.prev_frame = Instant::now();
                        self.frame_counter = 0;
                    }

                    self.world.update();
                    self.renderer.update(&self.world);

                    self.renderer.window().request_redraw();
                }
                _ => (),
            }
        })
    }
}