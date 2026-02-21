use glium::{
    Program, Surface, VertexBuffer, backend::glutin::Display, glutin::surface::WindowSurface, index::NoIndices
};
use nalgebra::Point3;

use crate::glue::points_to_vertices;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);

pub struct Render {
    display: Display<WindowSurface>,
    indices: NoIndices,
    program: Program,
    vertex_buffer: Option<VertexBuffer<Vertex>>
}

impl Render {
    pub fn init_render(display: Display<WindowSurface>) -> Self {
        //let perspective = na::Perspective3::new(4./3., PI/3., -1., 1.).into_inner();
        //let perspective_raw: [[f32;4]; 4] = perspective.into();
        // let perspective_raw = [
        //     [0.1, 0.0, 0.0, 0.0],
        //     [0.0, 0.1, 0.0, 0.0],
        //     [0.0, 0.0, 0.1, 0.0],
        //     [0.0, 0.0, 2.0, 1.0f32]
        // ];

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
        //let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let vertex_shader_src = r#"
            #version 140
            #define SCALE 2.0
            #define AR 4.0/3.0

            in vec3 position;
            // uniform mat4 perspective;
            uniform float scale;
            uniform vec2 cam;
            uniform float current_time;

            out float c;
            
            mat3 rotate2d(float _angle){
                return mat3(
                    cos(_angle), 0 , -sin(_angle),
                    0, 1, 0,
                    sin(_angle), 0,   cos(_angle)
                    );
                }

            void main() {
                // gl_Position = vec4((position.x)/scale, position.y*AR/scale-0.6, 0.0, 1.0);
                vec3 new_position = rotate2d(current_time/1000) * position;
                gl_Position = vec4((new_position.x+cam.x)/scale-0.3, (new_position.y+cam.y)*AR/scale, new_position.z/10., 1.0);
                c = exp(-new_position.z-0.5);

                //gl_Position = perspective*vec4(position.x, position.y, position.z, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in float c;
            out vec4 color;
            // uniform vec3 csetting;

            void main() {
                float intensity = clamp(c,0,1);
                float light = clamp(c-1,0,1); 
                color = vec4(intensity, light, 0., 1.0);
            }
        "#;

        let program =
            glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        Render {
            display,
            indices,
            program,
            vertex_buffer: None
        }
    }

    pub fn set_vertex_buffer(self: &mut Self, points: Vec<Point3<f32>>) -> () {
        let shape = points_to_vertices(points);
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
        self.vertex_buffer = Some(vertex_buffer);
    }

    pub fn draw(self: &Self, cam_x: f32, cam_y: f32, current_time: f32) -> () {

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // draw
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        let vb = self.vertex_buffer.as_ref().unwrap();
        target
            .draw(
                vb,
                &self.indices,
                &self.program,
                &uniform! {scale: 2.0f32, cam: [cam_x, cam_y], current_time: current_time},
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    }
}
