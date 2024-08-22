# S2shell

S2shell (pun on S2 Cell and :crab: shell) is a Rust port of Google's [S2 Geometry Library](https://github.com/google/s2geometry), a package for manipulating geometric shapes. Unlike many geometry libraries, S2 is primarily designed to work with spherical geometry, i.e., shapes drawn on a sphere rather than on a planar 2D map. This makes it especially suitable for working with geographic data.

S2 documentation can be found on [s2geometry.io](http://s2geometry.io/).

## Setup

You will need nightly Rust and rustfmt (to format doctests)

1. Install Nightly Rust

    ```
    rustup toolchain install nightly
    ```

2. Install [Nightly rustfmt](https://github.com/rust-lang/rustfmt)
    
    ```
    rustup component add rustfmt --toolchain nightly
    ```

3. Build project

    ```
    cargo build
    ```

4. Run tests

    ```
    cargo test
    ```
    
