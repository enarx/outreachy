# Why Wasm is the perfect runtime for server-side applications.

![1](https://github.com/aryankaushik-git/outreachy/blob/main/aryankaushik/media/24.png)

Namaskar, welcome to another fantastic blog, previously we've tried to work with wasm on the user/client-side and learned its advantages but today in this blog we will try to understand the broader applications and advantages of Wasm in the tech industry i.e. using it on the server-side, it is much more functional than we've thought.

Every two years, the number of transistors on a microchip doubles e, though the cost of computers is halved. Moore's Law states that we can expect the speed and capability of our computers to increase every couple of years, and we will pay less for them. Another tenet of Moore's Law asserts that this growth is exponential. The machine learning tasks can run 60,000x times faster when we replace Python with native code and specialized hardware. WebAssembly is a key technology that makes the software more efficient, while preserving safety, portability, and software engineering best practices we built up in the past 25 years.

![Moore](https://www.wasm.builders/remoteimages/uploads/articles/zwez9z19edczvpj6havk.png)
 
WebAssembly was invented as a client-side technology, but it is also proven very useful on the server-side. Server-side WebAssembly provides crucial benefits for modern web and service applications. 

**Fast:** WebAssembly achieves near-native performance. Compared with the Java, Python, or JavaScript runtimes, it can be 10x to 100x faster (how is this possible?). It is also much faster than Docker, especially in the cold start and system access.

**Safe:** WebAssembly is a sandbox with a capability-based security model. It is not only safer than native binaries but also safer than OS-level containers like Docker. Yet it provides access to the underlying system, including new hardware features.

**Portable:** WebAssembly apps can be written in C, C++, Rust, Go, and run without change on different OS and hardware platforms.

**Manageable:** WebAssembly programs can be provisioned, started, hot-swapped, stopped, and moved around by other applications.

To run WebAssembly outside of the browser, You need a runtime. A runtime is an application that runs on the command line or via a static library that allows you to load your Wasm module and run it as if it was just any other program. SSVM, Wasmer, WasmTime, Wasm3, WAVM, and Lucet, are some common emerging Wasm runtimes that have been coming out from the community as of late.

**SSVM:** Second State provides an open-source WebAssembly implementation (Second State Virtual Machine, or SSVM) that is specifically optimized for server-side applications. It is

Best-in-class in performance. It is 1000x faster than Docker for cold starts.
Seamlessly supports server application frameworks, such as Node.js. You can build high-performance Node.js apps with SSVM.
Supports safe access to external resources, such as databases, message queues, and even new AI hardware
Allows precise metering of computational resources for serverless apps.

**Wasmer:** Wasmer is designed to provide three key features:
Enable programs to run in any programming language
Enable extremely portable binaries to run unmodified on any "OS" supported by Wasmer (for example, Linux, macOS, Windows and FreeBSD).
Act as a secure bridge for Wasm modules to interact with native "OS" functionality, via Application Binary Interfaces (ABIs) such as WASI and Emscripten (version 1.38.43 and earlier).

In a blog written by [Libhunt](https://www.libhunt.com/compare-SSVM-vs-wasmtime), they compared SSVM & Wasmer

![1](https://www.wasm.builders/remoteimages/uploads/articles/np4xpnxyxw7r3l3a8rrx.png)

#### Emerging Tech Stacks where Wasmer is making a Significant Difference:

**a) Blockchain:** Wasmer's Single-pass compiler helps eliminate JIT bombs, enabling blockchain providers with a high-quality-of-service infrastructure for their applications.

**b) Function-as-a-Service (Faas):** Wasmer is a market leader for enabling WebAssembly on the server. It nearly eliminates all the challenges developers must deal with to fully adopt FaaS as a part of their application development strategy.

**c) Machine Learning and AI:** Wasmer nearly eliminates all the challenges developers must deal with for deploying machine learning applications in and outside of data center environments.

**SSVM** is also running with pace to make its runtime more efficient and progressing exponentially, while other WebAssembly runtimes are in the development stage and are making progress day by day. Soon we will see a lot of efficient Wasm runtimes on the server-side.


**Conclusion:** 
Lately in the server-side development community, which is the desire to simplify, and is coming in the form of things like Functions as a Service, or serverless technology. There's AWS Lambda, OpenFaaS the serverless framework, things like that are really designed to make things simpler, again, because things have gotten much more complex in the last 5 or 10 years, with the rise of microservices and whatnot. It's made things more difficult. 
Wasm can really help with this goal by building these very tightly constrained, highly composable modules from various languages, really whatever you want, and fitting them together into whatever configuration that you need for your application, is pretty compelling.

Check this [Blog](https://www.infoq.com/presentations/wasm-server-api/) by @cohix where he explains Server-Side WASM: Today and Tomorrow!

References:
1. https://www.infoq.com/presentations/wasm-server-api/
2. https://www.libhunt.com/compare-SSVM-vs-wasmer
3. https://www.secondstate.io/articles/why-webassembly-server/
4. https://docs.wasmer.io/ecosystem/wasmer
5. https://noti.st/lostinbrittany/hsL7QE
6. https://webassembly.org/docs/use-cases/

Do comment your thoughts and suggestions related to the blog and please share if you found it useful.



