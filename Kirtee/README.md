# Enarx With Webassembly.

[Enarx](https://enarx.dev/) Cloud Native, Hardware Neutral and provides runtime TEE(Trusted Execution Environments), based on a [WebAssembly](https://webassembly.org/) and [wasmtime](https://wasmtime.dev/), allowing developers to deploy applications without any rewrites from languages like Rust, C/C++, C#, Go, Java, Python, Haskell, Grain and many more.

In this documentation i'll be demonstrating how webassembly is used to compile high level languages so mentioned above  and later how to run them on Enarx.

## Why WebAssembly?

The question instead could have been why not webassembly!</br>
WebAssembly (or Wasm) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable target for compilation of high-level languages.
All major browsers support Wasm. The benefits of WebAssembly include:
- **Fast, efficient and portable:** WebAssembly code can be executed at near-native speed across different platforms.
- **Readable and debuggable:** WebAssembly is a low-level assembly language, but it has a human-readable text format
- **Secure:** WebAssembly is specified to be run in a safe, sandboxed execution environment.
- **Part of the open web platform:** It is designed to maintain the versionless, feature-tested, and backwards-compatible nature of the web.
WebAssembly is an exciting technology for deploying highly secure, performant, and portable code.

## contents
### Understanding WASM
 - [WebAssembly: New Capability to Web](https://github.com/kirteeprajapati/outreachy/blob/main/Kirtee/Understanding%20WASM/WebAssembly:%20New%20Capability%20to%20Web.md)
 - [Webassembly JavaScript Inter-Operability](https://github.com/kirteeprajapati/outreachy/blob/main/Kirtee/Understanding%20WASM/Webassembly%20JavaScript%20Inter-Operability.md)

### Hands On Wasm
 - [WebAssembly with Grain](https://github.com/kirteeprajapati/outreachy/tree/main/Kirtee/Hands%20On%20WASM/WebAssembly%20with%20Grain)
 - [WebAssembly with C++](https://github.com/kirteeprajapati/outreachy/tree/main/Kirtee/Hands%20On%20WASM/WebAssembly%20with%20C%20and%20C%2B%2B)
 - [WebAssembly Emscripten C and C++ on browser](#).
 - [WebAssembly with RUST](https://github.com/kirteeprajapati/outreachy/tree/main/Kirtee/Hands%20On%20WASM/WebAssembly%20with%20Rust) </br>
     -[Rust with Wasm-bindgen Intro]()</br>
     -[Rust with Wasm-Bindgen Example]()</br>
