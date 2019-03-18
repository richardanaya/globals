# globals
Painless globals in Rust for people who don't mind being unsafe because sometimes it's not a big deal.

```toml
[dependencies]
globals = "0.0.1"
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
globals::remove(handle);
```

## get a singleton of a type
Type must implement Default

```rust
#[derive(Default)]
struct Foo {}

let f = globals::singleton::<Foo>();
let f = globals::singleton_mut::<Foo>();
```
