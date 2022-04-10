
<h1>Case Study: Shopify</h1>

<img src ="../../../../images/shopifyweb.png">

<p>&nbsp;</p>

<h1> Concerns </h1>

To keep the code base and user experience simple, Shopify has a product principle : Build what merchants need, most of the time. If merchants need features  beyond that core set, Shopify provides an app store with third party apps to solve for that long tail of requirements.

 The primary mechanism of this ecosystem is an “App”, an independently hosted web service which communicates with Shopify over the network. This model is powerful, but comes with a host of technical issues. Partners are stretched beyond their available resources as they have to build a web service that can operate at Shopify’s scale. 
 
 Many of those apps are hosted on their own infrastructure, and during flash sale events, are sometime taken down by the surge in load.

 Even if Partners’ resources were unlimited, the network latency incurred when communicating with Shopify precludes the use of Apps for time sensitive use cases, so that partners can focus on using their domain knowledge to solve problems, and not on managing scalable web services. Thus, to help app developers in building more stable apps, Shopify wants to allow app code to run internally right within Shopify.
 
But, executing untrusted code can cause harm to Shopify’s platform at large.  So, there is a need to prevent security flaws and mitigate their impacts when they occur.

In ecommerce, speed is an another aspect that merchants need to drive sales. If a feature we deliver to merchants doesn't come within the time stipulations, then we may as well not deliver it at all. Thus, there is a need of a format that is performant, secure, and flexible.

But how can you alleviate all these concerns? By using WebAssembly...


<p>&nbsp;</p>

<h1> Solution </h1>

Shopify executes Wasm outside of the browser and with no Javascript involved. Wasm satisfies their three main technical requirements: security, performance, and flexibility.

1. Security:
	Wasm executes within a sandboxed stack-based environment, the communication with the host can only be thorugh explicit imports. Because of this, you are unable to express anything malicious in Wasm.
  
2. Performance:
	Wasm is designed to leverage common hardware capabilities that provide it near native performance (Highly optimized) on a wide variety of platforms. It’s used & developed by a performance driven community of developers looking to optimize browser execution.
  
3. Flexibility:
	Wasm can be used as a compilation target by a number of different languages(Rust, C, C++). This allows us to support multiple languages for developer use without altering the underlying execution model.  
  
<p>&nbsp;</p>

<h1> How is it executing? </h1>

Shopify uses open source tool called Lucet for wasm compilation and runtime.

- Lucet validates the Wasm module as a security check. After the validation, the module is compiled to an executable artifact. It supports ahead of time compilation, which allows the artifacts to execute at runtime, providing optimum performance. 
- Lucet containers has an impressive startup time of 35 μs. That’s because it’s a container that doesn’t need to do anything at all to start up. Lucet is wrapped within a Rust web service. This web service engine manages the I/O and storage of modules, that is also known as the Wasm Engine. 
- Shopify calls this engine during runtime process, usually in a form of web request, to satisfy some function. This application could involve the creation of a discount, the enforcement of a constraint, or any form of synchronous behaviour Merchants want to customize within the Platform.  
- Shopify uses AssemblyScript even though Shopify's  language of choice is Ruby because Ruby is a dynamic language, so it can’t be compiled down to Wasm directly. Many alternative solutions involve compiling interpreters, but there is a steep performance penalty. Thus Shopify needed tools that produce WebAssembly, rather than tools which are powered by WebAssembly. AssemblyScript is one such tool.
<p>&nbsp;</p>
<p align="left">
<img src ="../../../../images/Pasted image 20211103235714.png">
</p> 
<p>&nbsp;</p>

<h1> Execution Performance </h1>
Here’s some metrics pulled from a recent performance test. During this test, 100k modules were executed per minute for approximately 5 min.
<p>&nbsp;</p>

<p align="left">
    <img src ="../../../../images/Pasted image 20211103235747.png">
</p>

This chart demonstrates a breakdown of the time taken to execute a module, including I/O with the container and the execution of the module. The y-axis is execution time in ms, the x-axis is the time over which the test was running.
From the graph, time taken by module execution process takes under 5 ms, thus the performance impact of Lucet execution is negligible.

