# Webassembly JavaScript Inter-Operability

In [My last post](https://www.wasm.builders/kirteeprajapati/webassembly-new-capability-to-web-2oke) , I provided an introduction to **WebAssembly**, How and where it can be implemented, 

In this post I’ll be trying to explore the depth of its inner working in collaboration with Javascript. 
I'll be answering the following question
> *  Why there was a need for WebAssembly? or in what fields does JavaScript lacking behind !.
> * How WebAssembly fills the gap and their Working together.

First of all a quick recap about WebAssembly and [Made with webassembly application](https://madewithwebassembly.com/) a whole lot list of it.

## **WebAssembly:** 
*Efficient safe, low-level bytecode for the web standard binary instruction format for the web.* A perfect definition for it.

- Runs with near-native performance and provides languages with low-level memory models such as C++ and Rust with a compilation target so that they can run on the web. 
- It is not primarily intended to be written by hand, rather it is designed to be an effective compilation target for source languages like C, C++, Rust, etc.
- WebAssembly defines a .wasm binary executable format that can be downloaded and run in the browser (like a .js file) while taking advantage of the speed and low-level hardware capabilities of machine code. 
WebAssembly has been implemented In all the browsers, they literally use the existing virtual machine they have because it's been hardened over the years and it's Very fast and very powerful.

There is a really quick quote from **Ben Smith, the Chrome team** 
``` bash
WebAssembly Shortcut to your JavaScript engine's optimizer
```
Because it is a strictly tight bytecode it can go straight to the optimized and doesn't need to do all these intermediate just-in-time compilations.

**WASM Module:** the basic unit of WebAssembly code in binary that has been compiled by the browser into executable machine code. 

A Module is stateless and thus, like a [Blob](https://developer.mozilla.org/en-US/docs/Web/API/Blob), can be explicitly shared between windows and workers. declares imports and exports symmetric in many ways to ES2015 modules.

## Javascript and its advantages.
It does not require any introduction for sure.

Javascript used to be an interpreted language, it is JIT-compiled to native machine code in all major JavaScript implementations.
It is dynamically typed, requires no compile step, and has a huge ecosystem that provides powerful frameworks, libraries, and other tools.


## Where JavaScript was lacking behind.
The workflow of JavaScript when compiled to V8. 

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/wq3h0f33px8fh0ttdym1.png)
 
This makes JavaScript a bit slow for CPU-intensive tasks *Ex. JSON decoding or Cryptographic things.*

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/1akntr09ttie3zrwnwee.png)
 
JS is parsed, compiled, and optimized before execution. Whereas WASM is already in a binary format which is decoded and compiled and makes execution faster. 

**Performance problems:** for more intensive use cases like 3D games, Virtual and Augmented Reality, computer vision, image/video editing, and a number of other domains that demand native performance.

**The cost of downloading, parsing, and compiling very large** JavaScript applications can be prohibitive.  Mobile and other resource-constrained platforms can further amplify these performance bottlenecks.

## How Webassembly filles that gap:

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/cuqu25qzdiaqv3nho2pe.png)
 
WebAssembly format was designed to be as fast as possible for the browser to parse. This isn’t true for JavaScript syntax, which was designed for humans and contains lots of redundancy and extra rules that must be checked before it can be run. 

1. WebAssembly parses around **20x faster than asm.js.**

2. WebAssembly has **native support for 64-bit integers.** JavaScript only has 64-bit floating-point numbers so it only supports 53-bit integers. 64-bit integers have to be emulated in JavaScript, which can be much slower.

3. It’s trivial for browsers to cache the translation of a WebAssembly module to native code. This means the second time you load a page using a WebAssembly module, there’s virtually **no load time at all!** This isn’t true for asm.js, which is mixed in with regular JavaScript and requires a complex verification pass to validate that it actually **follows the asm.js restrictions.** 

4. **Typed Stacked Machine / Limited Virtual registers:** At every point in the time program, the type of the tech slot is known. You can use locals to access values you don't wanna necessarily store on the stack.
``` 
i32.const 7         (; declares const 2 and pushes it to stack ;)
i32.const 2          (; declares const 2 and pushes it to stack ;)
i32.add                 (; adds the last 2 values from the stack and pushes back the result onto the stack ;)
``` 
5. **Validated before execution:** Since it is statically typed enabling CPU can validate the entire binary. This is necessary to ensure that the untrusted code from the web running in the browser isn’t going to do anything nasty.
Machine-verified formal specification: Provides fairly certainty that in whatever environment they are running the code will result same.

6. **Harvard Architecture:** separate storage and signal pathways for instructions and data. there is no need to make the two memories share characteristics.

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/jzo25tert9rszpt9joa0.png)
 
