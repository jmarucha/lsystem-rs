use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lsystem::{get_points_bfs, get_points_dfs, get_points_batched};
use lsystem::test_actually_nice_tree;

fn benchmark_get_points_bfs(c: &mut Criterion) {
    c.bench_function("get_points_bfs_depth_5", |b| {
        b.iter(|| {
            let transformations = black_box(test_actually_nice_tree());
            get_points_bfs(&transformations, 5)
        })
    });

    c.bench_function("get_points_bfs_depth_8", |b| {
        b.iter(|| {
            let transformations = black_box(test_actually_nice_tree());
            get_points_bfs(&transformations, 8)
        })
    });

    c.bench_function("get_points_bfs_depth_9", |b| {
        b.iter(|| {
            let transformations = black_box(test_actually_nice_tree());
            get_points_bfs(&transformations, 9)
        })
    });
}

fn benchmark_get_points_dfs(c: &mut Criterion) {
    c.bench_function("get_points_dfs_depth_5", |b| {
        b.iter(|| {
            let transformations = black_box(test_actually_nice_tree());
            get_points_dfs(&transformations, 5)
        })
    });

    c.bench_function("get_points_dfs_depth_8", |b| {
        b.iter(|| {
            let transformations = black_box(test_actually_nice_tree());
            get_points_dfs(&transformations, 8)
        })
    });

    c.bench_function("get_points_dfs_depth_9", |b| {
        b.iter(|| {
            let transformations = black_box(test_actually_nice_tree());
            get_points_dfs(&transformations, 9)
        })
    });
}

fn benchmark_get_points_batched(c: &mut Criterion) {
    c.bench_function("get_points_batched_depth_5", |b| {
        b.iter(|| {
            let transformations = black_box(test_actually_nice_tree());
            get_points_batched(&transformations, 5)
        })
    });

    c.bench_function("get_points_batched_depth_8", |b| {
        b.iter(|| {
            let transformations = black_box(test_actually_nice_tree());
            get_points_batched(&transformations, 8)
        })
    });

    c.bench_function("get_points_batched_depth_9", |b| {
        b.iter(|| {
            let transformations = black_box(test_actually_nice_tree());
            get_points_batched(&transformations, 9)
        })
    });
}

criterion_group!(benches, benchmark_get_points_bfs, benchmark_get_points_dfs, benchmark_get_points_batched);
criterion_main!(benches);
