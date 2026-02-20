
use glium::{Program, Surface, backend::glutin::Display, glutin::surface::WindowSurface, index::NoIndices};
use nalgebra::Point3;


#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32;2]
}
implement_vertex!(Vertex, position);

pub struct Render {
    display: Display<WindowSurface>,
    indices: NoIndices,
    program: Program,
}

impl Render {
    pub fn init_render(display: Display<WindowSurface>) -> Self {
        //let perspective = na::Perspective3::new(4./3., PI/3., -1., 1.).into_inner();
        //let perspective_raw: [[f32;4]; 4] = perspective.into();
        let perspective_raw = [
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];


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
        Render {display, indices, program}
    }

    pub fn draw(self: &Self, points: Vec<Point3<f32>>) -> () {
        // convert to vertex buffer
        let shape_iter = points.into_iter().map(|point: Point3<_>| -> Vertex {Vertex { position: [point[0],point[1]] }});
        let shape = Vec::from_iter(shape_iter);
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();

        // draw
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &self.indices, &self.program, &uniform!{scale: 2.0f32},
            &Default::default()).unwrap();
        target.finish().unwrap();

    }

}