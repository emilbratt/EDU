### WebAssembly

[mdn web docs - rust wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm)

* Building parts of an application..

Run the wasm-pack command to see the options for compiling rust to webassembly.

```sh
wasm-pack
```

* wasm-pack uses wasm-bindgen, another tool, to provide a bridge between rust and javasript types.

* It allows JavaScript to call a Rust API with a string, or a Rust function to catch a JavaScript exception.

[Building our WebAssembly package](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm#building_our_webassembly_package)

├── Cargo.toml
└── src
    └── lib.rs


Run 'hello-wasm' example (cd into hello-wasm directory)

1. Compile to web assembly targetting 'web'.

```sh
wasm-pack build --release --target web
```

2. Serve this directory via http (we can just use python for this).

```sh
python3 -m http.server
```

3. Open a web-browser and visit http://localhost:8000.
