*Read this in [Hindi](enarx_intro_hindi.md) language*

# Enarx: A major open source project for confidential computing


## Abbreviation

Organizations with sensitive code or data need to run their applications with strong integrity and privacy protections, especially when deployed to the cloud or edge. Enarx is an open source project that uses TEE (Trusted Execution Environment) to enable organizations to run applications and process confidential data on systems they do not trust.


## Role

As organizations in different sectors move their computing workloads across multiple environments, from on-premises to the public cloud to the edge, they need more assurance that their sensitive code and data is secure. This is especially true for sectors such as banking and finance; government and public sector; telecommunications; Internet of Things (IoT); health (such as HIPAA); customer data (such as GDPR); sensitive enterprise functions; Defense; and human rights.

There are three stages in which data can be preserved: at rest, in transit, and in use. The rest of the data includes files, objects, and storage. Data in transit includes data that moves from one place to another such as across the Internet or over a private network. Data in use is the data that is being processed in the CPU or RAM. Encrypting data at rest and in transit has become a norm in cloud computing, but data in use is still an emerging concern.

When running a workload in a public cloud, either as a VM or container, the workload is likely to be tampered with by any person or software with access to the host system. Even if a cloud provider has strict security policies, the workload is still susceptible if the host system is compromised, including the operating system, firmware library, hypervisor, application stack, third party libraries, middleware, and drivers.


Trusted Execution Environments (TEEs) provide a hardware-based solution to this need to maintain data confidentiality and integrity in use, regardless of whether that host system owns or has access to the host system. on which the application is running.

## Background

Trusted Execution Environment (TEE) allows organizations to run applications within a set of memory pages that are encrypted by the host CPU with a secret key so that these pages are not accessible to the operating system or any other software, even if The same operates at the highest privilege level as well.

`There are currently two major models of TEE:`

`Process-based`: In this model, a process is divided into two components: reliable and unreliable. The trusted component resides in encrypted memory and handles confidential computing, while the untrusted component interfaces with the operating system and transmits I/O from encrypted memory to the rest of the system Is. Data can only enter and exit this encrypted area through predefined channels, with strict checks on the size and type of data passing through. Current implementations of the process-based approach include Intel's SGX (SGX - Software Guard Extension).

`VM-based`: In this model, the memory is encrypted with a traditional VM boundary running on top of the VMM. While traditional VMs (as well as containers) provide some measure of isolation, in this TEE model the VMs are protected by hardware-based encryption keys that prevent interference by malicious VMs. Current implementations include AMD's SEV (Secure Encrypted Virtualization).

The applications required to run in TEE must be developed specifically for each platform, and they vary greatly depending on the process-based or VM-based TEE model. Additionally, they have to implement something called verification, which is a test of TEE.There is a verification process to prove that it is genuine before it can be trusted by the application. Rewriting applications or custom VMMs for each hardware platform, as well as validation, is extremely complex and time-consuming.




In the next section, we'll introduce Enarx, an open source framework for running applications in TEE that addresses many of the issues raised. We'll give a simplified overview of Enarx's component architecture (and how it allows for support for multiple hardware platforms) and the process of building and deploying applications to TEE instances using Enarx.

## Enarx

Enarx is a framework for running applications in TEE instances - what we refer to as "keyup" - without the need to implement validation separately, without needing to rely on a lot of dependencies, and recompile the application. without the need to write from. It provides a webassembly runtime based on wasemtime.It is designed to work in a silicon architecture transparent to the user so that applications can run on the Intel platform (SGX or the recently announced TDX), the AMD platform (SEV) or similar on upcoming platforms such as Arms Realms and IBM's PEF, be able to walk easily without recompiling all the application code.

Enarx aims to reduce the trust relationships required when executing applications, which means that the only components that need to be trusted are: the CPU and associated firmware, the workload itself, and Enarx middleware, which is completely It is open source and auditable. No layer in the application stack stack (eg hypervisor, kernel, user-space) is able to see or change the "keep" or its contents.

Enarx provides attestation, packaging and provisioning of applications in a transparent manner for the user. Each instance of an application goes through three phases:

`Attest`: Enarx checks that the host you plan to deploy to is a real TEE instance.

`Packaging`: Once the attestation is complete and the TEE instance is verified, the Enarx management component encrypts the application along with any required data.

`Provision`: Enarx then sends the application and data along with the host for execution in the Enarx keyup. At no time is the host system, application or data inside able to see or change.




Enarx handles attestation and distribution in a run-time "keyup" based on WebAssembly, giving developers a wide range of language options for implementation. Keep's run-time can accept applications written in languages ​​such as Rust, C, C++, C#, Go, Java, Python, and Haskell. Enarx is CPU-architecture independent, enabling the same application code to be deployed across multiple targets, so that hardware vendors stay away from issues such as cross-compilation and separate attestation mechanisms.

## Conclusion
Enarx is an open source project that uses TEE (Trusted Execution Environment) to allow applications running within "keyups" on systems that are not trusted. Enarx manages the creation of these keys, providing cryptographic confidence that the keys are using valid CPU hardware and Then encrypting and provisioning the application and data in the keycap using one-time cryptographic keys. Application stack: Any layer in the stack (eg hypervisor, kernel, user-space, middleware) is not able to be seen in keyup. This allows it to run on TEEs from different CPU manufacturers without worrying about portability: Enarx manages this with validation and deployment.
