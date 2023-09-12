# Vecto-rs
This is a math library where i will dump all my data types.

## Current Contents:
-   Vector
-   Quadtree
-   AABB ( Axis Aligned Bounding Box )
-   Useful triangle functions
-   Lines

## Using the Library
This is not available on crates.io.
In order to use this Library use this command.
```bash
cargo add --git https://github.com/Zycrasion/vecto-rs/
```

## Todo before releasing version 1.0.0
- [x] Stabilise the API
- [x] Add Polish and Quality Of Life features
- [x] Change Vector 2 & 3 (Merge them together?)
- [x] Document Everything
- [x] Comprehensive Tests
  - [x] Vector
    - [x] Vector +, -, *, /
      - [x] f32
      - [x] Vector
    - [x] new2
    - [x] new3
    - [x] magnitude
    - [x] dist
    - [x] clamp
    - [x] normalized
    - [x] ==, >, <, <=, >=
    - [x] From
      - [x] From<(f32, f32)>
      - [x] From<(f32, f32, f32)>
    - [x] Into
      - [x] Into<(f32, f32)>
      - [x] Into<(f32, f32, f32)>
  - [x] Triangles
    - [x] new
    - [x] get_edge
      - [x] AB, BC, CA
    - [x] point_inside_triangle
    - [x] barycentric coordinates
  - [x] QuadTree
  - [x] Lines
    - [x] edge_function
  - [x] AABB
    - [x] new
    - [x] point_inside
    - [x] intersection

## Version
This library is currently pre 1.0.0, therefore many things may change without any warnings. I might change any public api without backwards compatibility.
