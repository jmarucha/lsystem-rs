mod lsystem;

extern crate nalgebra as na;
use na::{Affine3, Point3, Rotation3, Scale3, Translation3, Vector3};

use crate::lsystem::test_tree;

// use lsystem::pyth_tree1;

fn main() {
    println!("Hello, world!");
    let trans= test_tree();
    println!("{}", test_tree().into_inner());

    let mut pt = Point3::origin();
    for _ in 1..20 {
        println!("{}", pt);
        pt = trans * pt
    }
}
