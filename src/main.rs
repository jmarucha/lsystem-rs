mod lsystem;
mod lsystem_test;
mod render;

use std::f32::consts::PI;

use crate::lsystem::get_points_bfs;
use crate::lsystem_test::test_tree_that_sucks;

use crate::lsystem_test::test_bfs_that_sucks;
use crate::lsystem::test_actually_nice_tree;

#[macro_use]
extern crate glium;
use glium::Surface;
use nalgebra::Perspective3;
use nalgebra as na;
use nalgebra::Point3;

use crate::render::Render;

const RDEPTH: i32 = 10;

fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #2")
        .build(&event_loop);

    let points = get_points_bfs(&test_actually_nice_tree(), RDEPTH);
    let render = Render::init_render(display);
    render.draw(points);
    

    #[allow(deprecated)]
    event_loop.run(move |ev, window_target| {
        match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                _ => (),
            },
            _ => (),
        }
    })
    .unwrap();
}