Imagine a world where you can build software with C++, Rust, Python, Go or Grain then delivers that software to the end-user in a web browser without any installation and near-native performance. That world became a reality in Dec 2019 when WebAssembly became an official [W3C (World Wide Web Consortium)](https://www.w3.org/2019/12/pressrelease-wasm-rec.html.en) Standard.

- Following HTML, CSS, and JavaScript, WebAssembly becomes the fourth language for the Web which allows code to run in the browser.

- Open standard with cross browsers supports and backed by Mozilla, Microsoft, Apple, and Google.

-  Supported by Chrome, Safari, Firefox, and [Edge since 2017](https://webassembly.org/roadmap/), almost [90% of devices worldwide](https://caniuse.com/wasm).

**WebAssembly** Will change the way we things of **"Web apps"**

![App Screenshot](https://blog.qburst.com/wp-content/uploads/2019/01/WebAssembly-featured-image.jpg)

[WebAssembly](https://webassembly.org/) a low-level assembly-like language with a compact binary format, is not really a new programming language or more suitably **Compiler target**.

We can't run high-level languages ( C/C++/C#/Rust/Go .. ) directly into the browser but rather can via WebAssembly, it compiles high-level language into WebAssembly module (WASM module), load that into modern web browsers, delivering a performant run-time, builds high performance compiled languages and runs it inside your web application, 
It also brings lots of other benefits of native compilation to the web and calls it for javascript.

All the WASM module information is encoded in a dense binary format which means it has 

``` Smaller Binary sizes --> Faster Loading --> Faster Decoding. ```

It's a smaller memory footprint, just fewer bits over the wire so everything is faster.

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/y4ehkuw68jrttq8fzj1l.png)

The browser doesn't allow running arbitrary code without any sandbox or may any other functionality, and here comes WebAssembly. 

## What does it enables you to do?

Write an application for the web in languages other than javascript

1. **Speed:** Fast to **load** and **execute**  (Fast to transport the compiled files over the internet to the person's browser and it's also fast to compile because they run inside a virtual machine and once it's done parse it is still fast to execute and, it's Efficient primarily because it is a low-level bytecode(hexadecimal-1 byte) ) 

2. **Secure:** It's very secure, it was specifically designed to be safer than javascript, and the instruction is very restricted, on top of that it runs inside a chrome sandbox so there are layers of protection.

3. **Portability:** The bigger goal is portability, it compiles once and runs across a whole lot of browsers, we can add functionality without changing browsers, Just like javascript it doesn't have to deal with memory overflow exploits and things.

There are many different ways to build a WebAssembly app, many are under development, one of the most popular tools is Emscripten converts a C or C++ to WebAssembly, 

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/5xvfacna7j0lvnjpvoxn.png)

It's **not limited to desktop** it's a mobile thing too. Safari and IOs 11 as well as Chrome and Android both support WebAssembly, we can build the highly performant application using native code for mobile devices as well.
> Which of course means faster execution time and less battery usage 

**Webassemly Shines with computation:** WebAssembly focuses on Heavily CPU-bound number Computations that deal with math, maybe 32-bit 64-bit, or any. 

The most common thing related to numbers is **Games**, there are tones of Pixel stuff and lots of these are going to be heavily OpenGL and so it enables you to write your app in C++ or Rust using the OpenGL, compile that to web assembly and it will run in a browser.


## The .wasm file is in binary then how you gonna debugg it!!
There's a [**Textual Representation**](https://www.bitfalter.com/webassembly-compiler-text-format-and-ast) to rescue! 
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/uzcht02ffos7edzlr8yi.png)

The WebAssembly text format is a textual representation of the Wasm byte code, just like x86 assembly is for Intel or AMD byte code. Most tooling supports are Abstract Syntax Tree (AST) 

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/8x5ddmx1j6qjezp6sa4f.png)
 
At a first glance, it seems that there is not much going on, but from a parser's perspective, there is a lot of work to do to extract an AST from the code.

you probably would be wandering

# Why web Apps?

Web apps have numerous benefits to be mentioned.
- Allowing multiple users access to the same version of an application.
- Web applications do not need to be downloaded since they are accessible through any network.
- Web apps can be run through various platforms such as a desktop, laptop, or mobile.
- Can be achieved through multiple browsers.

WebAssembly has found the most success as a mechanism for bringing mature and substantial C/C++ codebases to the web.

### Best examples: 
### 1. Figma

[WebAssembly cut Figma's load time by 3x](https://www.figma.com/blog/webassembly-cut-figmas-load-time-by-3x/#:~:text=Because%20our%20product%20is%20written,that%20supports%20very%20large%20documents.) 

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/ug67bzid625c0nqs95kb.png)
 
It’s a browser-based interface design excellent tool for designing animations with a powerful **2D WebGL** rendering engine that supports very large documents. 

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/o63uw1je9nw89re5qgcr.png)

It uses **ReactJS for its outer UI** and has high performance **C++ design tool**, which can easily be compiled into WebAssembly. create desktop-quality experiences on the web without compromising on performance.


### 2. FFmpeg

> FFmpeg is a 20-year-old project!

FFmpeg.wasm is a pure WebAssembly / JavaScript port of FFmpeg. It enables video & audio recording, conversation, and stream right inside browsers.
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/piwcoxhffvt3h3qrae07.png)

Prior to WebAssembly, it would be almost inconceivable to create a JavaScript port of this library, the effort involved would be colossal! Furthermore, the performance characteristics of JavaScript might limit the effectiveness of this approach.
FFmpeg.wasm is composed of a thin JavaScript API layer and a more substantial (20MByte!) WebAssembly binary.

These are Hybrid apps: these apps work as a Web app and can be installed on the device as a native app too, which enables it to take advantage of device-specific resources by using internal APIs. it typically shares similar navigation elements as a Web app since they are based on it.
 

### 3. Google Earth

> 14 years ago Earth was released as a native application 
because rendering the whole world in real-time required advanced technologies that weren’t available in the browser. 

First came to the Web about two years ago using Native Client (NaCl), a Chrome-only solution—at the time.

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/3dlq7poor2emk7w186d6.png)

But now Google Earth is accessible on Firefox, Edge, and Opera browsers. This was made possible by moving Google Earth for Chrome onto **WebAssembly (Wasm)**, not just making apps more accessible across browsers, but smoothing out the online experience too.


### 4. TensorFlow

TensorFlow.js provides a WebAssembly (WASM) backend for both the browser and Node.js! 

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/ahmhwmsb6crs9yv482x3.jpg)
 
This backend is an alternative to the WebGL backend, bringing fast CPU execution with minimal code changes. 

It's a really exciting technology not only for the browsers vendors but other large companies working with WebAssembly. 

- Unity, and Epic’s Unreal Engine,
- AutoCAD Moving a 30 Year Code Base to the Web
