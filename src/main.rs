mod glue;
mod lsystem;
mod render;

use crate::lsystem::get_points_bfs;
use crate::lsystem::test_actually_nice_tree;

#[macro_use]
extern crate glium;

use crate::render::Render;

const RDEPTH: i32 = 11;

fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .unwrap();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #2")
        .with_inner_size(1920, 1080)
        .build(&event_loop);

    let points = get_points_bfs(&test_actually_nice_tree(), RDEPTH);
    let render = Render::init_render(display);

    render.draw(points);

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                _ => (),
            },
            _ => (),
        })
        .unwrap();
}
