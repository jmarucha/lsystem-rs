mod glue;
mod lsystem;
mod render;

use std::time::SystemTime;

use crate::lsystem::TreeType;
use crate::lsystem::generate_tree;
use crate::lsystem::get_points_dfs;

#[macro_use]
extern crate glium;

use crate::render::Render;
use glium::winit::event::ElementState;
use glium::winit::event::Event::WindowEvent;
use glium::winit::event::KeyEvent;
use glium::winit::event::WindowEvent as WindowEventType;
use glium::winit::keyboard::Key;
use glium::winit::keyboard::NamedKey;

#[cfg(debug_assertions)]
const RDEPTH: i32 = 10;

#[cfg(not(debug_assertions))]
const RDEPTH: i32 = 13;

fn main() {
    let now = SystemTime::now();

    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Trees")
        .with_inner_size(1920, 1080)
        .build(&event_loop);
    // let points = get_points_dfs(&test_actually_nice_tree(), RDEPTH);
    let mut render = Render::init_render(display);
    {
        let points = get_points_dfs(&generate_tree(TreeType::RandomTree), RDEPTH);
        render.set_points(points);
    }

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;
    let mut taa = false;
    let mut rotation = true;

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| if let WindowEvent { event, .. } = ev { match event {
            WindowEventType::CloseRequested => {
                window_target.exit();
            }
            WindowEventType::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match logical_key {
                Key::Named(NamedKey::Escape) => window_target.exit(),
                Key::Named(NamedKey::Space) => rotation = !rotation,
                Key::Character(s) => match s.as_str() {
                    "w" => cam_y += 0.1,
                    "a" => cam_x -= 0.1,
                    "s" => cam_y -= 0.1,
                    "d" => cam_x += 0.1,
                    "q" => taa = !taa,
                    "n" => {
                        let points =
                            get_points_dfs(&generate_tree(TreeType::RandomTree), RDEPTH);
                        render.set_points(points);
                    }
                    _ => (),
                },
                _ => (),
            },
            WindowEventType::RedrawRequested => {
                let current_time = now.elapsed().unwrap_or_default().as_millis() as f32;
                render.draw(cam_x, cam_y, if rotation { current_time } else { 0. }, taa);
                window.request_redraw();
            }
            _ => (),
        } })
        .unwrap();
}
