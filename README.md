# wasm-template

This is a template for building web assembly projects with Rust and Webpack. It uses the `wasm-pack` tool to build the Rust code and `webpack` to bundle the JavaScript.

## Usage

- preview with `pnpm run serve`
- build with `pnpm build`

## Warnings

I just ignore these for now...

- `webpack` warnings
    - Circular dependency between chunks with runtime
- `rust-analyzer` warnings (`wasm_bindgen` related, on nightly toolchain)
    - proc macro not expanded: No proc-macros present for crate
    - this operation is unsafe and requires an unsafe function or block

