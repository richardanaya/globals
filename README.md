# 🌍 Globals

<a href="https://docs.rs/globals"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

Painless globals in Rust. Works with any type that implements Default trait.

```toml
[dependencies]
globals = "0.1.3"
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

let f = globals::get::<Foo>().lock();

assert_eq!(f.v,42);
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
