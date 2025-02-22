# thread-register
A library for obtaining and modifying thread register.

# Example
```rust
// Get thread register value
let val = ThreadRegister::get();
// Set thread register value
unsafe { ThreadRegister::set(val) };
```