# Conways Game of Life with Rust and WASM

This is the implementation of this tutorial: https://rustwasm.github.io/docs/book/game-of-life

## Running

Requirements are working rust-toolchain

`$ cargo install` (?)
`$ wasm-pack build`
`$ cd www && npm install && npm start`

## Notes

- You define the exports of the rust modules by marking the rust language constructs with the `#[wasm_bindgen]`-macro
  - you need to annotate declaration and implementation for structs
  - in a struct only public members are exposed as JS exports
  - everyhing you declared as a binding in such a fashion shows up in the `*_bg.js` file in the generated `pkg`-directory and through this allows interop with wasm
- WASMs memory is linear and can be accessed directly by importing it from the `*_bg.js`-file
  - you can directly access this memory by layering a `UInt*Array` on top of it
  - this saves the overhead of copying data to and from WASMs linear memory to the JS heap
  - this memory is not directly mutable (thank god)
- you can pull in a local directory of files as a node-module by using this syntax in `package.json`: `"wasm-game-of-life": "file:../pkg"`
- Webpack seems to be able to directly import wasm-files without further plugins (?)
