extern crate nalgebra as na;
use crate::lsystem::{getPointsBFS, test_tree, test_tree_right};
use na::Point3;

pub fn test_tree_that_sucks() {
    let trans = test_tree();
    println!("{}", test_tree().into_inner());

    let mut pt = Point3::origin();
    for _ in 1..20 {
        println!("{}", pt);
        pt = trans * pt
    }
}

pub fn test_bfs_that_sucks() {
    let trans = test_tree();
    let trans2 = test_tree_right();
    let points = getPointsBFS(&[trans, trans2], 3);
    for p in points {
        println!("{}", p);
    }
}
