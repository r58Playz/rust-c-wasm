# rust-c-wasm
Example project compiling C to wasm(32-unknown-unknown) without Emscripten and linking with Rust and its allocator.

## Explanation

[This StackOverflow post](https://stackoverflow.com/questions/51666736/how-do-i-use-a-c-library-in-a-rust-library-compiled-to-webassembly) was the initial starting point.

`clang` can compile straight to a relocatable wasm module, without linking with a libc with `clang -c <something.c> -o <something.wasm> --target=wasm32-unknown-unknown`.
Clang is run in `build.rs` to compile the C code to wasm and then `llvm-ar` is used to package the wasm into an ar archive so `rustc` can link with it.
The rest of the work is done by `rustc` and `cargo`.

## Optimizing for size further

- Run `wasm-opt -Oz` on the wasm file
- See the `wasm-opt`/binaryen [Optimizer Cookbook](https://github.com/WebAssembly/binaryen/wiki/Optimizer-Cookbook)
