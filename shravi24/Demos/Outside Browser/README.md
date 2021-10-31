<h1>Outside the Browser</h1>


For out of browser scenarios, one of its main advantage is that it provides system level access without compromising on security. This is done through WASI, the Web Assembly System Interface. WASI is a collection of C-like functions that provide access to functionality such as fd_read, rand, fd_write, threads (WIP), in a safe way.

Here are a few scenarios where you would be able to use web-assembly outside of a browser:

- A scripting language for a video game.

- To run some code with minimal overhead as Fastly/Cloudflare are doing with their compute-at-edge scenarios.

- To run some easy to update code on IoT devices safely and with minimal runtime overhead.

- Extreamly fast programs in environments where you can’t JIT for reasons.