mod glue;
mod lsystem;
mod render;

use crate::lsystem::get_points_bfs;
use crate::lsystem::test_actually_nice_tree;

#[macro_use]
extern crate glium;

use crate::render::Render;
use glium::winit::event::Event::WindowEvent;
use glium::winit::event::KeyEvent;
use glium::winit::event::WindowEvent as WindowEventType;
use glium::winit::keyboard::Key;
use glium::winit::keyboard::NamedKey;

const RDEPTH: i32 = 11;

fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #2")
        .with_inner_size(1920, 1080)
        .build(&event_loop);

    let points = get_points_bfs(&test_actually_nice_tree(), RDEPTH);
    let render = Render::init_render(display);

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| match ev {
            WindowEvent { event, .. } => match event {
                WindowEventType::CloseRequested => {
                    window_target.exit();
                }
                WindowEventType::KeyboardInput {
                    event: KeyEvent { logical_key, .. },
                    ..
                } => match logical_key {
                    Key::Named(NamedKey::Escape) => window_target.exit(),
                    Key::Character(s) => match s.as_str() {
                        "w" => cam_y += 0.1,
                        "a" => cam_x -= 0.1,
                        "s" => cam_y -= 0.1,
                        "d" => cam_x += 0.1,
                        _ => (),
                    },
                    _ => (),
                },
                WindowEventType::RedrawRequested => {
                    render.draw(points.clone(), cam_x, cam_y);
                    window.request_redraw();
                }
                _ => (),
            },
            _ => (),
        })
        .unwrap();
}
