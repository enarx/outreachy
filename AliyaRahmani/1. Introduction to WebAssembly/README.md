# Basics of WebAssembly

### Introduction
Javascript has been popularly used as a web-running programming language. But due to some drawbacks and limitations, it cannot be used in many areas. For this purpose, WebAssembly is created. It is a low-level binary format language for the web i.e. compatible with the web and runnable in modern web browsers. The file size generated is small thus loads and executes faster.
WebAssembly defines an Abstract Syntax Tree (AST) that gets stored in a binary format for a stack-based virtual machine. Binary is great because it helps to create smaller app bundles. It is often abbreviated as Wasm. When we run WebAssembly code in the browser, the browser converts it into machine format so that the processors can understand. One of the advantages of WebAssembly over javascript is that in javascript, the code has to be downloaded, parsed, and converted to machine format. A lot of time goes into it and heavy tasks like, we mentioned earlier can be very slow.
WebAssembly is much more secure in a sandbox environment, it doesn’t have access to file systems and doesn’t lead to security issues. It has a lot of scope in the future for the aspects of a container like Docker etc. WebAssembly is an open standard, which was created with certain objectives including:
* Be executed at near-native speeds.
* Be readable and easily debuggable.
* Be secure.
* Don’t break the web.
### How WebAssembly work?
1. You write code with its types, usually in a statically typed language.
2. Then you generate a pre-compiled WASM module.
3. Then you can run this code straight by the engine compiler, skipping the parsing and the transformation to Intermediate Representation.
![Workflow](https://github.com/aliya-rahmani/aliya-rahmani/blob/main/Untitled%20Jam%201.png)


### WebAssembly Key Concepts
There are several key concepts needed to understand how WebAssembly runs in the browser, which are:
* Module: Represents a WebAssembly binary that has been compiled by the browser into executable machine code. A module collects definitions for types, functions, tables, memories, and globals. A Module declares imports and exports just like an ES2015 module.
* Memory: A resizable ArrayBuffer that contains the linear array of bytes read and written by WebAssembly’s low-level memory access instructions.
* Table: A resizable typed array of references (e.g. to functions) that could not otherwise be stored as raw bytes in Memory (for safety and portability reasons).
* Instance: A Module paired with all the states it uses at runtime including a Memory, Table, and set of imported values. An Instance is like an ES2015 module that has been loaded into a particular global with a specific set of imports.
### .WASM and .WAT formats(WebAssembly Text Format)
WebAssembly supports both binary and text formats. The binary format (.wasm) is a stack-based virtual machine’s compact binary instruction format, designed to be a portable compilation target for various higher-level languages like C, C++, Rust, C#, Go, Python, and others. The text format (.wat) is a human-readable format for viewing the code of a WebAssembly module by developers. Codes that can be compiled into binary format can also be written in text format. The code below shows an example of the .wat format for printing helloWorld:
```
(module
  (func (result i32)
    (i32.const 42)
  )
  (export "helloWorld" (func 0))
)
```
