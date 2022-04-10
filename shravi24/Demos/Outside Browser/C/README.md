<h1>Calculator</h1>
In this tutorial, We are going to build  calculator using c and compile it to WebAssembly. We will use Wasmer to run outside the browser.
<p>&nbsp;</p>

<h2>Pre-requisites</h2>

Please ensure the [steps included in toolsetup](https://github.com/enarx/outreachy/tree/main/shravi24/Demos/Outside%20Browser) included in toolsetup are completed.

In this example I have used Wasmer as runtime.Please make sure you have installed Wasmer at  [link]( https://github.com/enarx/outreachy/tree/main/shravi24/Demos/Outside%20Browser/C/Wasmer).

<p>&nbsp;</p>

<h2> Writing Code</h2>
You can use any IDE of your choice for writing c code.
Here I have used Visual Studio and created one .c file 

```c

#include <emscripten.h>

EMSCRIPTEN_KEEPALIVE
float add(float x, float y) {
  return x + y;
}

EMSCRIPTEN_KEEPALIVE
float multiply(float x, float y) {
  return x * y;
}

EMSCRIPTEN_KEEPALIVE
float subtract(float x, float y) {
  return x - y;
}

EMSCRIPTEN_KEEPALIVE
float divide(float x, float y) {
  return x / y;
}

```

Compile the code using below command :

```bash
emcc -O2 -s WASM=1 -s SIDE_MODULE=1 calci.c -o  calci.wasm
```

Run the code using Wasmer :

```bash
wasmer calci.wasm -i add 80 90
```

Similarly, you can do subtract, divide, multiple.

You can also refer to below picture :

<img src ="../../../images/Pasted image 20211102202651.png">