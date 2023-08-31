## WASM - Game of Life

This is a simple implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust,
compiled to WebAssembly and run in the browser.
It is mostly based on [this tutorial](https://rustwasm.github.io/docs/book/game-of-life/introduction.html)

### How to run

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
3. Install [npm](https://www.npmjs.com/get-npm)
4. Run `wasm-pack build` in the root directory
5. Run `npm install` in the `www` directory
6. Run `npm run dev` in the `www` directory
7. Open `localhost:5173` in your browser
