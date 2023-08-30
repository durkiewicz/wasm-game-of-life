import * as wasm from "wasm-game-of-life";

export function setupCounter(element) {
    element.addEventListener('click', () => wasm.greet())
}
