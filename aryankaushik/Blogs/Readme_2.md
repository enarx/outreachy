# WebAssembly x Rust, a Deadly Combination!

![WebAssembly x Rust](https://github.com/aryankaushik-git/outreachy/blob/main/aryankaushik/media/WebAssembly%20.gif)

Welcome to Another Blog, Today we will learn about why WebAssembly & Rust is considered a Deadly Combination!

Firstly, Rust isn't doing a singular thing differently than any other programming language, but rather the way the whole package comes together creates an amazing and unique solution! 

Rust has an impressive set of features that can be categorized into four categories:

1. Superb performance
2. Great safety
3. Easy tooling
4. A welcoming community


### Why Wasm and Rust make so much sense!

#### Efficiency:
Additionally, Rust with Wasm has the advantage of fast execution and small binary size. These two languages are comparable to or even a little better than C and C++, which are garbage-collected languages. However, even a relatively small runtime language like Go has a hello world binary size of 2MB after being compiled to .wasm.

#### Low-Level Control with High-Level Ergonomics:
Applications written in JavaScript have difficulty attaining and retaining reliable performance. Garbage collection pauses and JavaScript's dynamic type system are ineffective. Even seemingly minor changes could result in drastic performance regressions if you accidentally stray from the JIT's happy path.
With Rust, programmers have low-level control and reliable performance. There are no garbage collection pauses, as there are in JavaScript. Direction, monomorphism, and memory layout can be controlled by programmers.

#### Small .wasm Sizes:
As the .wasm must be downloaded over the network, code size is extremely important. The absence of a runtime enables small .wasm sizes since there is no extra bloat, such as a garbage collector. Your code size is only affected by the functions you use.

#### No Need to Rewrite Everything:
We don't have to get rid of existing codebases. To start, you can port your most performance-sensitive JavaScript functions to Rust and you can even stop there if you want to.

#### A Collaborative Nature:
Rust and WebAssembly integrate with existing JavaScript tooling. It supports ECMAScript modules and you can continue using the tooling you already love, like npm and Webpack.

#### Conclusion:
With Rust and Wasm's extensive tooling, excellent efficiency, and portability across a wide spectrum of hardware and software environments, the pair can form a formidable combination. 
