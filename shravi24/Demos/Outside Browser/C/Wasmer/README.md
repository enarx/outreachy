<h1> What is WASMER? </h1>

Wasmer is a WebAssembly runtime that enables super lightweight containers to run anywhere from Desktop to the Cloud, Edge and IoT devices.

Wasmer provides a CLI in order to easily execute wasm files in the command line. It supports multiple backends including LLVM, Cranelift, and Single pass (dynasm).
<p>&nbsp;</p>
<h1> Features </h1>

-   Fast. Run WebAssembly at near-native speeds.
-   Secure by default. No file, network, or environment access, unless explicitly enabled.
-   Supports WASI and Emscripten out of the box.
-   Embeddable in multiple programming languages
-   Compliant with latest WebAssembly Proposals (SIMD, Reference Types, Threads, ...)
<p>&nbsp;</p>
<h1> Installation </h1>
Wasmer provides a CLI in order to easily execute wasm files in the command line.

On Linux you can install it like this:

```bash
curl https://get.wasmer.io -sSfL | sh
```

On windows, you can go the [Github Releases Page](https://github.com/wasmerio/wasmer/releases), and there you can find .exe installers.

To check if everything is working fine you can download the qjs.wasm file at the [wapm website](https://wapm.io/package/quickjs#explore).

You can also refer to below picture to install quickjs :

Now you can run Wasmer to get a QuickJS prompt which you can use to run javascript.

Take a look at below example :

```bash
kali@kali:~/Downloads$ wasmer qjs.wasm
QuickJS - Type "\h" for help
qjs > [1, 2, 3, 4].map(t =>t*t*t)
[1, 2, 3, 4].map(t =>t*t*t)
[ 1, 8, 27, 64 ]
qjs > 


```
<p>&nbsp;</p>
<h1> WAPM - Package Manager for WebAssembly </h1>

WAPM is a package manager for WebAssembly modules which comes bundled with the Wasmer itself.

In previous section, we downloaded QuickJS manually but using wapm we can install the QuickJS package much easily.

You can use below command to install QuickJS.

```bash
wapm install -g quickjs
```
![Alt Text](../../../../images/quickjs.gif)

