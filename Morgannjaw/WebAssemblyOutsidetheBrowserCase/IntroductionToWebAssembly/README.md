# What is Web Assembly and how does it work?

Most WebAssembly tutorials and examples you will find online focus on using WebAssembly inside the browser in order to accelerate various functionality of a website or web app.
However, in this tutorial we’re going to focus on another none less important area of application “ outside the browser”.

### What is Web Assembly?

Web Assembly, also known as Wasm is an open standard, or code format that allows the execution of binary code on a web browser. Web Assembly is more of an improvement for JS and not a replacement.  It is designed to make large applications run on a browser with near native performance and will run native code such as C/C++ and Rust (things which will interact with the computer but on a browser and helps the browser understand binary format). 
Web assembly defines an Abstract Syntax Tree (AST) (i.e., a way to parse C/C++ into binary) that gets stored in a binary format which can then create smaller app bundles. The AST will be represented in a friendly text format. 


## Getting Started with Web Assembly.

###	Basic Concepts.

Before we get into more detail about Web Assembly, we need to familiarize ourselves with some basic concepts to understand how it runs in and out of the browser. We shall discuss about Modules and Stack Machines.

#### Modules.
Web Assembly is organized into modules which are compiled in binary by the browser (or virtual machine). Each module can contain functions and can import and export other functions. The code below illustrates a simple module which takes two 32-bit parameters, add them up and return the function.
```
(module
    (func $add (param $lhs i32) (param $rhs i32) (result i32)
        (i32.add
            (get_local $lhs)
            (get_local $rhs)
        )
    )
    (export "add" (func $add))
)
```
 
#### Stack Machine.

Web Assembly is a stack machine because it works under a system of instructions (ISA- Instruction Set Architecture). These instructions allow the control of the loops, arithmetic operations, and memory access. 
In a stack machine, most of the instructions assume that the operands are sitting on the stack, rather than stored in specified registers. The WebAssembly stack is a LIFO (Last In, First Out) stack. If you’re unfamiliar with the concept of a stack: it is as its name implies— values are piled (stacked) on top of each other, and unlike arrays where you can access any data regardless of location in the pile, stacks only allow you to pop data off or push data onto the top.
To add two numbers in a stack machine, you push those numbers onto the top of the stack. Then you push the ADD instruction onto the stack. The two operands and the instruction are then popped off the top and the result of the addition is pushed on in their place

For example, in the code above.

-	```get_local $lhs```  means gets the first parameter that was passed into this function and 
-	```get_local $rhs``` gets the value of b and place on top of a in the stack. 
-	The function ```i32.add```, receives two 32-bit integers and returns the result of their addition.


## Why would we want to run WebAssembly outside of a browser?

The main advantages of running WebAssembly outside the browser is that it provides system level access without compromising on security of its host. This is done through WASI (the Web Assembly System Interface). WASI is a collection of C-like functions that provide access to functionality such as ```fd_read, rand, fd_write```, threads (WIP), in a safe way.In addition, embedding WebAssembly in applications is important for the following reasons. First, WebAssembly provides secure extensibility mechanisms for applications. Second, application and Wasm modules communicate through bi-directional ABI and API. Lastly, WebAssembly allows users to install extensions securely, portably, and in any language.


- [See Rust Example](../BasicExampleofWebAssemblyUsingRust)

### References: 

1. Fullstack Labs. Luis Hernandez [What is WebAssembly And What is it Used for?](https://www.fullstacklabs.co/blog/what-is-webassembly)
2.	Eric Elliot, “What is web assembly?” [What is WebAssembly - The dawn of a new Era](https://medium.com/javascript-scene/what-is-webassembly-the-dawn-of-a-new-era-61256ec5a8f6) June 19 2015.
3. The PragKevin Hoffman, “ Programming WebAssembly with Rust”, March 5 2019.
4. Tetrate, [Why WebAssembly is innovative even outside the browser](https://www.tetrate.io/blog/wasm-outside-the-browser/), July 16, 2021. 
