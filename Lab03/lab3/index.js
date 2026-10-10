import init, { Cell, Universe } from "./pkg/lab3.js";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const DYING_COLOR = "#FF0000";
const ALIVE_COLOR = "#000000";
const FRAMES_PER_TICK = 6;

init().then((wasm) => {
  const universe = Universe.new();
  const width = universe.width();
  const height = universe.height();

  const canvas = document.getElementById("game-of-life-canvas");
  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;
  const ctx = canvas.getContext("2d");

  let animationId = null;
  let frame = 0;

  const getIndex = (row, column) => row * width + column;

  const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
    ctx.stroke();
  };

  const colorFor = (state) => {
    if (state === Cell.Alive) return ALIVE_COLOR;
    if (state === Cell.Dying) return DYING_COLOR;
    return DEAD_COLOR;
  };

  const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(wasm.memory.buffer, cellsPtr, width * height);

    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        ctx.fillStyle = colorFor(cells[getIndex(row, col)]);
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE,
        );
      }
    }
  };

  const draw = () => {
    drawGrid();
    drawCells();
  };

  const renderLoop = () => {
    frame++;
    if (frame % FRAMES_PER_TICK === 0) {
      universe.tick();
      draw();
    }
    animationId = requestAnimationFrame(renderLoop);
  };

  const isPaused = () => animationId === null;

  const playPauseButton = document.getElementById("play-pause");
  const stepButton = document.getElementById("step");

  const play = () => {
    playPauseButton.textContent = "Stop";
    renderLoop();
  };

  const pause = () => {
    playPauseButton.textContent = "Start";
    cancelAnimationFrame(animationId);
    animationId = null;
  };

  playPauseButton.addEventListener("click", () => {
    if (isPaused()) play();
    else pause();
  });

  stepButton.addEventListener("click", () => {
    if (!isPaused()) pause();
    universe.tick();
    draw();
  });

  draw();
  play();
});
