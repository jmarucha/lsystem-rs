use glium::{
    Program, Surface, VertexBuffer, backend::glutin::Display, glutin::surface::WindowSurface,
    index::NoIndices,
};
use nalgebra::{Isometry3, Perspective3, Point3, Vector3};

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
    vertex_buffer: Option<VertexBuffer<Vertex>>,
}

impl Render {
    pub fn init_render(display: Display<WindowSurface>) -> Self {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
        //let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            uniform float current_time;

            out float c;
            
            mat3 rotate2d(float _angle){
                return mat3(
                    cos(_angle), 0 , +sin(_angle),
                    0, 1, 0,
                    -sin(_angle), 0,   cos(_angle)
                    );
                }
            uniform mat4 pmatrix;
            uniform mat4 camera;

            void main() {
                vec4 new_position = pmatrix*camera*vec4(rotate2d(current_time/1000) * position, 1.0);
                new_position.y = new_position.y - 3.;
                gl_Position = new_position;
                c = exp(4.0 - new_position.z)/2;
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in float c;
            out vec4 color;
            // uniform vec3 csetting;

            void main() {
                float intensity = clamp(c,0,1);
                float light = clamp((c-1),0,1); 
                color = vec4(light, intensity, 0., 1.0);
            }
        "#;

        let program =
            glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        Render {
            display,
            indices,
            program,
            vertex_buffer: None,
        }
    }

    pub fn set_vertex_buffer(self: &mut Self, points: Vec<Point3<f32>>) -> () {
        let shape = points_to_vertices(points);
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
        self.vertex_buffer = Some(vertex_buffer);
    }

    pub fn draw(self: &Self, _cam_x: f32, cam_y: f32, current_time: f32) -> () {
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
        let perspective: [[f32; 4]; 4] = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
            Perspective3::new(1. / aspect_ratio, 3.141 / 6.0, 0.1, 10.0)
                .into_inner()
                .into()
        };

        let r = 5.;

        let camera: [[f32; 4]; 4] = Isometry3::look_at_rh(
            &Point3::new(0., r * cam_y.sin(), r * cam_y.cos()),
            &Point3::origin(),
            &Vector3::new(0., 1., 0.),
        )
        .to_homogeneous()
        .into();
        println!("{:?}", camera);

        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        let vb = self.vertex_buffer.as_ref().expect("Vertex Buffer unset.");
        target
            .draw(
                vb,
                &self.indices,
                &self.program,
                &uniform! {
                    current_time: current_time,
                    pmatrix: perspective,
                    camera: camera
                },
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    }
}
