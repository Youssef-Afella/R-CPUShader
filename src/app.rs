use crate::common::*;

use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Instant};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use rayon::prelude::*;

pub struct App {
    width: u32,
    height: u32,
    shader: ShaderFn,
    app_start: Instant,
    app_last: f32,
    app_fps_timer: f32,
    frame_count: u32,
    frame_buffer: Vec<Vec4>,
    temporal_accumulation: bool,
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
}

impl App {

    pub fn new(width: u32, height: u32, temporal_accumulation: bool, shader: ShaderFn) -> Self {

        let total_pixels = (width * height) as usize;
        let frame_buffer = vec![Vec4::ZERO; total_pixels];

        Self {
            width,
            height,
            shader,
            app_start: Instant::now(),
            app_last: 0.0,
            app_fps_timer: 0.0,
            frame_buffer,
            frame_count: 0,
            window: None,
            surface: None,
            temporal_accumulation
        }
    }

    
    fn draw(&mut self) {
        let Some(surface) = self.surface.as_mut() else { return; };
        let Some(window) = self.window.as_ref() else { return; };
        
        let time = self.app_start.elapsed().as_secs_f32();

        if time - self.app_fps_timer > 0.1 {
            let sec = time - self.app_last;
            let fps = 1.0 / sec;
            let title = format!("Render: {:.1}fps, {:.1}ms | Elapsed : {:.1}s, {}frames", fps, sec * 1000.0, time, self.frame_count);
            window.set_title(&title);

            self.app_fps_timer = time;
        }

        self.app_last = time;

        let size = window.inner_size();
        if size.width == 0 || size.height == 0 { return; }

        if size.width != self.width || size.height != self.height {
            self.width = size.width;
            self.height = size.height;
            
            let new_total_pixels = (self.width * self.height) as usize;

            self.frame_buffer.resize(new_total_pixels, Vec4::ZERO);
            self.app_start = Instant::now();
            self.app_last = 0.0;
            self.app_fps_timer = 0.0;
            self.frame_count = 0;
            
            surface
                .resize(
                    NonZeroU32::new(self.width).unwrap(),
                    NonZeroU32::new(self.height).unwrap(),
                )
                .unwrap();
        }

        let mut buffer = surface.buffer_mut().unwrap();
        let resolution = glam::vec2(self.width as f32, self.height as f32);
        let width = self.width as usize;
        
        self.frame_count += 1;

        let alpha = 1.0 / self.frame_count as f32;

        self.frame_buffer
            .par_chunks_exact_mut(width)
            .zip(buffer.par_chunks_exact_mut(width))
            .enumerate()
            .for_each(|(y, (frame_row, buffer_row))| {
                let y_f32 = self.height as f32 - y as f32;

                for x in 0..width {
                    let frag_coord = glam::vec2(x as f32, y_f32);

                    let color = (self.shader)(frag_coord, resolution, time, self.frame_count);
                    
                    if self.temporal_accumulation {
                        frame_row[x] = frame_row[x].lerp(color, alpha);
                    }
                    else{
                        frame_row[x] = color;
                    }
                    
                    buffer_row[x] = color_to_u32(tonemap(frame_row[x]));
                }
            });

        buffer.present().unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("")
                        .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height)),
                )
                .unwrap(),
        );
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.draw();
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}
