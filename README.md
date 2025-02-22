[![](https://img.shields.io/crates/v/thread-register.svg)](https://crates.io/crates/thread-register)
[![](https://img.shields.io/crates/d/thread-register.svg)](https://crates.io/crates/thread-register)
[![license](https://img.shields.io/crates/l/thread-register.svg)](https://crates.io/crates/thread-register)
[![thread-register on docs.rs](https://docs.rs/thread-register/badge.svg)](https://docs.rs/thread-register)
# thread-register
A library for obtaining and modifying thread register.

# Example
```rust
// Get thread register value
let val = ThreadRegister::get();
// Set thread register value
unsafe { ThreadRegister::set(val) };
```