# R-CPUShader
Minimal CPU shader multithreaded renderer written in Rust.
I just felt like trying Rust.

## Code
```rust
pub mod common;

mod app;
mod shaders;

use crate::shaders::ShaderType;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let app = &mut app::App::new(
        500,//width
        500,//height
        true,//temporal accumulation
        shaders::get(ShaderType::GlassDisks).unwrap()//shader
        //Available: ShaderType::SphereTracing, ShaderType::VolumetricCube
    );

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(app).unwrap();
}
```

## Screenshots