7. **Structured control flow:** It doesn’t have a go-to statement/instruction instead it has a [Block](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/block). This makes the code a little bit smaller, important when send over the web.

8. **Streaming execution manner:** Wasm is designed like this and can start executing and getting converted to machine code soon after its first byte gets downloaded.

## How to implement wasm in web apps?

There are many ways to do that some are mentioned below and a lot are under development.

- **[Emscripten-](https://emscripten.org/docs/introducing_emscripten/index.html)** compiles high-level languages and runs them to browsers, also capable of converting OpenGL to WebGL.

- **[AssemblyScript -](https://www.assemblyscript.org/)** language similar to typescript/javascript but compiles to WebAssembly. We can easily start a new Assembly script project using node.js and npm.

- **[Blazor WebAssembly-](https://dotnet.microsoft.com/en-us/apps/aspnet/web-apps/blazor)**  is a client-side in-browser implementation of Blazor which includes a .NET runtime and C# instead of JavaScript implemented in WebAssembly. We can easily create a single-page web app on the full-stack side too.

- **[Kotlin-Native-](https://stackoverflow.com/questions/55407468/whats-the-difference-between-kotlin-jvm-and-kotlin-native)** for compiling Kotlin code to native binaries can be run without a Virtual machine.

- **[Binaryen -](https://github.com/WebAssembly/binaryen)** Written in C++, Binaryen, can be used from JavaScript.

- **[Pyodide-](https://github.com/pyodide/pyodide)** recently moved from [Mozilla to become an independent project,](https://www.infoworld.com/article/3616629/mozilla-spins-out-pyodide-python-in-the-browser-project.html)  compiles Python and the Python scientific stack to WebAssembly, bringing the Python 3.8 runtime, NumPy, SciPy, Matplotlib, Scikit-learn, and dozens of other packages to the browser.

###### We’ll be discussing about this in detail in my future blogs
 
## How they(JS & WASM) work together.

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/muw15middf3verqkhum8.png)

The binary format `.wasm`  is executed by a virtual machine that works **alongside the JavaScript VM**, sharing resources (for example, memory) and executing on the **same thread**.

- .wasm is designed to execute in a sandboxed environment and it adheres to the same security constraints as JavaScript does in the browser. 

## asm.js before .wasm
- Before WebAssembly, C++ code could be run in the browser by cross-compiling it to a subset of JavaScript known as asm.js.
 
> The **asm.js** subset is basically JavaScript where you can only use numbers (no strings, objects, etc.). This is all you need to run C++ code since everything in C++ is either a number or a pointer to a number, and pointers are also numbers. The C++ address space is just a giant JavaScript array of numbers and pointers are just indices in that array.

- Since it can only load and store numbers, it needs to call out JavaScript code (create DOM nodes, make network connections, etc). WebAssembly code is still inside the browser sandbox and can only use the browser APIs that JavaScript has access to.

- WebAssembly modules can be imported **Non-Web environments** may include JavaScript VMs (e.g. node.js) apps, exposing it is also capable of being executed without a JavaScript VM.

- The [WebAssembly JavaScript API](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly)  wraps exported WebAssembly code with JavaScript functions that can be called normally, and WebAssembly code can import and synchronously call normal JavaScript functions.

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/2f89njx9el7vnegtfz5l.png)
 
By itself, **WebAssembly cannot currently directly access the DOM**, it can only call JavaScript, passing in integer and floating-point primitive data types. 

To access any Web API, WebAssembly needs to call out JavaScript, which then makes the Web API call. 

Emscripten, therefore, creates the HTML and JavaScript glue code needed to achieve this.

> Glue Code: executable code (often source code) that serves solely to "adapt" different parts of code that would otherwise be incompatible. Glue code does not contribute any functionality towards meeting program requirements.

## Conclusion: 

**More CPU intensive operation** that requires more power and Computation goes to **WebAssembly**

**Web-facing UI front** the DOM, Network access, memory management, and communicating to hardware goes to **JavaScript.**

### References:

1. **[WebAssembly cut Figma's load time by 3x](https://www.figma.com/blog/webassembly-cut-figmas-load-time-by-3x/)**

2. **[webassembly.org](https://webassembly.org/getting-started/developers-guide/)**

3. **[Mozilla WebAssembly Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly)**

4. **[Emscripten Documentation](https://emscripten.org/docs/porting/guidelines/index.html)**

5. **[Glue-Code](https://en.wikipedia.org/wiki/Glue_code)**

6. **[Why WebAssembly is Faster Than asm.js
By Alon Zakai](https://hacks.mozilla.org/2017/03/why-webassembly-is-faster-than-asm-js/)**
