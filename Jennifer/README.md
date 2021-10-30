
# Enarx and Webassembly

Enarx in summary is a deployment framework for running applications in Trusted Execution Enviroments (TEE), while web assembly is a compilation target that provides a way to compile codes written in diffrent languages into a binary form that can be executed in a predefined space e.g web-browsers.

## So why exactly did Enarx choose to use webassembly ???

- <h5>Platform Portability :</h5>
Enarx uses WASM to provide a WebAssembly runtime to allow workloads run on whatever platform is available, as webAssembly is not tied to any hardware architecture. workloads can be executed on several different platforms without the need to re-compile. 

 - <h5>Isolation or Sandboxing :</h5>
 By compiling to wasm, the code is sandboxed, preventing it from accessing anything on the outside e.g., memory. The sandboxed code can’t read or write to anywhere outside it, the sandboxed code can’t do anything but pure computation, unless given a function to call to do things like read from a file, tell the time, etc.


sources :

[Lin clarks tutorial](https://hacks.mozilla.org/2017/02/a-crash-course-in-assembly/)


[Kassian Rosner Wren's tech talk](https://www.digitalocean.com/community/tech_talks/webassembly-for-beginners)

[Lin clarks tech talk](https://www.youtube.com/watch?v=fh9WXPu0hw8&t=281s)

[Deploy Friday](https://web.facebook.com/platform.sh/videos/403806487694538)