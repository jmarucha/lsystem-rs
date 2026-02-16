mod lsystem;
mod lsystem_test;

use crate::lsystem_test::test_tree_that_sucks;

use crate::lsystem_test::test_bfs_that_sucks;

fn main() {
    println!("Hello, world!");
    test_tree_that_sucks();

    println!("dupa");
    test_bfs_that_sucks();
}
