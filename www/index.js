import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";


const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const CELL_COLORS = ["#FFFFFF", "#15aa20", "#15aa6b", "#159faa", "#1554aa", "#2015aa", "#6b15aa", "#000000"]

const universe = Universe.new(256, 256);
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
      const cellVal = cells[universe.get_index(i, j)];
      ctx.fillStyle = CELL_COLORS[cellVal]

      ctx.fillRect(
        j * (CELL_SIZE + 1) + 1,
        i * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    });
  });
}

canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  const universeWidth = universe.width();
  const universeHeight = universe.height();

  if (event.shiftKey) {
    // draw glider
    const coordinates = [
      [0, 0],
      [1, 0],
      [0, 1],
      [1, -1],
      [-1, -1]
    ];

    coordinates.forEach(([r_delta, c_delta]) => {
      universe.insert_cell((row + r_delta) % universeHeight, (col + c_delta) % universeWidth);
    });
  } else if (event.altKey) {
    //draws lwss
    const coordinates = [
      [0, 0],
      [0, 1],
      [0, 2],
      [0, 3],
      [-1, 0],
      [-2, 0],
      [-3, 1],
      [-3, 4],
      [-1, 4],
    ];
    coordinates.forEach(([r_delta, c_delta]) => {
      universe.insert_cell((row + r_delta) % universeHeight, (col + c_delta) % universeWidth);
    });
  } else {
    universe.toggle_cell(row, col);
  }

  drawGrid();
  drawCells();
});


let animationId = null;
const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  loop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (animationId === null) {
    play();
  } else {
    pause();
  }
});

const loop = () => {
  drawGrid();
  drawCells();
  universe.tick();
  animationId = requestAnimationFrame(loop);
}

play();