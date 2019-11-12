# 🌍 Globals

<a href="https://docs.rs/globals"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

Painless globals in Rust. Works with any type that implements Default trait.

When I first started studying Rust, getting a simple global variable was oddly difficult. As I became more experienced, I got tired of these weird macros hanging around my code. This is a library for people who like the simplicity of global singleton in a single one liner and instantiation with standard Default trait. This library also uses no standard library by default so it's great for web assembly and embedded development where you want as little extra as possible. Enjoy.

```toml
[dependencies]
globals = "1"
```
- [x] `#![no_std]` + `alloc`
- [x] uses `lazy_mutex!` under the covers (battle tested safe lazy static mutexes) 
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

# How this works

Rust is a language that values memory safety. When it comes to globals, this means making sure there's exclusive access to things that can be written to. The primary mechanism for protecting globals which could potentially have multiple threads interacting with it is the Mutex. `globals` has a HashMap of singletons of various types stored in a Mutex. When you call `get()` it looks up your singleton, if it doesn't exist, it calls the `Default` trait method `default()` implementation of type to create the singleton instance and stores it in the hashmap. When `get()` is called, this Mutex wrapped singleton is locked and a handle is given to safely interact with this mutex wrapped value -- a `MutexGaurd<T>`. You can interact with the mutex guard as if it were your singleton type (this why it magically looks like you are just interacting with a global). Once your mutex gaurd is dropped, your mutex is unlocked for other threads to interact with your singleton.

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `globals` by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
