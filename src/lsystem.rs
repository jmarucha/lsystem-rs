// rotate - move - scale - rotate

extern crate nalgebra as na;
use std::collections::VecDeque;
use std::f32::consts::{PI, SQRT_2};

use na::{Affine3, Point3, Rotation3, Scale3, Translation3};

// todo - generalize type
pub fn get_transformation_matrix(
    rot: &Rotation3<f32>,
    trans: &Translation3<f32>,
    scale: &Scale3<f32>,
) -> Affine3<f32> {
    let rot_c: Affine3<f32> = na::convert_ref(rot);
    let trans_c: Affine3<f32> = na::convert_ref(trans);
    let scale_c: Affine3<f32> = na::convert_ref(scale);
    rot_c * scale_c * trans_c
}

pub fn _get_2d_transformation(rot: f32, scale: f32) -> Affine3<f32> {
    get_transformation_matrix(
        &Rotation3::from_euler_angles(0., 0.0, rot),
        &Translation3::new(0., 1., 0.),
        &Scale3::new(scale, scale, scale),
    )
}

pub fn _get_2dd_transformation(rot: f32, scale: f32) -> Affine3<f32> {
    get_transformation_matrix(
        &Rotation3::from_euler_angles(1., 1., rot),
        &Translation3::new(0., 1., 0.),
        &Scale3::new(scale, scale, scale),
    )
}

pub fn get_points_bfs(transformations: &[Affine3<f32>], max_depth: i32) -> Vec<Point3<f32>> {
    let mut output: Vec<Point3<f32>> = Vec::new();
    let mut queue: VecDeque<(Affine3<_>, i32)> = VecDeque::new();

    queue.push_back((Affine3::identity(), max_depth));

    while !queue.is_empty() {
        let (old_trans, rdepth) = queue.pop_front().expect("Empyt Queue");

        if rdepth > 0 {
            for transformation in transformations {
                let new_trans = transformation * old_trans;
                output.push(new_trans * Point3::new(0., -1., 0.));
                output.push(new_trans * Point3::origin());
                queue.push_back((new_trans, rdepth - 1));
            }
        }
    }
    output
}

pub fn test_actually_nice_tree() -> [Affine3<f32>; 3] {
    [
        _get_2dd_transformation(PI / 4., 0.5),
        _get_2dd_transformation(-PI / 4., 0.5),
        _get_2dd_transformation(0., 0.7),
        //get_2d_transformation(-PI / 2., 0.3)
    ]
}
