import { Universe, Cell } from "wasm-game-of-life";
// * this DIRECTLY accesses Web Assembly's memory
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
import { fps } from "./frame-counter";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// * these objects originate in rust, typescript definition files are created to provide
// * type information
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

canvas.addEventListener("click", (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
});

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  // * first argument is all of wasms memory
  // * second argument provides the pointer (offset) to the beginning of the cells data
  // * third argument provides the length of data in memory that holds the info of the cells
  // * in result the Uint8Array now directly accesses exactly all the internal data about the cells within wasm
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

const isPaused = () => {
  return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶️";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", (event) => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const ticksPerFrame = document.getElementById("num-ticks");

let ticksPerFrameValue = 1;
ticksPerFrame.addEventListener("change", (event) => {
  console.log("change", event, event.target.value);
  ticksPerFrameValue = event.target.value;
});

let animationId = null;
const renderLoop = () => {
  fps.render();
  for (let i = 0; i < ticksPerFrameValue; i++) {
    universe.tick();
  }

  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};

drawGrid();
drawCells();
play();
