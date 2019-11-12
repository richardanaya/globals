# 🌍 Globals

<a href="https://docs.rs/globals"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

Painless globals in Rust. Works with any type that implements Default trait.

When I first started studying Rust, getting a simple global variable was oddly difficult. As I became more experienced, I got tired of these weird macros hanging around my code. This is a library for people who like the simplicity of global singleton in a single one liner and instantiation with standard Default trait. This library also uses no standard library by default so it's great for web assembly and embedded development where you want as little extra as possible. Enjoy.

```toml
[dependencies]
globals = "1.0.0"
```
- [x] `#![no_std]` + `alloc`
- [x] uses `lazy_mutex!` under the covers (battle tested lazy static mutexes) 
- [x] leaves your code nice and clean

## Example

```rust
struct Foo {
  v:u32
}

impl Default for Foo {
  fn default() -> Self {
    Foo {v:42}
  }
}

fn main() {
  let f = globals::get::<Foo>();
  assert_eq!(f.v,42);
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `globals` by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
