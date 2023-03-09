# OpenGL Rust

Example project for small OpenGL programs written in Rust. The project consists of
a library for asset-management and opengl-wrappers as well as multiple binaries.

|||
|---|---|
|**Hello Triangle**|[Hello Triangle] OpenGL example with [gamma correction]|
|**Sobel Cube**|Rotating cube to which multiple filter kernels are applied|
|**Atmospheric Scattering** |Implementation of planetary terrain with atmosphere|

Code has been ported from C++ to Rust from [this repository][VoxelRendering]
and extended using [this article][OpenGL from scratch].

## Installation

I recommend using [rustup] to setup the development environment for Rust. Follow the
[tutorial][rustup] to setup your development environment including cargo, the package/
build manager for Rust.

## Usage

*(Documentation for the library will be added at a later stage, when the API is more
stable)*

Once [rustup] has been installed, you can build the project using `cargo build` or
`cargo build --release` for a release-build respectively.

> :information_source: You might be required to install development libraries for any of the dependencies
> (SDL2, OpenGL)

You can execute the binaries directly from cargo using `cargo run --bin <binary-name>`.
If no binary name is supplied, the command fails and the list of possible binaries is
printed to console.

## Images

![Hello Triangle example](./hello_triangle.png)

![Sobel Cube example](./sobel_cube.png)

[gamma correction]: https://learnopengl.com/Advanced-Lighting/Gamma-Correction

[Hello Triangle]: https://learnopengl.com/Getting-started/Hello-Triangle

[VoxelRendering]: https://github.com/platc2/VoxelRendeirng

[OpenGL from scratch]: http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html

[Rustup]: https://rustup.rs/
