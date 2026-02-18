mod lsystem;
mod lsystem_test;

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

    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position: [f32;2]
    }
    implement_vertex!(Vertex, position);

    //let perspective = na::Perspective3::new(4./3., PI/3., -1., 1.).into_inner();
    //let perspective_raw: [[f32;4]; 4] = perspective.into();
    let perspective_raw = [
        [0.1, 0.0, 0.0, 0.0],
        [0.0, 0.1, 0.0, 0.0],
        [0.0, 0.0, 0.1, 0.0],
        [0.0, 0.0, 2.0, 1.0f32]
    ];

    let shape_iter = get_points_bfs(&test_actually_nice_tree(), RDEPTH).into_iter().map(|point: Point3<_>| -> Vertex {Vertex { position: [point[0],point[1]] }});
    let shape = Vec::from_iter(shape_iter);
;
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
    //let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        // uniform mat4 perspective;
        uniform float scale;

        #define SCALE 2.0
        #define AR 4.0/3.0

        void main() {
            // gl_Position = vec4(position.x/SCALE, position.y*AR/SCALE-0.6, 0.0, 1.0);
            gl_Position = vec4(position.x/scale, position.y*AR/scale-0.6, 0.0, 1.0);

            //gl_Position = perspective*vec4(position.x, position.y, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;
        // uniform vec3 csetting;

        void main() {
            color = vec4(1.0, 0., 0., 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &uniform!{scale: 2.0f32},
        &Default::default()).unwrap();
    target.finish().unwrap();

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