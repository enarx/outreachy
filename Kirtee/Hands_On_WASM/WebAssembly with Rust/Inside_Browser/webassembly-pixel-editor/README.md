# Pixel editor with Rust and wasm-bindgen

In [last post](https://www.wasm.builders/kirteeprajapati/rust-with-wasm-bindgen-5080) we got an introduction to the wasm-bindgen module and saw how wasm-bindgen reduces the difficulty in Rust wasm module run using JavaScript with an example project[.
 
**Brief description about wasm-bindgen**

Wasm-bindgen command-line tool reads that metadata to generate an appropriate JavaScript wrapper containing the kinds of functions, classes, and other primitives that the developer wants to be bound to Rust.

allows Rust to see JavaScript classes, expose and invoke callbacks in either language, send strings as function parameters, and return complex values, all while maintaining Rust’s strict sharing rules, and the same goes for JavaScript.

Now we'll be implementing it over a fun project **Pixel-Editor**
The final project will look like this. !
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/mymf4f7j1jsni4soqaun.png)

So to begin initializing a project with Rust-webpack-template.

Install Rust Dependency 
```
$ npm init rust-webpack webassembly-pixel-editor
```
this will create a project with Rust-webpack editor named **webassembly-pixel-editor** now move to that directory
```
$ cd editor
$ npm start
```
once it gets compiled successfully this will launch the project in your default browser or visit.
[http://localhost:8081](http://localhost:8081)

Got a blank page..!! scam. 
wait I'll guide you to create that.

Open project in your fav IDE recommended VSCode for windows and Linux users.

Inside this folder, your file structure would be like this.
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/43ypw3bjex1ud0ytwo6p.png)
 
Inside `index.js` b default it would web dynamic importing of "../pkg/index.js". 

What you have to do is replace all the codes with the one given below.
.

### src/lib.rs
```
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
use std::iter::FromIterator;

use im::Vector;
use wasm_bindgen::prelude::*;
use web_sys::console;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str("Webassembly loaded"));

    Ok(())
}
#[wasm_bindgen]
pub fn addsome(a: i32, b: i32) -> i32 {
    a + b
}
#[wasm_bindgen]
pub fn mainmock() {
    std::thread::spawn(|| println!("whatever"));
    std::thread::spawn(|| println!("whatever2"));
}
#[wasm_bindgen]
#[derive(Clone, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}
#[wasm_bindgen]
#[derive(Clone, PartialEq)]
pub struct Image {
    height: usize,
    width: usize,
    cells: Vector<Rgb>,
}

#[wasm_bindgen]
impl Image {
    pub fn new(height: usize, width: usize) -> Self {

        let cells = Vector::from_iter((0..height * width).map(|i| {
            return Rgb {
                r: 200,
                g: 200,
                b: 255,
            };
        }));
        Image {
            height,
            width,
            cells,
        }
    }

    pub fn getCells(&self) -> Vec<u8> {
        let ret = self
            .cells
            .iter()
            .map(|v| vec![v.r, v.g, v.b])
            .collect::<Vec<Vec<u8>>>();
        let ret = ret.concat();

        ret
    }
    pub fn getHeight(&self) -> usize {
        self.height
    }

    pub fn getWidth(&self) -> usize {
        self.width
    }

    pub fn brush_old(&mut self, x: usize, y: usize, color: Vec<u8>) {
        let offset = y * self.width + x;
        self.cells[offset] = Rgb {
            r: color[0],
            g: color[1],
            b: color[2],
        };
    }

    pub fn brush(&self, x: usize, y: usize, color: Vec<u8>) -> Self {
        let index = y * self.width + x;

        let newCell = self.cells.update(
            index,
            Rgb {
                r: color[0],
                g: color[1],
                b: color[2],
            },
        );

        Self {
            height: self.height,
            width: self.width,
            cells: newCell,
        }
    }
}

#[wasm_bindgen]
pub struct InternalState {
    undo_queue: UndoQueue<Image>,
}

#[wasm_bindgen]
impl InternalState {
    #[wasm_bindgen(constructor)]
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            undo_queue: UndoQueue::new(Image::new(height, width)),
        }
    }
    pub fn getCurrent(&self) -> Image {
        self.undo_queue.getCurrent()
    }
    pub fn push(&self) -> Image {
        self.undo_queue.getCurrent()
    }
    pub fn brush(&mut self, x: usize, y: usize, color: Vec<u8>) {
        let image = self.undo_queue.getCurrent().clone();
        let topush = image.brush(x, y, color);
        if topush != image {
            self.undo_queue.push(topush);
        }
    }

    pub fn undo(&mut self) {
        self.undo_queue.undo();
    }

    pub fn redo(&mut self) {
        self.undo_queue.redo();
    }

    pub fn start_dragging(&mut self) {
        self.undo_queue.setMode(DrawingMode::StartDragging);
    }

    pub fn stop_dragging(&mut self) {
        self.undo_queue.setMode(DrawingMode::Normal);
    }
}

enum DrawingMode {
    Normal,
    StartDragging,
    DraggingProgress,
}
struct UndoQueue<T: Clone> {
    queue: Vec<T>,
    index: usize,
    mode: DrawingMode,
}
impl<T: Clone> UndoQueue<T> {
    fn new(entry: T) -> Self {
        Self {
            queue: vec![entry],
            index: 0,
            mode: DrawingMode::Normal,
        }
    }

    fn getCurrent(&self) -> T {
        self.queue[self.index].clone()
    }

    fn push(&mut self, entry: T) {
        match self.mode {
            DrawingMode::Normal => {
                self.queue.truncate(self.index + 1);
                self.queue.push(entry);
                //to reflect the above push on getCurretn
                self.index += 1;
            }
            DrawingMode::StartDragging => {
                self.queue.truncate(self.index + 1);
                self.queue.push(entry);
                //to reflect the above push on getCurretn
                self.index += 1;
                self.mode = DrawingMode::DraggingProgress
            }
            DrawingMode::DraggingProgress => {
                self.queue[self.index] = entry;
            }
        }
    }

    fn undo(&mut self) {
        console::log_1(&JsValue::from_str("undoinf"));
        if self.index > 0 {
            self.index -= 1;
        }
    }

    fn redo(&mut self) {
        console::log_1(&JsValue::from_str("redoing"));
        if self.index < self.queue.len() - 1 {
            self.index += 1;
        }
    }

    fn setMode(&mut self, mode: DrawingMode) {
        self.mode = mode;
    }
}

#[wasm_bindgen]
pub fn mulbyhundred(a: i32) -> i32 {
    a * 100
}
```
.

