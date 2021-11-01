
<h1>WASM Runtimes</h1>

## Wasmer

Version 1.0 of [Wasmer](https://blog.logrocket.com/how-to-debug-wasm-and-achieve-a-reliable-stack-trace/) was just released in January 2021 with a stabilized API and easier usage. It offers exceedingly lightweight containers executable from anywhere. Wasmer is embeddable inside any programming language, acting as a library you embed in whatever language; truly use WebAssembly wherever.

 Wasmer features:

-   **Pluggability**: Compatible with the various compilation frameworks, whatever you need (e.g., Cranelift)
-   **Speed/safety**: Able to run Wasm at nearly _native speed_ in a completely sandboxed setting.
-   **Universality**: Works on any platform (Windows, Linux, and so on) and chipset
-   **Support**: Compliant with WebAssembly test suite standards with a large base of developer and contributor community support

[Install Wasmer](https://wasmer.io/)

---

## Wasmtime

Wasmtime is an efficient, compact solution for working with the latest WASI/WebAssembly innovations. If you’re using the following languages, embed Wastime (can also be hosted by the Bytecode Alliance): Rust, Python, C, .NET, and Go.

Wasmtime features:

-   **Compact**: Non-demanding standalone runtime that you can scale up as your needs grow. Can work with small chips or be utilized with massive servers. Embeddable on almost any app
-   **Easily modified**: Tweak Wasmtime in advance for pre-compilations, generate light-speed code using Lightbeam, or use for runtime interpretations. Configure for whatever you need Wasm to accomplish
-   **Speedy**: Compatible with Cranelift; perform high-resolution runtime machine coding
-   **WASI-compatible**: Supports a flush of APIs, allowing you to implement alongside the host via the WASI interface
-   **Support**: Compliant with WebAssembly test suite standards with a large base of developer and contributor community support

[Wasmtime installation/embedding guide](https://wasmtime.dev/).

---

## Lucet

Lucet was [announced](https://www.fastly.com/blog/announcing-lucet-fastly-native-webassembly-compiler-runtime) on 28 March 2019. It’s a lesser-known compiler/runtime native to WebAssembly and an option for developers who want to execute potentially hazardous Wasm programs contained in their application.

You can use it for running untrusted code, whether the infrastructure, device, or OS — a capability drawn from established examples used in web browsers.

Lucet features:_

-   **Fast specialization**: An open source project, run untrusted programs at near-native speed
-   **WASI-friendly**: Use WebAssembly outside of your browser. Lucet supports the WebAssembly System Interface on the Fastly edge cloud
-   **Supports many languages**: This includes Typescript, Rust, C/C++, and many more are in development
-   **Massively lowered runtime footprint**: Any request Fastly mediates can be executed by a WebAssembly instance representing all of the many thousands of requests per second per process, without extra [web app security risks](https://www.clouddefense.ai/blog/web-application-security)
-   **Ahead-of-time (AOT) compilation support**: Configure AOT compilations to streamline runtime overhead and design

---

## WebAssembly Micro Runtime (WAMR)

 WAMR has a tiny footprint and is made up of three components:

1.  It offers just-in-time (JIT) and AOT compilation, and WebAssembly interpretation
2.  An application framework that allows for building out multiple Wasm apps that can run on devices and for IoT purposes
3.  Remote application management from the cloud or host environment

KWAMR (iwasm) features: 

-   **Fast**: Runs at near-native speed (AOT)
-   **Compliance**: Compliance standards for the [W3C WASM](https://www.w3.org/TR/wasm-core-1/) MVP
-   **Small footprint**: Binary size of only 85K (interpreter) and 50K (AOT) and requires few memory resources

---

## Summary

-   Wasmer has the best overall support compatibility with every programming language at super-speed
-   Wasmtime is lightning-fast and compact, with good configurability but fewer languages supported
-   Lucet is a specialized solution for running untrusted WebAssembly programs inside a larger application
-   WAMR runs with a small footprint

 There are many WebAssembly runtimes other than this.
  If you want to take a look, you can refer to this : [awesome WebAssembly runtimes](https://github.com/appcypher/awesome-wasm-runtimes).
