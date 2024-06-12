import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

const loop = () => {
  universe.tick();
  pre.textContent = universe.render();
  setTimeout(loop, 500);
}

pre.textContent = universe.render();
loop();