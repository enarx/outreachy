# WebAssembly


### As WebAssembly official site define:

WebAssembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine.
Wasm is designed as a portable compilation target for programming languages, enabling deployment 
on the web for client and server applications.


### In other words


WebAssembly is a new type of code that can be run in modern web browsers — it is a low-level assembly-like language with a compact binary format that runs with near-native performance and provides languages such as C/C++, C# and Rust with a compilation target so that they can run on the web. It is also designed to run alongside JavaScript, allowing both to work together.


## How WebAssembly can be used

Entire code base in WebAssembly.

Main frame in WebAssembly, but the UI is in JavaScript / HTML.

Re-use existing code by targeting WebAssembly, embedded in a larger JavaScript / HTML application. This could be anything from 
simple helper libraries, to compute-oriented task offload.



## Why would we want to run WebAssembly outside of a browser?  

For out of browser scenarios, one of its main advantage is that it provides system level access without compromising on security. This is done through WASI, the Web Assembly System Interface. WASI is a collection of C-like functions that provide access to functionality such as fd_read, rand, fd_write, threads (WIP), in a safe way.



## Outside the browser


* Game distribution service (portable and secure).

* Server-side compute of untrusted code.

* Server-side application.

* Hybrid native apps on mobile devices.

* Symmetric computations across multiple nodes


## What is WASI?

WASI stands for WebAssembly System Interface. It's an API designed by the Wasmtime project that provides access to several operating-system-like features, including files and filesystems, Berkeley sockets, clocks, and random numbers, that we'll be proposing for standardization.

It's designed to be independent of browsers, so it doesn't depend on Web APIs or JS, and isn't limited by the need to be compatible with JS. And it has integrated capability-based security, so it extends WebAssembly's characteristic sandboxing to include I/O.