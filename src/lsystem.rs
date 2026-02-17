// rotate - move - scale - rotate

extern crate nalgebra as na;
use core::fmt;
use std::collections::VecDeque;
use std::f64::consts::{PI, SQRT_2};
use std::fmt::Debug;

use na::{Affine3, Point3, Rotation3, Scale3, Translation3, Vector3};

// todo - generalize type
pub fn get_transformation_matrix(
    rot: &Rotation3<f64>,
    trans: &Translation3<f64>,
    scale: &Scale3<f64>,
) -> Affine3<f64> {
    let rot_c: Affine3<f64> = na::convert_ref(rot);
    let trans_c: Affine3<f64> = na::convert_ref(trans);
    let scale_c: Affine3<f64> = na::convert_ref(scale);
    rot_c * trans_c * scale_c
}

pub fn get_2d_transformation(rot: f64, scale: f64) -> Affine3<f64> {
    get_transformation_matrix(
        &Rotation3::from_axis_angle(&Vector3::z_axis(), rot),
        &Translation3::new(1., 0., 0.),
        &Scale3::new(scale, scale, scale),
    )
}

pub fn test_tree() -> Affine3<f64> {
    get_2d_transformation(PI / 2., 1. / SQRT_2)
}

pub fn get_points_bfs(transformations: &[Affine3<f64>], max_depth: i32) -> Vec<Point3<f64>> {
    let mut output: Vec<Point3<f64>> = Vec::new();
    let mut queue: VecDeque<(Point3<f64>, i32)> = VecDeque::new();

    queue.push_back((Point3::origin(), max_depth));

    while !queue.is_empty() {
        let (point, rdepth) = queue.pop_front().expect("Empyt Queue");

        if rdepth > 0 {
            for transformation in transformations {
                let new_point = transformation * point;
                output.push(point);
                output.push(new_point);
                queue.push_back((transformation * point, rdepth - 1));
            }
        }
    }
    output
}


pub fn test_tree_right() -> Affine3<f64> {
    get_2d_transformation(-PI / 2., 1. / SQRT_2)
}


pub fn test_actually_nice_tree() -> [Affine3<f64>; 2] {
    [
        get_2d_transformation(PI / 4., 0.2),
        get_2d_transformation(-PI / 4., 0.5),
        //get_2d_transformation(-PI / 2., 0.3)
    ]
}