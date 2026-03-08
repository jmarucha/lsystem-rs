// rotate - move - scale - rotate

extern crate nalgebra as na;
use std::collections::VecDeque;
use std::f32::consts::PI;

use na::{Affine3, Point3, RealField, Rotation3, Scale3, Translation3};
use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
    random, random_range,
};

// todo - generalize type
pub fn get_transformation_matrix<T: RealField>(
    rot: &Rotation3<T>,
    trans: &Translation3<T>,
    scale: &Scale3<T>,
) -> Affine3<T> {
    let rot_c: Affine3<T> = na::convert_ref(rot);
    let trans_c: Affine3<T> = na::convert_ref(trans);
    let scale_c: Affine3<T> = na::convert_ref(scale);
    rot_c * scale_c * trans_c
}

pub fn _get_2d_transformation<T: From<f32> + RealField + Copy>(rot: T, scale: T) -> Affine3<T> {
    get_transformation_matrix(
        &Rotation3::from_euler_angles(From::from(0.0), From::from(0.0), rot),
        &Translation3::new(From::from(0.), From::from(1.), From::from(0.)),
        &Scale3::new(scale, scale, scale),
    )
}

pub fn _get_2dd_transformation<T: From<f32> + RealField + Copy>(rot: T, scale: T) -> Affine3<T> {
    get_transformation_matrix(
        &Rotation3::from_euler_angles(From::from(0.0), From::from(PI / 2.0), rot),
        &Translation3::new(From::from(0.), From::from(1.), From::from(0.)),
        &Scale3::new(scale, scale, scale),
    )
}

#[allow(dead_code)]
pub fn get_points_bfs(transformations: &[Affine3<f32>], max_depth: i32) -> Vec<Point3<f32>> {
    // somewhere around 2-4x slower than DFS
    let mut output: Vec<Point3<f32>> = Vec::new();
    let mut queue: VecDeque<(Affine3<_>, i32)> = VecDeque::new();

    queue.push_back((Affine3::identity(), max_depth));

    while !queue.is_empty() {
        let (old_trans, rdepth) = queue.pop_front().unwrap();

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

#[allow(dead_code)]
pub fn get_points_batched<const N: usize>(
    transformations: &[Affine3<f32>; N],
    max_depth: i32,
) -> Vec<Point3<f32>> {
    // was aiming batching operations, actually sucks performance-wise (6-10x worse dhan DFS)
    let mut output: Vec<Point3<f32>> = Vec::new();
    let mut queues: [VecDeque<(Affine3<f32>, i32)>; N] = [const { VecDeque::new() }; N];
    for q in &mut queues {
        q.push_back((Affine3::identity(), max_depth));
    }

    while !(queues.iter().all(|q| -> bool { q.is_empty() })) {
        for i in 0..N {
            while !queues[i].is_empty() {
                let (old_trans, rdepth) = queues[i].pop_back().unwrap();
                let new_trans = transformations[i] * old_trans;
                output.push(new_trans * Point3::new(0., -1., 0.));
                output.push(new_trans * Point3::origin());
                if rdepth > 0 {
                    for q2 in queues.iter_mut() {
                        q2.push_back((new_trans, rdepth - 1));
                    }
                }
            }
        }
    }
    output
}

pub fn get_points_dfs(transformations: &[Affine3<f32>], max_depth: i32) -> Vec<Point3<f32>> {
    let mut output: Vec<Point3<f32>> = Vec::new();
    let mut queue: VecDeque<(Affine3<_>, i32)> = VecDeque::new();

    queue.push_back((Affine3::identity(), max_depth));

    while !queue.is_empty() {
        let (old_trans, rdepth) = queue.pop_back().unwrap();

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

#[allow(unused)]
pub fn test_actually_nice_tree() -> [Affine3<f32>; 3] {
    [
        _get_2dd_transformation(-PI / 4., 0.5),
        _get_2dd_transformation(0., 0.65),
        _get_2dd_transformation(PI / 4., 0.5),
    ]
}

pub enum TreeType {
    Random,
    RandomScaled,
    Degree45,
}

impl Distribution<TreeType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TreeType {
        match rng.random_range(0..3) {
            0 => TreeType::Random,
            1 => TreeType::RandomScaled,
            _ => TreeType::Degree45,
        }
    }
}

pub fn generate_tree(tree_type: TreeType) -> Vec<Affine3<f32>> {
    match tree_type {
        TreeType::Random => generate_random_tree(),
        TreeType::RandomScaled => generate_scale_hierarchy_tree(),
        TreeType::Degree45 => generate_degree_45_tree(),
    }
}

pub fn generate_random_tree() -> Vec<Affine3<f32>> {
    // as random as it can be
    let uniform_scale = &|s: f32| -> Scale3<f32> { Scale3::new(s, s, s) };
    let random_scale = || -> Scale3<f32> { uniform_scale(random_range(0.2..0.8)) };
    vec![
        get_transformation_matrix(
            &random_rotation(),
            &Translation3::new(0., 1., 0.),
            &random_scale(),
        ),
        get_transformation_matrix(
            &random_rotation(),
            &Translation3::new(0., 1., 0.),
            &random_scale(),
        ),
        get_transformation_matrix(
            &random_rotation(),
            &Translation3::new(0., 1., 0.),
            &random_scale(),
        ),
    ]
}

pub fn generate_scale_hierarchy_tree() -> Vec<Affine3<f32>> {
    // sqrt(2) scale for accidental symmetries. doesn't look like much of change
    let uniform_scale = &|s: f32| -> Scale3<f32> { Scale3::new(s, s, s) };

    let scale_base = (2_f32).sqrt().sqrt();
    let random_scale = || -> Scale3<f32> { uniform_scale(scale_base.powi(random_range(-6..-0))) };

    vec![
        get_transformation_matrix(
            &random_rotation(),
            &Translation3::new(0., 1., 0.),
            &random_scale(),
        ),
        get_transformation_matrix(
            &random_rotation(),
            &Translation3::new(0., 1., 0.),
            &random_scale(),
        ),
        get_transformation_matrix(
            &random_rotation(),
            &Translation3::new(0., 1., 0.),
            &random_scale(),
        ),
    ]
}

pub fn generate_degree_45_tree() -> Vec<Affine3<f32>> {
    // only 45 degree twists allowed. sqrt(2) scale for accidental symmetries
    let uniform_scale = &|s: f32| -> Scale3<f32> { Scale3::new(s, s, s) };

    let scale_base = (2_f32).sqrt();
    let random_angle_45 = || -> f32 { random_range(0..8) as f32 * PI / 4.0 };
    let random_rotation_45 = || -> Rotation3<f32> {
        Rotation3::from_euler_angles(random_angle_45(), 0., random_angle_45())
    };

    vec![
        get_transformation_matrix(
            &random_rotation_45(),
            &Translation3::new(0., 1., 0.),
            &uniform_scale(1.0 / scale_base),
        ),
        get_transformation_matrix(
            &random_rotation_45(),
            &Translation3::new(0., 1., 0.),
            &uniform_scale(0.5),
        ),
        get_transformation_matrix(
            &random_rotation_45(),
            &Translation3::new(0., 1., 0.),
            &uniform_scale(0.5 / scale_base),
        ),
    ]
}

pub fn random_rotation() -> Rotation3<f32> {
    Rotation3::from_euler_angles(
        2. * PI * random::<f32>(),
        2. * PI * random::<f32>(),
        2. * PI * random::<f32>(),
    )
}
