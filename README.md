# Globals 🌎🔥⚠️
Painless globals in Rust for people who don't mind being totally unsafe about it. Library writers (especially in web assembly) are many times forced to write global data because they don't have a single point of entry.  *This library will never be multi-thread safe* (there are much [better alternatives](https://github.com/rust-lang-nursery/lazy-static.rs) for multithreaded), but that may not be a problem given your use case.

**use at your own risk of undefined behavior ☠**

```toml
[dependencies]
globals = "0.0.2"
```

## add something into global state

```rust
let f = Foo{}
let handle = globals::add(foo);
```

## get something from global state
```rust
let f = globals::get(handle);
let f = globals::get_mut(handle);
```

## remove something from global state
```rust
let f = globals::remove(handle);
```

## get all things of a certain type
```rust
let fs = globals::get_all::<Foo>();
for f in fs.iter() {
  ...
}
```

## get a singleton of a type
Type must implement Default

```rust
#[derive(Default)]
struct Foo {}

let f = globals::singleton::<Foo>();
let f = globals::singleton_mut::<Foo>();
```
