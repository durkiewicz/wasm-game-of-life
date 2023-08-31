import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();
requestAnimationFrame(renderLoop);

function renderLoop () {
    pre.textContent = universe.render();
    universe.tick();

    requestAnimationFrame(renderLoop);
}
