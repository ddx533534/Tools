const js = import("./node_modules/@mx/rust-wasm/rust_wasm.js");
js.then((js) => {
  js.greet("WebAssembly");
});
