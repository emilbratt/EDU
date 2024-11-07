### WebAssembly

[mdn web docs - rust wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm)

* Building parts of an application..

```sh
wasm-pack
```

* wasm-pack uses wasm-bindgen, another tool, to provide a bridge between rust and javasript types.

* It allows JavaScript to call a Rust API with a string, or a Rust function to catch a JavaScript exception.

[Building our WebAssembly package](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm#building_our_webassembly_package)

├── Cargo.toml
└── src
    └── lib.rs