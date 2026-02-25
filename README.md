# L-System Tree Generator

3D visualization of procedurally generated trees using L-Systems (Lindenmayer Systems) in Rust.

## Overview

This project implements an interactive 3D tree generator that creates realistic tree structures using L-System rules and transformation matrices. The generated trees are visualized in real-time using OpenGL rendering through Glium.

## Images

![Fractal 1](/screenshots/1.png)
![Fractal 2](/screenshots/2.png)
![Fractal 3](/screenshots/3.png)
![Fractal 4](/screenshots/4.png)
![Fractal 5](/screenshots/5.png)

## Features

- **Procedural Tree Generation**: Generates trees using L-System transformations with customizable parameters
- **Real-Time 3D Rendering**: Interactive visualization with OpenGL (Glium)
- **Multiple Traversal Algorithms**: 
  - Depth-First Search (DFS) - fastest
  - Breadth-First Search (BFS) - alternative traversal
  - Batched processing - experimental
- **Interactive Controls**: Real-time camera control and tree regeneration
- **Benchmark Suite**: Criterion-based benchmarks for performance analysis across different tree depths

## Project Structure

- `src/main.rs` - Application entry point with interactive visualization and camera controls
- `src/lsystem.rs` - Core L-System implementation with transformation matrices and traversal algorithms
- `src/render.rs` - OpenGL rendering engine and shader management
- `src/glue.rs` - Data conversion utilities (points to vertices)
- `src/lib.rs` - API exports (for benchmarks)
- `benches/tree_gen.rs` - Performance benchmarks

## Dependencies

- **nalgebra** (0.34.1) - Linear algebra for transformations and 3D math
- **glium** (0.36) - OpenGL bindings and rendering
- **rand** (0.10) - Random number generation for procedural variation
- **criterion** (0.5) - Benchmarking framework with HTML reports

## Build & Run

### Debug Build (Less _performante_)
```bash
cargo run
```
Default recursion depth: 10 levels

### Release Build (More _performante_)
```bash
cargo build --release
./target/release/lsystem
```
Default recursion depth: 13 levels

## Interactive Controls

- **W/A/S/D** - Camera movement
- **Space** - Toggle automatic tree rotation
- **N** - Generate a new random tree
- **Q** - Toggle temporal anti-aliasing (TAA)
- **Esc** - Exit application

## Performance

The project includes comprehensive benchmarks comparing different traversal algorithms:

```bash
cargo bench
```

Generates HTML reports in `target/criterion/` showing performance comparisons between tree-generating algorithms

## How It Works

### L-System Implementation

Trees are generated using transformation matrices that define how branches grow. Each level of recursion applies a set of transformation rules to create increasingly complex structures.

### Rendering

The renderer uses:
- Vertex shaders for dynamic point positioning and animation
- Line rendering to visualize tree branches
- Rudimentary temporal anti-aliasing for smoother visuals (enable for still images, otherwise it sucks)

It's a rather primitive pipeline, and TAA has no motion conservation, so it's basically camera shake with PS2-style blur


## Optimization Notes

- DFS traversal is the fastest method (2-4x faster than BFS)
- Batched processing sucks and currently slower than DFS
- Release builds enable higher recursion depths as rust optimization is unreal.
- No parallelism or whatsoever.

## Future Improvements

- Additional tree generators.
- Improved batched traversal performance, possibly parallel.
