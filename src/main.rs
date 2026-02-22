mod glue;
mod lsystem;
mod render;

use std::time::SystemTime;

use crate::lsystem::get_points_dfs;
use crate::lsystem::test_actually_nice_tree;

#[macro_use]
extern crate glium;

use crate::render::Render;
use glium::winit::event::ElementState;
use glium::winit::event::Event::WindowEvent;
use glium::winit::event::KeyEvent;
use glium::winit::event::WindowEvent as WindowEventType;
use glium::winit::keyboard::Key;
use glium::winit::keyboard::NamedKey;

const RDEPTH: i32 = 14;

fn main() {
    let now = SystemTime::now();

    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Trees")
        .with_inner_size(1920, 1080)
        .build(&event_loop);

    let points = get_points_dfs(&test_actually_nice_tree(), RDEPTH);
    let mut render = Render::init_render(display);

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;
    let mut taa = true;

    render.set_vertex_buffer(points);

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| match ev {
            WindowEvent { event, .. } => match event {
                WindowEventType::CloseRequested => {
                    window_target.exit();
                }
                WindowEventType::KeyboardInput {
                    event: KeyEvent { logical_key, state: ElementState::Pressed, .. },
                    ..
                } => match logical_key {
                    Key::Named(NamedKey::Escape) => window_target.exit(),
                    Key::Character(s) => match s.as_str() {
                        "w" => cam_y += 0.1,
                        "a" => cam_x -= 0.1,
                        "s" => cam_y -= 0.1,
                        "d" => cam_x += 0.1,
                        "q" => taa = !taa,
                        _ => (),
                    },
                    _ => (),
                },
                WindowEventType::RedrawRequested => {
                    let current_time = now.elapsed().unwrap_or_default().as_millis() as f32;
                    render.draw(cam_x, cam_y, current_time, taa);
                    window.request_redraw();
                }
                _ => (),
            },
            _ => (),
        })
        .unwrap();
}
