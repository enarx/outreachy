# WebAssembly in C - Example

A simple example to create a WebAssemby code using C

## Introduction

<p align="justify"> WebAssembly also known as WASM (web assembly state machine) is not just a programming language or instruction set, it is also a compilation target for other languages to compile to, as well as a language in itself. 
WASM is augmentation of the abilities of JS by allowing other languages to operate in the browser, it executes binary code in the browser.

For instance, you have a C++ or Rust code running on terminal and you want to show i off to your friends, webasembly can help you do that seamlessly, You write code in other languages, compile them into WebAssembly and ship o the web!. Rust, C/C++, Go, C#; these are just a few of the languages with WebAssembly as a compile target.</p>

## How it Works

High level languages like C, C++ and Rust are compiled into binary format .wasm and text format .wat using a compiler. Later, the .wasm code can be used with the help of javascript in your html file to display the output.
![alt text](https://www.tutorialspoint.com/webassembly/images/wasm.jpg)

## Why WASM
Using WebAssembly means using the right tool for the job. For instance, who wants to write a banking app in JS?. If you're running anything that relies on mathematical numerical accuracy or speed that meant, until now, another AJAX call to have another language do alllllllll the math. With WebAssembly, we can do this in the browser, with, say, Rust.

## Environment setup

To compile, you must install Emscripten compiler.

```bash
cd ~/
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
emsdk install latest
emsdk activate latest
```

With the tools installed, you will also want to set the some environment variables. Add variables to PATH:

```bash
emsdk_env.bat             (windows)

source ./emsdk_env.sh      (unix)
```
The updates that this makes to environment variables isn't persistent; it will need to be run again with the next reboot.

## Compiling

To compile the code: 

```bash
emcc add.c -o add.js
```

This will create two files: `add.js` and `add.wasm`.

add.wasm - the compiled version of your program

add.html - an HTML page for hosting your web assembly

add.js - JavaScript for loading your web assembly into the page

## Serving

If you try to open the HTML file directly, your code probably will not run. Instead, the page will have to be served through an HTTP server
Install the package `http-server` for the node application to be able to serve the html file:

```bash
npm install -g http-server
```

And start it:

```bash
http-server
```

Now, open the browser and access `http://localhost:8080/add.html` and you will see the page.