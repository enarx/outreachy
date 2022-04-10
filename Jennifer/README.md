
# Enarx and Webassembly

Enarx in summary is a deployment framework for running applications in Trusted Execution Enviroments (TEE), while web assembly is a compilation target that provides a way to compile codes written in diffrent languages into a binary form that can be executed in a predefined space.

## So why exactly did Enarx choose to use webassembly ???

- <h5>Platform Portability :</h5>
Enarx uses WASM as a portable binary format that allows workloads written in any language such as C/C++, rust, etc., to be executed in TEE without the need to recompile.
 

 - <h5>Isolation or Sandboxing :</h5>
We all know that Enarx needs to protect guests from host and also host from guests. The Wasm virtual machine architecture by default protects the hosts from the guests due to the capabilities of the language and specification, as wasm modules don’t have access to the host runtime’s memory; they utilise their own private linear memory space. The Trusted Execution Environments then protects guests from hosts.



## sources :

[Lin clarks tutorial](https://hacks.mozilla.org/2017/02/a-crash-course-in-assembly/)


[Kassian Rosner Wren's tech talk](https://www.digitalocean.com/community/tech_talks/webassembly-for-beginners)

[Lin clarks tech talk](https://www.youtube.com/watch?v=fh9WXPu0hw8&t=281s)

[Deploy Friday](https://web.facebook.com/platform.sh/videos/403806487694538)
