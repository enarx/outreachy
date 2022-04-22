# 1 - Introduction to Confidential Computing - Part 1

The COVID-19 pandemic has amplified how much of our lives we live online. From ordering food for delivery, to completing a banking transaction to see a doctor, our personal data is in many different locations online.

So the question consumers must ask - **Who can access this data?**
And more importantly, **is it at risk of being compromised?**

Before going further lets first consider the three states of data:
![Three States of Data](https://www.wasm.builders/remoteimages/uploads/articles/076zudjx3sls6ra9s78o.png)

- Data at rest and in-transit can be protected using encryption. Even if the data is intercepted by a hacker, it can't be deciphered- and is therefore meaningless.
- Data in use, however, is a different story. Before it can be processed by an application, data must be decrypted. To put it simply, in order to use the data, you have to see the data. This leaves the data unencrypted in the memory of whatever device it's stored on and potentially exposed to malicious actors.

So, here comes **Confidential Computing** in picture.

> Confidential computing refers to cloud computing that isolates and protects sensitive data and code during processing in a hardware-based trusted execution environment (TEE).

When using confidential computing capabilities in the cloud, sensitive data is isolated into a protected enclave during processing.

The contents of this enclave- the data being processed and the techniques used to process it are only accessible to authorized code, invisible to anything or anyone else, including the operating system and cloud provider.

It means that your data is yours- and even the cloud provider cannot access it.
Let's understand it more clearly, by an example.

![office-building](https://www.wasm.builders/remoteimages/uploads/articles/enamk1z6cvg78x1ensmb.png)

Think of it like an office in an office building. The office is a private, secure location where you can have a meeting. There are a number of other offices in that building too, but you can lock your door and have a private meeting in your office and no one has access to your discussions- even though you are in the same building. The owners of the office building and tenants in other offices do not know what is going on in your office.


![Office building](https://www.wasm.builders/remoteimages/uploads/articles/zmmrk8dz370u8vlg7ih9.png)

In this case of confidential computing, the cloud is the office building, and the enclave is the office. This means that not even the cloud provider can access this data, which unlocks the computing scenarios that were previously unthinkable.

We will explore more about Confidential Computing/ Confidential Computing Consortium in next parts.

Thank you! :)

[Blog Post](https://www.wasm.builders/gunjan_0307/introduction-to-confidential-computing-part-1-2d1d)

