# Globals
Painless globals in Rust. Works with any type that implements Default trait.

```toml
[dependencies]
globals = "0.1.0"
```

Example

```rust
struct Foo {
  v:u32
}

impl Default for Foo {
  fn default() -> Self {
    Foo {v:42}
  }
}

let f = globals::get<Foo>().lock();

assert_eq!(f.v,42);
```