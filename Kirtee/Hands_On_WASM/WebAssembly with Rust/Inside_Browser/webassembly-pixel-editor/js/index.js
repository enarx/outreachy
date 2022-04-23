import("../pkg/index.js")
  .catch(console.error)
  .then((wasm) => {
    window.mainmock = wasm.mainmock;
    let internalState = new wasm.InternalState(10, 10);
    let state = {
      internalState,
      currentColor: [200, 255, 200],
      dragging: false,
    };
    setUpCanvas(state);
    drawToCanvas(state);
  });
function setUpCanvas(state) {
  const image = state.internalState.getCurrent();
  const c = document.getElementById("my-canvas");

  let palette = ["red", "green", "blue", "undo", "redo"];
  palette.forEach((color) => {
    let colorButton = document.getElementById(color);
    colorButton.addEventListener("click", (e) => {
      switch (e.target.innerText) {
        case "prev":
          break;
        case "undo":
          state.internalState.undo();
          drawToCanvas(state);
          break;
        case "redo":
          state.internalState.redo();
          drawToCanvas(state);
          break;
        case "red":
          state.currentColor = [255, 0, 0];
          break;
        case "blue":
          state.currentColor = [0, 0, 255];
          break;
        case "green":
          state.currentColor = [0, 255, 0];
      }
    });
  });
  const cellSize = 50;
  c.addEventListener("mousedown", (e) => {
    state.dragging = true;
    state.internalState.start_dragging();
  });
  c.addEventListener("mouseup", (e) => {
    state.dragging = false;
    state.internalState.stop_dragging();
  });
  c.addEventListener("mousemove", (e) => {
    if (!state.dragging) return;
    const rect = c.getBoundingClientRect();
    let x = e.clientX - rect.left;
    let y = e.clientY - rect.top;
    x = Math.floor(x / cellSize);
    y = Math.floor(y / cellSize);
    state.internalState.brush(x, y, state.currentColor);
    drawToCanvas(state);
  });
  c.addEventListener("click", (e) => {
    const rect = c.getBoundingClientRect();
    let x = e.clientX - rect.left;
    let y = e.clientY - rect.top;
    x = Math.floor(x / cellSize);
    y = Math.floor(y / cellSize);

    state.internalState.brush(x, y, state.currentColor);
    drawToCanvas(state);
  });
}

function drawToCanvas(state) {
  const image = state.internalState.getCurrent();

  const c = document.getElementById("my-canvas");
  const context = c.getContext("2d");
  const cellSize = 50;

  context.strokeStyle = "black";
  context.lineWidth = 1;

  const width = image.getWidth();
  const height = image.getHeight();
  const cells = image.getCells();
  let isRed = false;
  let x = 1;
  for (let x = 0; x < width; x++) {
    for (let y = 0; y < height; y++) {
      const index = (y * width + x) * 3;

      let color = `rgb(${cells[index + 0]}, ${cells[index + 1]}, ${
        cells[index + 2]
      })`;
      context.fillStyle = color;
      context.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
    }
    isRed = !isRed;
  }

  for (let x = 0; x < 10; x++) {
    context.beginPath();
    context.moveTo(x * cellSize + 0.5, 0);
    context.lineTo(x * cellSize + 0.5, height * cellSize);
    context.stroke();
  }

  for (let y = 0; y < 10; y++) {
    context.beginPath();
    context.moveTo(0, y * cellSize + 0.5);
    context.lineTo(width * cellSize, y * cellSize + 0.5);
    context.stroke();
  }
}

function drawToCanvas2(state) {
  const image = state.image;

  const c = document.getElementById("my-canvas");
  const context = c.getContext("2d");

  context.strokeStyle = "black";
  context.lineWidth = 1;

  const width = 10;
  const height = 10;
  const cellSize = 50;

  for (let x = 0; x < width; x++) {
    context.beginPath();
    context.moveTo(x * cellSize + 0.5, 0);
    context.lineTo(x * cellSize + 0.5, height * cellSize);
    context.stroke();
  }

  for (let y = 0; y < 10; y++) {
    context.beginPath();
    context.moveTo(0, y * cellSize + 0.5);
    context.lineTo(width * cellSize, y * cellSize + 0.5);
    context.stroke();
  }
}
