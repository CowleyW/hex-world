mod application;
mod renderer;
mod world;

use crate::application::Application;

fn main() {
    env_logger::init();

    let app = Application::new();
    app.run();
}
