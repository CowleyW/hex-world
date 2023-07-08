use winit::event::WindowEvent;
use crate::world::camera_controller::CameraController;

pub(crate) mod camera_controller;

pub struct World {
    pub camera_controller: CameraController
}

impl World {
    pub fn new() -> Self {
        let camera_controller = CameraController::new(0.2);

        Self { camera_controller }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event)
    }

    pub fn update(&mut self) {

    }
}