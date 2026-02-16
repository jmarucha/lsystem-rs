// rotate - move - scale - rotate

extern crate nalgebra as nalgebra;
use std::f64::consts::{PI, SQRT_2};

use na::{Affine3, Rotation3, Scale3, Translation3, Vector3};

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
