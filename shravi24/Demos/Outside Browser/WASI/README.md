<h1> What is WASI ? </h1>

 <img src ="../../../images/Pasted image 20211102130951.png">

 WebAssembly needs a system interface so that it can be run across all different OSs. This system interface is called WASI. It allows to run WebAssembly outside the web as well as inside it.To run Wasm outside the web using WASI, standalone Wasm runtimes can work as interpreters / “hosts.” 
 
 Previously We used Emscripten tool for producing WebAssembly which uses OS system interface called POSIX, on the web. POSIX provides source code portability. You can compile the same source code with different versions of libc to target different machines.

But we need to have something due to which we can compile once and run across a whole bunch of different machines which means that we need portable binaries.

<img src ="../../../images/Pasted image 20211102141013.png">

Another important principle is Security. WebAssembly’s way of doing security is different. WebAssembly is sandboxed.This means that code can’t talk directly to the OS.This means that the host can limit what a program can do on a program-by-program basis.Each host can have their own implementation of wasi-core that is specifically written for their platform. WASI Core is  an API module that covers files, networking, clocks, and random numbers.



 
With WASI, if you’re calling a function that needs to access a file, you have to pass in a file descriptor, which has permissions attached to it. This happens on a module-by-module basis. By default, a module doesn’t have any access to file descriptors.The module can only access the exact resources it needs to do its job which provides least privilege.


<h1> WASI Software Architecture </h1>

<img src ="../../../images/Pasted image 20211102132312.png">

To facilitate use of the WASI API, a libc implementation called WASI libc is being developed, which denotes the top half musl based libc. The system call wrapper layer makes calls to the actual WASI implementation, which maps the calls to whatever the surrounding environment provides, whether it's native OS resources, JS runtime resources, or something else entirely.[This libc is part of a "sysroot"](https://github.com/WebAssembly/reference-sysroot), which is a directory containing compiled libraries and C/C++ header files providing standard library and related facilities laid out in a standard way to allow compilers to use it directly.


<h1> Why is it so exciting? </h1>

 Let’s explore this from a high level so you can get an idea of just why WASI is so exciting. Here are a few of the possible ways WASI could manifest :

#### 1) Cross-platform apps/games

The dream here is [cross-platform games](https://blog.logrocket.com/first-game-in-webassembly/) and applications from a single file, also harnessing the [power of PlayOS](https://medium.com/rutile/wasi-the-new-way-of-writing-apps-rutile-news-32b51847cba7). The mechanism behind this: one binary executable capable of being realized on any platform containing the WebAssembly runtime — communities building libraries that map these, with WebAssembly itself doing the foundational work.

#### 2) Reusing source code

The benefit here is obvious. You could reuse source code across various platforms for your application architecture — for instance, mobile/desktop, server and client, and even IoT.

#### 3) Single runtime able to execute WASI-friendly (Wasm) applications

This reduces compiling demands enormously. No need to stitch together different languages and their separate runtimes — one runtime capable of realizing every one!

#### 4) Grouping many targets as one target

This is really about containerization — a compilation of many applications and their dependencies into one or few Wasm files. Everything is Wasm-led, boosting usability and reducing the need for finicky containers.


<h1> How can I run programs that use WASI? </h1>

You can use Wasmtime. [For more information](https://github.com/enarx/outreachy/tree/main/shravi24/Demos/Outside%20Browser/Rust) 


<h1> Where can I learn more? </h1>

 Take a look at the various [WASI documents](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-documents.md).