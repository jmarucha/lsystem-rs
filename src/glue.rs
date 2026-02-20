use nalgebra::{Point3};
use crate::render::Vertex;
pub fn point_to_array(point: Point3<f32>) -> [f32; 3] {
    point.into()
}

fn point_to_vertex(point: Point3<f32>) -> Vertex {
    Vertex { position: point_to_array(point) }
}

pub fn points_to_vertices(points: Vec<Point3<f32>>) -> Vec<Vertex> {
    let shape_iter = points.into_iter().map(point_to_vertex);
    Vec::from_iter(shape_iter)
}