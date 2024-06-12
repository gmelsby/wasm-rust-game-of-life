import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";


const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  [...Array(width)].forEach((_, i) => {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * width + 1);
  });

  [...Array(height)].forEach((_, i) => {
    ctx.moveTo(0, i * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, i * (CELL_SIZE + 1) + 1);
  });

  ctx.stroke();
}

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  [...Array(height)].forEach((_, i) => {
    [...Array(width)].forEach((_, j) => {
      const cell = cells[universe.get_index(i, j)];
      ctx.fillStyle = cell === Cell.Alive ?
      ALIVE_COLOR :
      DEAD_COLOR;

      ctx.fillRect(
        j * (CELL_SIZE + 1) + 1,
        i * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    });
  });
}

const loop = () => {
  drawGrid();
  drawCells();
  universe.tick();
  setTimeout(loop, 500);
}

loop();