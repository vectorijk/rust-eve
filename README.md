# Rust EVEonline Character Sniffer

Copyright © 2018 Kai Jiang


### Intro

This is a tool for the EVEonline recruitment. It is easy to view / share all of your ESI data.
The idea is inspired by [ESI Knife](https://esi.a-t.al/)

The project is implemented by [Rust](www.rust-lang.org) and Rocket (A web framework for Rust. https://rocket.rs). Based on web service, 
this project can be also feasible extended to a Rust API client library.

### Building

build on rust nightly version

`rustup default nightly`

`rustup override set nightly-2018-05-15`

`cargo run --package rust-eve --bin rust-eve`

### Testing

unit tests are in `src/tests.rs`

run `cargo test`

### Documentation

Detail document by rust doc generator
`cargo doc`
open `target/doc/rust_eve/index.html`

See `docs/vision.md` for a roadmap and status.

### License

This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for license terms.

### Contact

email: jiangkai@gmail.com

### screenshots

[pic1](screenshot/Screen%20Shot%202018-06-12%20at%2007.45.32.png)
[pic2](screenshot/Screen%20Shot%202018-06-12%20at%2007.45.44.png)
