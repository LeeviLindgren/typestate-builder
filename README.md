# Typestate builder pattern

Demonstration of the typestate pattern used with the builder pattern in Rust. This is heaily insipired by the
[great video](https://www.youtube.com/watch?v=pwmIQzLuYl0) by Jeremy Chone.

The pattern provides a **compile time** safe way to build different structs.
In this example, we have a `Request` and `RequestBuilder`. Using typestates we
can ensure that the `Request` cannot be build without an url and HTTP method
defined. Moreover, the builder needs to be 'sealed' before building. The
sealing prevents the builder to be modified even if the builder is cloned.

See the `src/main.rs` for an example. You can run the program with `cargo run`.
