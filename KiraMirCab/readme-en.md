*Read this in Spanish language: [English](readme.md).*

# Enarx: A Leading Open Source Project in Confidential Computing

## Summary

Organizations that operate with sensitive code or data need to ensure its integrity and confidentiality when running their applications, especially when deployed in the public cloud or on the Edge. Enarx is an open source project that uses TEE (Trusted Execution Environment), or Secure Execution Environments, which enables organizations to run applications and process sensitive data on untrusted systems.

## Introduction

Organizations in different industries launch their workloads in multiple execution environments, from their own premises to the public cloud and Edge. And they require the highest possible assurance that their code and confidential data are protected. This matter is especially important for sectors such as banking and finance, public and administrative sector, telecommunications, Internet of things (IoT), healthcare (HIPAA, the confidentiality standard of the Health Insurance Portability and Accountability Act in the USA), data of the client (RGPD, General Data Protection Regulation), confidential business functions and human rights.

There are three states in which data can be protected: in storage, in transit, and in use. Data in storage are files and saved objects. Data in transit is data that is traveling from one site to another through the Internet or through a private network. The data in use is what is being processed in the CPU or RAM. Encrypting data in storage and in transit has already become a norm in the cloud, while the need to protect data in use is only beginning to emerge.

When a workload runs in the public cloud, either as a virtual machine or a container, it can be manipulated by anyone or software with access to the host. Even if the cloud provider has strict security policies, the workload is still vulnerable in case the host's own system is compromised: its operating system, firmware libraries, hypervisor, application stack, applications. third-party libraries, middleware, and drivers.

Secure Execution Environments (TEEs) provide a hardware-based solution to this need to maintain the confidentiality and integrity of data in use, regardless of who owns or has access to the host system on which the application runs.

## Context

Secure Execution Environments (TEE) allow organizations to run their applications within a set of memory pages encrypted with a secret key by the host CPU in such a way that these pages are not accessible to the operating system or any other software. , even with the highest level of privileges.

There are currently two main TEE models:

- _Based on process_: In this model, a process is divided into two components: trusted and unreliable. The trusted component resides in the encrypted memory and handles confidential computing, while the untrusted component interacts with the operating system and communicates the encrypted memory by I / O with the rest of the system. Data can only enter and exit this encrypted zone through certain channels, which have strict controls on the size and type of data that can pass. One of the current implementation examples of this model is Intel's SGX (Software Guard eXtensions).

- _Virtual machine-based_: in this model the encrypted memory is delimited like a traditional VM, managed by the VMM. Traditional VMs (and containers) offer some measure of isolation, but in this type of TEE the VMs are protected by hardware-based encryption keys, thus preventing interference from a malicious VMM. This TEE model is used in AMD's Secure Encrypted Virtualization (SEV).

The applications to be run on a TEE must be developed specifically for each platform, as the architecture depends on the type of TEE: process-based or VM-based. In addition, they must implement what is called "attestation", a validation process whose objective is to confirm that the TEE to be used by the application is authentic and secure. Rewriting the application or the custom VMM that runs it, making the attestation for each hardware platform is extremely complex and time consuming.

In the next section we will introduce Enarx, an open source framework for running TEE applications, and we will explain how it solves the problems posed. We will provide a simplified overview of the Enarx component architecture (and its adaptability to multiple hardware platforms). We will explain how applications are built and deployed to TEE instances using Enarx.

## Enarx

Enarx is a framework that runs applications in what we call "Keeps", TEE instances, without the need to implement the certificates separately, or to rewrite the application, or to rely on many dependencies. Provides a WebAssembly runtime, based on wasmtime. It is designed to work on different processor architectures in a transparent way for the user, so that the application can easily run on both Intel platforms (SGX or the recently announced TDX), AMD platforms (SEV) or future platforms such as Arms & # 39; Realms and PEF from IBM, without the need to recompile the application code.

The goal of Enarx is to minimize trusts when running applications, so the only components to trust are: the CPU and associated firmware, the workload itself, and the Enarx middleware, which is code. fully open and auditable. The applications are executed without any of the layers of the stack (hypervisor, kernel or user space) being able to access the Keep, or alter its content.

Enarx makes the attestation, packaging, and provisioning of the application transparent to the user. Each instance of the application goes through three stages:

1. _Atested_: Enarx verifies that the host where the application is to be deployed is a genuine instance of TEE.
2. _Packaging_: Once the attestation is completed and the TEE instance is verified, the Enarx management component encrypts the application, along with the rest of the necessary data.
3. _Provisioning_: Enarx then sends the application and data to the host for execution in Enarx Keep. At no time can the host system access the application or modify the data.

Enarx manages the attestation and provisioning process in a "Keep" runtime, based on WebAssembly, allowing developers to use a wide range of languages. The Keep runtime supports applications written in Rust, C, C ++, C #, Go, Java, Python, and Haskell. Enarx does not depend on the architecture of the CPU, which allows to implement the same application code on different platforms, avoiding problems such as cross-compilation or different attestation mechanisms, depending on the hardware vendor.

## Conclusion

Enarx is an open source project that uses TEE (Trusted Execution Environments) to run applications within "Keeps" on untrusted systems. Enarx manages the creation of these Keeps, verifies that the CPU to be used is valid, then encrypts and passes the applications and data to the Keep using single-use cryptographic keys. Applications run without any layer in the stack (eg hypervisor, kernel, user space, middleware) being able to access the Keep. Enarx allows you to run applications on TEE from various CPU manufacturers without having to worry about portability, manages crowding and deployment.
