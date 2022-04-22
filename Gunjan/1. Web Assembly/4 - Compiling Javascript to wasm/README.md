# 4 - Compiling Javascript to wasm

In this post, we will convert our javascript code to web assembly.
So, first let's setup our environment.

## Local Environment Setup

1) [Javy](https://github.com/Shopify/javy#build) - A JavaScript to WebAssembly toolchain. It takes your JavaScript code, and executes it in a WebAssembly embedded JavaScript runtime.

In your terminal/shell clone the javy repository as follows:

```
git clone https://github.com/Shopify/javy.git
```
Now, navigate to javy by executing `cd javy`
Here, you have to install all the [build](https://github.com/Shopify/javy#build) and [development](https://github.com/Shopify/javy#development) dependencies in javy directory from [here](https://github.com/Shopify/javy).

After all the dependencies are installed, run `make`. You should now have access to the executable in target/release/javy

Alternatively you can run `make && cargo install --path crates/cli`. After running the previous command you'll have a global installation of the executable.

2) [msgpack-tools](https://github.com/ludocode/msgpack-tools) - It contains simple command-line utilities for converting from MessagePack to JSON and vice-versa.

It can installed by 

```
git clone https://github.com/ludocode/msgpack-tools.git
```
Now, navigate to msgpack-tools by `cd msgpack-tools` and run the following command

```
./configure && make && sudo make install
```
3)[ Wasmtime](https://wasmtime.dev/) - A small and efficient runtime for WebAssembly & WASI. 
It can be installed by 

```
curl https://wasmtime.dev/install.sh -sSf | bash
```

## Directory Structure
   - index.js (it will contain our source code)
   - input.json (it will be our input file)
_The reason you need to create this JSON file is because the default implementation of Javy expects a msgpack input to be sent through stdin_

index.js 
We need to call our main function i.e., fibonacci in our case, from the shopify object.

```
//Simple Program to calculate Fibonacci Sequence of an Integer Input
function fibonacci(num){
  var a = 1, b = 0, temp;

  while (num >= 0){
    temp = a;
    a = a + b;
    b = temp;
    num--;
  }
console.log("Fibonacci Term is ",b);

}

var Shopify = {
  main: fibonacci
};
```

input.json

```
10
```
'10' here will be input for our fibonacci function

## Compiling our code

1) Convert javascript code to wasm binary 

```
javy index.js -o index.wasm
```

2) Execute wasm file, using msgtools

```
json2msgpack -i input.json | wasmtime run index.wasm | msgpack2json
```
You will see the output:


![output file](https://www.wasm.builders/remoteimages/uploads/articles/6426oqwr58otrsfhs37b.png)

References - [https://www.wasm.builders/deepanshu1484/javascript-and-wasi-24k8](https://www.wasm.builders/deepanshu1484/javascript-and-wasi-24k8)

[https://enarx.dev/docs/WebAssembly/JavaScript](https://enarx.dev/docs/WebAssembly/JavaScript) 

[Tutorial for implementation](https://www.wasm.builders/gunjan_0307/compiling-javascript-to-wasm-34lk)

