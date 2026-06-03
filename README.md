# R-CPUShader
Minimal CPU shader multithreaded renderer written in Rust.</br>
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
<img width="630" height="665" alt="Desktop 2026-06-03 8-34-43 AM-767" src="https://github.com/user-attachments/assets/aefdbfde-6c20-4253-9b6d-e103f1b85210" />

<img width="626" height="667" alt="Desktop 2026-06-03 8-37-29 AM-886" src="https://github.com/user-attachments/assets/3a812c9c-008b-404d-ad95-737533163fdd" />

<img width="631" height="669" alt="Desktop 2026-06-03 8-38-21 AM-391" src="https://github.com/user-attachments/assets/d5aadb7e-f2fc-4221-9801-a60933937d54" />
