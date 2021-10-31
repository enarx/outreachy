# Markdown Parser

![Rust To Wasm](./../../../img/markdwonParser.gif)

A markdown parser, which parses any Markdown file, produces and log out HTML on terminal. As shown in above illustration the steps required to perform the same are as such:
- Rust code get compiled by `cargo-wasi` to produce a `.wasm` file
- WebAssembly runtime `wasmtime` executes the `.wasm` file with input `markdown` file which we want to parse, and log the `html` output on terminal. Here we need to pass our data directory so that it preloads the file because WebAssembly runs into a sandbox environment.

## Used Commands

- Project creation
  ```
  cargo new --bin markdown_parser

  ```
- Wasm file creation or compilation
  ```
  cargo build --target wasm32-wasi

  ```

  - Parsing and logging out the Markdown or Execution

  ```
  wasmtime --dir . target/wasm32-wasi/debug/markdown_parser.wasm -- README.md

  ```

### Input
This markdown file itself, when not having current headings `Input` ,  `Output` and its content.


### Output
```
<h1>Markdown Parser</h1>
<p><img src="./../../../img/markdwonParser.gif" alt="Rust To Wasm" /></p>
<p>A markdown parser, which parse any Markdown file, produce and log out HTML on terminal. As shown in above illustration the steps required to perform the same are as such:</p>
<ul>
<li>Rust code get compiled by <code>cargo-wasi</code> to produce a <code>.wasm</code> file</li>
<li>WebAssembly runtime <code>wasmtime</code> execute the <code>.wasm</code> file with input <code>markdown</code> file which we and to parse, and log the <code>html</code> output on terminal. Here we need to pass our data directory so that it preload the file because WebAssembly run into a sandbox environment.</li>
</ul>
<h2>Used Commands</h2>
<ul>
<li>
<p>Project creation</p>
<pre><code>cargo new --bin markdown_parser

</code></pre>
</li>
<li>
<p>Wasm file creation or compilation</p>
<pre><code>cargo build --target wasm32-wasi

</code></pre>
<ul>
<li>Parsing and logging out the Markdown or Execution</li>
</ul>
<pre><code>wasmtime --dir . target/wasm32-wasi/debug/markdown_parser.wasm -- README.md

</code></pre>
</li>
</ul>


```
