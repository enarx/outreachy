
![[Pasted image 20211031083221.png]]


---


## What is WebAssembly?

Currently, most browsers use JavaScript Engine that can interpret and execute the code. This along with the day-to-day improvemnets in JavaScript, have made development web apps with very rich features very easy. But JavaScript is still a high-level language which is not particularly developed to be a fast language.

This is where WebAssembly or WASM comes into picture. WASM is a relatively novel binary-code format that can be run over the browsers and it is also complemented with WAT file (text file) to make it more readable and debuggable, in addition it can also us to code in kind-of assembly language.

The main purpose behind WASM is to enable high performance applications on web, but it also does not have any web specific assumptions in place thus making it possible to be used in other areas as well.

## How it Works ?


![Alt Text](https://2r4s9p1yi1fa2jd7j43zph8r-wpengine.netdna-ssl.com/files/2018/01/ezgif-5-73711fc5d3.gif)


In order to understand that, first we need to know how the JS engine works behind the scenes : 
Let's see the stepes involved in working of JS engine
In order to compile JS code, engine must do the following things :

1.  The Parser –The parser goes through the code line by line and checks it for valid syntax as well as for the code types. If everything is valid, the Parser creates an Abstract Syntax Tree (AST).
2.  AST to Intermediate Representation (IR) – Then, the engine interpreter takes the AST and turns it into Bytecode, which is an intermediate representation of the code (an IR is an abstraction of machine code).
3.  Compiling the IR to Machine Code – Only then the engine compiler takes the Bytecode and turns it into a code a machine can run on its processor.


Now, let's see How WASM works in comparison :

For a fact, WASM is faster than JS engine  because WASM code goes directly to the compiler, effectively skipping step 1 and 2.

But you might be wondering, why? Why is WASM able to skip steps 1 and 2 and JS not?

The reason for that is because JS is a dynamically-typed language, which means that JS checks the type of variables at run-time by the Parser.

In contrast, statically-typed languages require to declare the types in advance, therefore types are known and are checked at compile time.

In short, following are the steps that WASM goes through to compile the code :

1.  You write code with its types, usually in a statically typed-language.
2.  Then you generate a pre-compiled WASM module.
3.  Then you can run this code straight by the engine compiler, skipping the parsing and the transformation to Intermediate Representation.


## Key Concepts

   ![[Pasted image 20211031082859.png]]

-   Values: WASM provides only 4 value types. These are integers and numbers, each in 32 and 64 bit width. The normal operations on these value types are available.

-   Instructions: Code consists of sequences of intructions that need to be executed in order (Last in First Out: Stack Based). The instructions fall in mainly two categories: Simple, these instructions perform basic operations on data and Complex, these instructions deal with the control flow like calling a function etc. 

	![Alt Text](https://images.hive.blog/0x0/https://cdn.steemitimages.com/DQmZ2dy3Z2nomjjZPKRn8C9TjdaRpteG5MogDQ55RBgZhq5/stack.gif)


-   Functions: Code is organized into separate functions. Each function takes in input values and return a sequence of values as output.

-   Linear Memory : It is contiguous, mutable array of raw bytes. This memory is initially created with a size but can be grown dynamically.

-   Table : A table is an array of values of a particular type. It allows programs to select such values indirectly through a index operand.

-   Embedder: WASM implementation in generally embedded in an environment. This environment decides how modules are loaded, how imports are done etc. Embedder provides details about the particular embedding.

-   Module :A WebAssembly binary takes the form of a module that contains definitions for functions, tables, and linear memories, as well as mutable or immutable _global variables.

       ![[Pasted image 20211031082159.png]]


## Use Cases

WebAssembly’s [high-level goals](https://webassembly.org/docs/high-level-goals/) define _what_ WebAssembly aims to achieve, and in _which order_. _How_ WebAssembly achieves its goals is documented for [Web](https://webassembly.org/docs/web/) and [non-Web](https://webassembly.org/docs/non-web/) platforms. The following is an unordered and incomplete list of applications/domains/computations that would benefit from WebAssembly and are being considered as use cases during the design of WebAssembly.

#### Inside the browser

-   Better execution for languages and toolkits that are currently cross-compiled to the Web (C/C++, GWT, …).
-   Image / video editing.
-   Games:
 -   Peer-to-peer applications (games, collaborative editing, decentralized and centralized).
-   Music applications (streaming, caching).
-   Image recognition.
-   Live video augmentation (e.g. putting hats on people’s heads).
-   VR and augmented reality (very low latency).
-   CAD applications.
-   Scientific visualization and simulation.


#### Outside the browser

-   Game distribution service (portable and secure).
-   Server-side compute of untrusted code.
-   Server-side application.
-   Hybrid native apps on mobile devices.
-   Symmetric computations across multiple nodes

## Features at a Glance

### Security

   ![Alt Text](https://marcoselvatici.github.io/WASM_tutorial/ref/sandbox.gif)
   
   WebAssembly modules are executed in a sandbox environment that is entirely separate from the host, and separate from the execution environment of other modules. This is a critical feature for browser-based applications, browsers themselves have a similar sandbox which prevents browser-based applications from accessing the host environment (in this case the user’s operating system). This is also a desirable feature in many other contexts where executing processes share the same underlying physical resources, e.g. cloud computing.
   

### Language Independance
           
 The WebAssembly specification was designed with a range of languages in mind. Furthermore, WebAssembly is vendor agnostic, having been developed in collaboration by many vendors and technologists. 


### Portability

 WebAssembly is portable. Its opcodes are processor and operating system agnostic. This is a frequently underrated aspect of wasm, especially by developers assuming wasm is a browser-first technology.Any valid WebAssembly module should be able to be deployed and run anywhere, on any architecture so long as the host is a valid WebAssembly runtime. 

### Simplicity

  WebAssembly is a simple and low-level runtime, it doesn’t have a specific approach to memory management (e.g. garbage collection), or its own system-level APIs. This has resulted in a runtime that is both simple and lightweight.


### Speed

  ![[Pasted image 20211031094952.png]]
  
   WebAssembly was primarily designed for delivery over the web, where the initial download and compilation time is just as important as the eventual runtime performance. Runtime features such as streaming compilation and fast/simple validation rules all contribute to fast start times and near-native performance.



## WebAssembly with serverless...

![[Pasted image 20211031095144.png]]

Serverless is a relative newcomer, with a very simple promise - you relinquish control of everything to the cloud provider other than your application code itself. In other words, you deploy your code (functions) to the cloud and they take care of provisioning a suitable runtime.

In order for a cloud provider to execute serverless functions, it still needs to provision hardware, operating systems and suitable runtime environments. As a result, there is some significant overhead to servicing requests. This provisioning time is visible to the consumer as a noticeable delay in response when additional resources are required to service a request - an effect known as a ‘cold start’.

Security and isolation are of critical importance to any cloud-based service, which is what makes WebAssembly an interesting prospect for serverless computing. The lightweight WebAssembly runtime, which has isolation/sandboxing built in, has the potential to be a very efficient approach to serverless.
