use std::f32::consts::PI;

use glium::uniforms::MagnifySamplerFilter::Nearest;
use glium::{
    BlitTarget, Program, Rect, Surface, Texture2d, VertexBuffer,
    backend::glutin::Display,
    glutin::surface::WindowSurface,
    index::{NoIndices, PrimitiveType},
};
use nalgebra::{Isometry3, Perspective3, Point3, Vector3};
use rand::random_range;

use crate::glue::{point_to_array, points_to_vertices};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Vertex2d {
    pub position: [f32; 2],
}
implement_vertex!(Vertex2d, position);

pub struct Render {
    display: Display<WindowSurface>,
    indices: NoIndices,
    program: Program,
    vertex_buffer: Option<VertexBuffer<Vertex>>,
    origin: Option<Point3<f32>>,
    previous_frame: Texture2d,
    target_frame: Texture2d,
    blend_program: Program,
    full_screen_quad: VertexBuffer<Vertex2d>,
}

impl Render {
    pub fn init_render(display: Display<WindowSurface>) -> Self {
        let (w, h) = display.get_framebuffer_dimensions();

        let target_frame = Texture2d::empty(&display, w, h).unwrap();
        let previous_frame = Texture2d::empty(&display, w, h).unwrap();

        let indices = NoIndices(PrimitiveType::LinesList);

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
            uniform vec2 taa_offset;
            uniform vec3 origin;

            void main() {
                vec3 pos = rotate2d(current_time/1000) * (position - origin);
                vec4 new_position = pmatrix*camera*vec4(pos, 1.0);
                // new_position.y = new_position.y - 3.;
                new_position.xy += taa_offset;
                gl_Position = new_position;
                c = exp(4.0 - new_position.z)/2;
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in float c;
            out vec4 color;

            uniform vec3 primary_c;
            uniform vec3 highlight_c;

            void main() {
                float intensity = clamp(c,0,1);
                float light = clamp((c-1),0,1); 
                color = vec4(
                    (1-light)*intensity*primary_c+light*highlight_c, 1.0
                );
            }
        "#;

        let program =
            Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let blend_program = Program::from_source(
            &display,
            r#"
            #version 140
            in vec2 position;
            out vec2 pos;

            void main() {
                pos = (position-vec2(1,1))/2;
                gl_Position = vec4(position, 0., 1.);
            }

            "#,
            r#"
            #version 140
            uniform sampler2D target_frame;
            uniform sampler2D previous_frame;

            in vec2 pos;
            out vec4 color;
            void main() {
                vec4 target_frame_col = texture(target_frame, pos);
                vec4 previous_frame_col = texture(previous_frame, pos);
                float t = 0.0625;
                color = t*target_frame_col+(1-t)*previous_frame_col-0.00390625*vec4(1,1,1,0);

            }
            "#,
            None,
        )
        .unwrap();

        let full_screen_quad_vertices = [
            Vertex2d {
                position: [1., -1.],
            },
            Vertex2d {
                position: [-1., -1.],
            },
            Vertex2d { position: [1., 1.] },
            Vertex2d {
                position: [-1., -1.],
            },
            Vertex2d { position: [1., 1.] },
            Vertex2d {
                position: [-1., 1.],
            },
        ];
        let full_screen_quad = VertexBuffer::new(&display, &full_screen_quad_vertices).unwrap();

        Render {
            display,
            indices,
            program,
            vertex_buffer: None,
            origin: None,
            previous_frame,
            target_frame,
            blend_program,
            full_screen_quad,
        }
    }

    pub fn set_points(&mut self, points: Vec<Point3<f32>>) -> &mut Self {
        let bbox_min = points
            .iter()
            .fold(Point3::origin(), |x, y| Point3::inf(&x, y));
        let bbox_max = points
            .iter()
            .fold(Point3::origin(), |x, y| Point3::sup(&x, y));
        self.origin = Some(bbox_max.lerp(&bbox_min, 0.5));

        let shape = points_to_vertices(points);
        let vertex_buffer = VertexBuffer::new(&self.display, &shape).unwrap();
        self.vertex_buffer = Some(vertex_buffer);
        self
    }

    pub fn resize_buffers(&mut self, width: u32, height: u32) -> &mut Self {
        self.target_frame = Texture2d::empty(&self.display, width, height).unwrap();
        self.previous_frame = Texture2d::empty(&self.display, width, height).unwrap();
        self
    }

    pub fn draw(&self, cam_x: f32, cam_y: f32, current_time: f32, taa: bool) -> &Self {
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let (width, height) = self.display.get_framebuffer_dimensions();
        let dx = 2. / width as f32;
        let dy = 2. / height as f32;
        let taa_offset = [
            (random_range(-dx..dx) + random_range(-dx..dx)) / 2.,
            (random_range(-dy..dy) + random_range(-dy..dy)) / 2.,
        ];

        let perspective: [[f32; 4]; 4] = {
            let aspect_ratio = height as f32 / width as f32;
            Perspective3::new(1. / aspect_ratio, PI / 6.0, 0.1, 10.0)
                .into_inner()
                .into()
        };

        let r = 4.;

        let camera: [[f32; 4]; 4] = Isometry3::look_at_rh(
            &Point3::new(
                r * cam_x.sin() * cam_y.cos(),
                r * 1. * cam_y.sin(),
                r * cam_x.cos() * cam_y.cos(),
            ),
            &Point3::origin(),
            &Vector3::new(0., 1., 0.),
        )
        .to_homogeneous()
        .into();

        // draw points
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        let vb = self.vertex_buffer.as_ref().expect("Vertex Buffer unset.");
        target
            .draw(
                vb,
                self.indices,
                &self.program,
                &uniform! {
                    // current_time: 0.0f32,
                    current_time: current_time,
                    pmatrix: perspective,
                    camera: camera,
                    primary_c: [1.,0.,0.] as [f32; _],
                    highlight_c: [0.8, 1., 0.0f32] as [f32; _],
                    taa_offset: if taa {taa_offset} else {[0., 0.]},
                    origin: point_to_array(self.origin.unwrap())
                },
                &params,
            )
            .unwrap();

        // TAA
        if taa {
            self.target_frame.as_surface().blit_from_frame(
                &Rect {
                    left: 0,
                    bottom: 0,
                    width,
                    height,
                },
                &BlitTarget {
                    left: 0,
                    bottom: 0,
                    width: width as i32,
                    height: height as i32,
                },
                Nearest,
            );

            target.clear_color(0.0, 0.0, 0.0, 1.0);
            target
                .draw(
                    &self.full_screen_quad,
                    NoIndices(PrimitiveType::TrianglesList),
                    &self.blend_program,
                    &uniform! {
                        target_frame: &self.target_frame,
                        previous_frame: &self.previous_frame
                    },
                    &Default::default(),
                )
                .unwrap();
        }

        self.previous_frame.as_surface().blit_from_frame(
            &Rect {
                left: 0,
                bottom: 0,
                width,
                height,
            },
            &BlitTarget {
                left: 0,
                bottom: 0,
                width: width as i32,
                height: height as i32,
            },
            Nearest,
        );

        target.finish().unwrap();

        self
    }
}
