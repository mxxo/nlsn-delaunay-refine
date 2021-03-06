<html>
<h1 align="center">Rust Delaunay Triangulation</h1>
<p align="center" >

<img src="https://img.shields.io/badge/language-rust-blue.svg" />

<img src="https://img.shields.io/github/issues/nelsonatgithub/nlsn-delaunay-refine" />

<img src="https://img.shields.io/github/license/mashape/apistatus.svg" />

<img src="https://img.shields.io/github/stars/nelsonatgithub/nlsn-delaunay-refine" />

<img src="https://img.shields.io/github/forks/nelsonatgithub/nlsn-delaunay-refine" />

</p>
</html>

# Description

This repository implements Delaunay Triangulation in Rust, according to this reference [1].

The major objective is to implement a 3D refinement in pure Rust, so that it may be portable to wasm-pack applications.

I've searched for some packages in open repositories. There were good jobs and efficient implementations ([svew](https://github.com/svew/rust-voroni-diagram), [mourner](https://github.com/mourner/delaunator-rs), [tynril](https://github.com/tynril/rtriangulate), [ucarion](https://github.com/ucarion/voronoi-rs), [d-dorazio](https://github.com/d-dorazio/delaunay-mesh)), but none are extensible to this purpose. Some lack documentation, some follow other approaches.

# Approach

The approach is to implement `Bowyer Watson` incremental insertion algorithm, with `ghost triangles` and `conflict graph` . This approach is extensible to 3D, given the proper handle to sliver exudation and smooth surfaces.

The choice for `Rust` is due to its portability in sereral rust contexts and its integration to `Javascript` through `wasm-pack` .

# Task List

    - [x] 2D Delaunay Triangulation
    - [ ] publishing release to crates.io
    - [ ] 2D Delaunay Refinement

    - [ ] 3D Delaunay Triangulation
    - [ ] 3D Delaunay Refinement

# Features

- Standard Delaunay Triangulation
- Incremental Vertex Insertion
- Decremental Vertex Deletion
- Holes
- Refinement (*in progress*)
- Tetrahedralization (*in progress*)

# API

> In progress

# Contributions

At first, clone the repository, with a cargo environment. Fork it if you want. Run the tests. Read the code.

Open an issue with suggestions, code reviews, refactoring.

# References

1. Cheng, Siu-Wing; Dey, Tama Krishna; Shewchuk, Jonathan Richard. Delaunay Mesh Generation. 2013 by Taylor & Francis Group, LLC.

