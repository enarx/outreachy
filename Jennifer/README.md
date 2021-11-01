
# Enarx and Webassembly

Enarx in summary is a deployment framework for running applications in Trusted Execution Enviroments (TEE), while web assembly is a compilation target that provides a way to compile codes written in diffrent languages into a binary form that can be executed in a predefined space e.g web-browsers.

## So why exactly did Enarx choose to use webassembly ???

- <h5>Platform Portability :</h5>
Enarx uses WASM as a portable binary format that provides runtime to allow workloads written in any language such as C/C++, rust, etc., to be executed on TEE without the need to recompile. WASM is usable across environments and is architecture-independent.
In simple terms, Webassembly allows users to use any language, compile it once to produce one binary file that will just run, securely, in any environment that recognizes WebAssembly
 

 - <h5>Isolation or Sandboxing :</h5>
A WASM module doesn’t have access to APIs and system calls in the OS. If there is a need to interact with anything outside the module, it would  explicitly imported. Enarx uses this feature to protect guests from hosts and also hosts from guests as webassenbly can safely run untrusted codes in a sandbox environment.





## sources :

[Lin clarks tutorial](https://hacks.mozilla.org/2017/02/a-crash-course-in-assembly/)


[Kassian Rosner Wren's tech talk](https://www.digitalocean.com/community/tech_talks/webassembly-for-beginners)

[Lin clarks tech talk](https://www.youtube.com/watch?v=fh9WXPu0hw8&t=281s)

[Deploy Friday](https://web.facebook.com/platform.sh/videos/403806487694538)