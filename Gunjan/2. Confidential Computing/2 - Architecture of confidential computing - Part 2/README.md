# 2 - Architecture of confidential computing - Part 2

In our previous post of confidential computing we discussed “[What is confidential computing](https://www.wasm.builders/gunjan_0307/introduction-to-confidential-computing-part-1-2d1d)” and we realized that the problem lied when the data was being used i.e., during the runtime.

> The runtime environment is the environment in which a program or application is executed.

In this post, let’s discuss how confidential computing works and its architecture.

So, I will discuss [Enarx](https://enarx.dev/) Runtime architecture here.


![architecture-diagram](https://www.wasm.builders/remoteimages/uploads/articles/2hys89bbhw1hskuxrkhv.png)

---

First layer from bottom is **Hardware Abstraction Layer**

_As we know Confidential Computing is essentially the protection of data in use by performing computation in a hardware-based Trusted Execution Environment._

The most important thing we have to build on the bottom is essentially our hardware abstraction layer to be able to run workloads appropriately across both Process-Based and VM-Based technologies. 

We abstract different types of TEE technologies. In the current industry, we have technologies like- 

**Process-Based technologies** - Intel’s SGX, RISCV’s Sanctum

**VM-Based technologies** - AMD's SEV, POWER’s PEF, and Intel's TDX

---
The next level up is **webassembly**

Web Assembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications.

Next Layer is **WASI**

WASI stands for “Web assembly standard interface” which  is comparatively newer and is currently undergoing stabilization, WASI is the basic system interface. For Example- ‘sys calls’ so if you're gonna read or write to a socket or get some random data, these ‘sys calls’ are standardized in WASI. So this is founded to be the bedrock of what we are actually exposing to interfaces.

WASI and WASM both provides standard interface which means you don't need to write your application to ENARX. You can write it to any open standards and just deploy it with ENARX. And in addition, you get language-specific bindings. 
So for example if you're writing in C you get a libc implementation that uses web assembly and WASI, then your application will work fairly unmodified.

You're able to write in any of the languages that WASI supports - C, C++, Java, Go, Rust, .Net, etc. So, you can pull this into your CI/ CD pipeline and the compilation is targeted at WASI which makes it easy to use.

WASI and WASM both follows, W3C standards.

---
Language bindings are wrapper libraries that bridge two programming languages, so that a library written for one language can be used in another language.

Let's talk in detail about how different components are integrated and interact with each other.


![architecture-diagram](https://www.wasm.builders/remoteimages/uploads/articles/je9df0lh7mlbg616ei4u.png)

We have host on left side and client on the right.
So, on the host side we have trusted execution environment (TEE) which is represented by white box which is a secure area of a main processor. Everything represented in green is trusted by the client and secure.

The whole process is as follows:

1) First, we use the CLI to talk to the Enarx client agent.

_Enarx client agent is responsible for deploying the workload, so it takes in a web assembly binary that could be any standalone file in your file system that you've just compiled or it could come from any web assembly repository somewhere that you want to deploy from._ 

_The Enarx client's job is to take that image and to get it to the host in a secure and confidential way along with any other data configuration keys and to deliver all of that into the **Keep** at the host side, without the host being aware of what's actually happening. So the host does not have visibility to either code or data that are running on the host_

_**Shim** - Each different type of technology has a separate Shim. For example, SGX Shim, SEV Shim, and each one of those adapts the specific technology to be able to run the code on top of it. (And that's how we get our hardware abstraction)_

2) Then, the client agent speaks via the Enarx host agent which is the server. Host agent works with the CPU and Firmware to create a basic Keep.

![Architecture](https://www.wasm.builders/remoteimages/uploads/articles/6bspdz0qwwka7usp29gd.png)

3) Once that had happened, the measurement of that goes back to the Enarx client agent, and it checks that measurement, if it is correct then it takes the WebAssembly file and encrypts it under a session key that is unique to that Keep.

4) If we had created another Keep on the same machine, it would have been a different key.  
So, it encrypted it and sent it to the Keep where it is unencrypted because the Keep has that key and then runs as a standard Web Assembly application. 

**Summary**-  We asked for a Keep, we did an attestation, verified the attestation, encrypted the workload, sent the workload, and ran it.

Reference - [https://www.youtube.com/watch?v=Ku9h1i4tCyQ&t=785s](https://www.youtube.com/watch?v=Ku9h1i4tCyQ&t=785s)
Thanks!! 

[Blog Post](https://www.wasm.builders/gunjan_0307/architecture-of-confidential-computing-58gd)
