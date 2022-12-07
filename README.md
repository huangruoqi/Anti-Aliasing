# Rust Anti-Aliasing Line

A desktop application with a canvas to draw lines, and the line will be displayed with anti-aliasing techniques.

Install Rust: https://www.rust-lang.org/tools/install

Execute either one of the following commands in terminal
* `cargo run --bin anti-aliasing` for anti-aliasing
* `cargo run --bin linear` for linear texture filter

Library:
* using ferrux_canvas currently, open to other Canvas implementations
* using macroquad for linear texture filter

Progress:
* Bresenham's line algorithm
* Xiaolin Wu's line algorithm
* Fast-Approximate Anti-Aliasing (FXAA)
* Supersampling Anti-Aliasing (SSAA)
* Linear Texture Filter



