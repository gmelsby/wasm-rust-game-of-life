import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

pre.textContent = universe.render();

/*
const loop = () => {
  console.log('going');
  pre.textContent = universe.render();
  universe.tick();
  requestAnimationFrame(loop);
}

requestAnimationFrame(loop);

*/