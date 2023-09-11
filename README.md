# Vecto-rs
This is a math library where i will dump all my data types.

## Current Contents:
-   Vector
-   Quadtree
-   AABB ( Axis Aligned Bounding Box )
-   Useful triangle functions

## Using the Library
This is not available on crates.io.
In order to use this Library use this command.
```bash
cargo add --git https://github.com/Zycrasion/vecto-rs/
```

## Todo before releasing version 1.0.0
- [ ] Stabilise the API
- [ ] Add Polish and Quality Of Life features
- [x] Change Vector 2 & 3 (Merge them together?)
- [x] Document Everything
- [ ] Comprehensive Tests
  - [ ] Vector
    - [ ] Vector +, -, *, /
      - [ ] f32
      - [x] Vector
    - [ ] new2
    - [ ] new3
    - [ ] magnitude
    - [ ] dist
    - [ ] clamp
    - [ ] normalized
    - [x] ==, >, <, <=, >=
    - [ ] From
      - [ ] From<(f32, f32)>
      - [ ] From<(f32, f32, f32)>
    - [ ] Into
      - [ ] Into<(f32, f32)>
      - [ ] Into<(f32, f32, f32)>
  - [x] Triangles
    - [x] new
    - [x] get_edge
      - [x] AB, BC, CA
    - [x] point_inside_triangle
    - [x] barycentric coordinates
  - [x] QuadTree
  - [ ] Lines
    - [ ] edge_function
  - [x] AABB
    - [x] new
    - [x] point_inside
    - [x] intersection

## Version
This library is currently pre 1.0.0, therefore many things may change without any warnings. I might change any public api without backwards compatibility.
