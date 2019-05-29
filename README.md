# localstoragefs

Web-based replacement for `std::fs` using [localStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)

Intended as a drop-in replacement when building for wasm32-unknown-unknown for HTML5.
Example of supporting both native and web with the same API using [cfg-if](https://crates.io/crates/cfg-if):

```rust
cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use localstoragefs::fs;
    } else {
        use std::fs;
    }
}
```

then use `fs::File` as usual. Not all functionality is supported (contributions welcome),
but at least minimal usage should work. See [examples/hello](./examples/hello) for a complete example.

Inspired by [Emscripten's file system support](https://emscripten.org/docs/porting/files/index.html)
(Rust target wasm32-unknown-emscripten), but not as complete. Files are stored as local storage string
values with hex-encoded data. May not be as efficient as alternatives.
