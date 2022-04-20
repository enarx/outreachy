# Building Tic Tac Toe Game in Cpp using Wasm

![1](https://github.com/aryankaushik-git/outreachy/blob/main/aryankaushik/media/Tic%20tac%20toc%20using%20wasm.png)

Hi Folks, Welcome to another fanatstic blog where we will build a Tic tac toe game in Cpp using WebAssembly and will run it on browser.

So lets get started.

#### 1. Install Emscripten

Enscrypten directly converts your C/C++ code to JavaScript which means you don’t even need to write any glue code for reading your .wasm file. This gives you enough flexibility to solely focus on the logic rather than implementation.
Secondly, it is very intuitive in terms of calling functions written in your C++ file to your JS file. You will see this once we dive into the code.

```
# Get the emsdk repo
git clone https://github.com/emscripten-core/emsdk.git

# Enter that directory
cd emsdk

Fetch the latest version of the emsdk (not needed the first time you clone)
git pull

# Download and install the latest SDK tools.
./emsdk install latest

# Make the "latest" SDK "active" for the current user. (writes .emscripten file)
./emsdk activate latest

# Activate PATH and other environment variables in the current terminal
source ./emsdk_env.sh

```

#### 2. Write the Logic for tic tac toe

We will be building a 3×3 tic-tac-toe game. Our game logic includes:

a. Computing which player has won
b. Calculating if there is no winner
c. Indicating pending moves

Here is the program:
```
#include <emscripten/bind.h>
#include <emscripten/val.h>


using namespace emscripten;

val tic_tac_toe() {
    val board_values = val::global("BoardValues");
    val moves_pending_label = val::global("movesPendingLabel");
    val no_winner_label = val::global("noWinnerLabel");
    val empty_block = val::global("emptyBlock");
    bool moves_pending = false;

    val solutions[8][3]= {
        { board_values[0][0], board_values[0][1], board_values[0][2]},
        { board_values[1][0], board_values[1][1], board_values[1][2]},
        { board_values[2][0], board_values[2][1], board_values[2][2]},
        { board_values[0][0], board_values[1][0], board_values[2][0]},
        { board_values[0][1], board_values[1][1], board_values[2][1]},
        { board_values[0][2], board_values[1][2], board_values[2][2]},
        { board_values[0][0], board_values[1][1], board_values[2][2]},
        { board_values[0][2], board_values[1][1], board_values[2][0]},
    };

    for ( int i = 0; i < 8; i++ ){
        if((solutions[i][0] != empty_block) && (solutions[i][1] != empty_block) && (solutions[i][2] != empty_block)&& (solutions[i][0] == solutions[i][1]) && ( solutions[i][1] == solutions[i][2] )) {
            return solutions[i][1];
        } else if((solutions[i][0] == empty_block) || (solutions[i][1] == empty_block) || (solutions[i][2] == empty_block)){
            moves_pending = true;
        }
   }

   if (moves_pending) {
       return moves_pending_label;
   }
    
    return no_winner_label;
    
   
}

EMSCRIPTEN_BINDINGS(my_module) {
    function("tic_tac_toe", &tic_tac_toe);
}
```

#### 3. Build command for converting C++ files to .wasm & .js files

```
emcc --bind -o tic_tac_toe.js tic_tac_toe.cpp
```

Now a index.html, tic_tac_toe.js & tic_tac_toe.wasm file will be genereated

![wasm](https://www.wasm.builders/remoteimages/uploads/articles/t854814j1w19x8qrj6ha.png)

Remember our tic_tac_toe() C++ function which was exposed by Embind, that is now available on the Emscripten module object.

However, we can only call our tic_tac_toe() once it’s fully loaded i.e, it’s runtime(.js glue code and .wasm file) is initialized. For this, we use onRuntimeInitialized callback which will run when the runtime is ready.

Now whenever any player clicks on any cell we call our C++ function as Module.tic_tac_toe() which will return the appropriate results.



#### 4. Install Live Server to deploy & view your changes

```
npm install -g live-server
```
#### 5. Open the project 

```
python3 -m http.server 8080
```

#### Go to browser and type in localhost:8080 or http://127.0.0.1:8080/

![localhost](https://www.wasm.builders/remoteimages/uploads/articles/k9yo4w9qel1n1tdb1476.png)
 
 
Hence our the game is ready to play on the browser. Two players can play it together.

![final](https://www.wasm.builders/remoteimages/uploads/articles/u3gkja042hpufccz8otl.png)
 

WebAssembly is truly remarkable and has indeed allowed us to do things that were previously impossible. This game can be your first step to work with wasm. 

Do check our previously created projects: 
1. [Video to gif convertor using Wasm](https://www.wasm.builders/aryank21/gif-creator-using-wasm-27fl)
2. [Image Recognition App using GoLang | Tensorflow | WasmEdge | Dapr | Docker](https://www.wasm.builders/aryank21/image-recognition-app-using-golang-tensorflow-wasmedge-dapr-docker-1el7)

References:
1. https://emscripten.org/
2. https://github.com/Canop/wasm-tictactoe
3. https://dev.to/dystroy/an-exploration-or-rust-wasm-50gp
4. https://dev.to/ysflghou/build-tic-tac-toe-game-with-blazor-webassembly-52ih
5. https://www.roundthecode.com/dotnet/blazor/create-tic-tac-toe-blazor-webassembly-in-hour
6. https://www.roundthecode.com/dotnet-samples/tic-tac-toe-game-blazor-webassembly
7. https://blog.logrocket.com/first-game-in-webassembly/
8. https://github.com/tapio/live-server#readme
9. https://github.com/arwalokhandwala/tictactoe-game-wasm


Do comment your ideas and suggestions related to the blog and please share if you found it useful.
Write your queries in comment section, we'll help you to resolve your errors.
