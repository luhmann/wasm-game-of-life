# Conways Game of Life with Rust and WASM

This is the implementation of this tutorial: https://rustwasm.github.io/docs/book/game-of-life

## Running

Requirements are working rust-toolchain

```sh
$ cargo install (?)
$ wasm-pack build
$ cd www && npm install && npm start
```

## Tests

`wasm-pack test --chrome --headless`

You can also use the following flags:

- `--firefox`
- `--safari`
- `--node`

## Notes

### General

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
- You can use the `js_sys`-crate to access functions in the JS Standard-Library: https://rustwasm.github.io/wasm-bindgen/api/js_sys/
- `usize`-Type
  > Additionally, the isize and usize types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
- Representing each cell with an 8bit data-type is wasteful as we only need a single bit to represent the state
  - could be optimized by using bitsets: https://rustwasm.github.io/docs/book/game-of-life/implementing.html#interfacing-rust-and-javascript
  - not implemented as the solution leads to uglier code, eg. in representing the cell-state, matching on it and also representing it in JS with a `UInt8Array` but needing to do manual division for data-access
- Rust-generated WebAssembly functions cannot return borrowed references.
- You can have multiple implementation blocks for each struct, if you have public methods in a struct-implementation, you do want to have a `wasm_binding`-macro on then you can create a new implementation without the macro
- You can have config flags like this `#[cfg(test)]` annotating code, this means that the code is only included in the compilation when certain configuration-flags are passed
- You can use the `web-sys`-crate (https://rustwasm.github.io/wasm-bindgen/web-sys/index.html) for getting bindings to the web-apis like `fetch`, `console`, etc

### Testing

- You can test your wasm-functions easily by running the module plus the js interop-code in any browser or node
- Annotate your actual tests with the `#[wasm_bindgen_test]`-macro
- Unit-Tests in Rust go into the same file as the code they test
- There is a mature property-based-testing package for rust called quickcheck https://github.com/BurntSushi/quickcheck (sounds like its pretty much the same package-name everywhere ;=) )

### Debugging

- @see https://rustwasm.github.io/docs/book/reference/debugging.html
- When debugging you need to have debug symbols enabled (seems to be similar to sourcemaps): `wasm-pack build --debug`
  - causes huge performance impact
- You can activate the `console_error_panic_hook` in order to log unexpected rust errors to the console. This can be done by calling `utils::set_panic_hook();` at an appropriate point in your code
- you can log messages with the `web-sys`-crate

  - @see: https://rustwasm.github.io/docs/book/game-of-life/debugging.html#add-logging-to-our-game-of-life
  - Wrap the console-function in a macro, for `println!`-style-comfort

    ```rustwasm
        extern crate web_sys;

        #[macro_export]
        macro_rules! log {
            ( $( $t:tt )* ) => {
                web_sys::console::log_1(&format!( $( $t )* ).into());
            }
        }
    ```

### Profiling

- You can access the tools you are used to from JS like `console.time`/`console.timeEnd` via the `web-sys`-crate
