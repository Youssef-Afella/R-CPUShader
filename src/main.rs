pub mod common;

mod app;
mod shaders;

use crate::shaders::ShaderType;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let app = &mut app::App::new(
        500, 
        500,
        true,
        shaders::get(ShaderType::GlassDisks).unwrap()
    );

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(app).unwrap();
}