### js/index.js
```
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

```
.

### static/index.js
```
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>Pixel editor</title>
  </head>
  <body>
    <script src="index.js"></script>
    <div>
      <canvas id="my-canvas" height="502" width="502" style="background-color: black;"></canvas> </canvas>
    </div>
    <div style="display: flex; width: 500px; height:50px; justify-content: space-between;">
    <button id="red" style="color:white; font-size: large; background-color: red; width: 90px; height: 50px; border-radius: 0.5em;">red</button>
    <button id="green" style="color:white; font-size: large;background-color: green;width: 90px; height: 50px; border-radius: 0.5em;">green</button>
    <button id="blue" style="color:white; font-size: large;background-color: blue;width: 90px; height: 50px; border-radius: 0.5em;">blue</button>
    <button id="undo" style="color:black; font-size: large;background-color: white;width: 90px; height: 50px; border-radius: 0.5em;">undo</button>
    <button id="redo" style="color:black; font-size: large;background-color: white;width: 90px; height: 50px; border-radius: 0.5em;">redo</button>
  </div>
  </body>
</html>

```
now if you check your 
**pkg/index_bg.wasm.d.ts**
it will look something like this

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/4fzr8zxxonr7p7cr4i2y.png)

These are all the functions that we declared for far in the program yet of now. 
with this, you are all set now to make the program run. inside the folder follow the instructions given below.
## How to install

```sh
npm install
```

## To run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

**Compiled successfully Yeahh!!**
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/9srqj9oyb0qvpe8goaj2.png)

Now have fun with this on your browser at port **[http://localhost:8081/](http://localhost:8081/)**

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/lm17p1as8fegr9kseqww.png)

## To build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/tbkjcayaplf2tky13va7.png)
 

## References 

1. **[Tutorial: Conway's Game of Life](https://rustwasm.github.io/book/game-of-life/introduction.html)**
2. **[Rust Wasm Game of life](https://github.com/rustwasm/wasm_game_of_life)**

**[Github Source code](https://github.com/sarik/webassembly-pixel-editor)** 
###### I made some changes in `src/lib.rs` and `.html` folders you will get same results with the codes pasted above.
