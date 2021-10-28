## OVERVIEW OF ENARX PROJECT

Enarx is the leading open source framework for running applications in TEEs (Trusted Execution Environments). It's part of the Confidential Computing Consortium from the Linux Foundation.

Enarx provides a run-time TEE based on WebAssembly, allowing developers to deploy applications without any rewrites from languages like Rust, C/C++, C#, Go, Java, Python, Haskell and many more.
It is designed to work across silicon architectures transparently to the user so that the application can run equally simple on Intel platforms (SGX or the recently-announced TDX), AMD platforms (SEV) or forthcoming platforms such as Arms’ Realms and IBM’s PEF - all without having to recompile the application code.

Enarx is CPU-architecture independent, letting developers deploy the same application code transparently across multiple targets. It provides a single run-time and attestation framework which is hardware vendor and CSP neutral.


### reference:
https://enarx.dev/
