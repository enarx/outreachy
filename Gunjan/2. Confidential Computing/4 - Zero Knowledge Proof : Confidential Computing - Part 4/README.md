# 4 - Zero Knowledge Proof : Confidential Computing - Part 4

There are a lot of useful technologies coming out, so more and more technologies are very great, but they have the big drawback that they need your data. And your data can actually be pretty private or sensitive.

In that scope, actually, privacy enhancing technologies aims to let you have both. So you can benefit from modern technologies without having to give back your data, or, on the other way around, you can have your privacy without having to go back to the Stone Age, for instance. 

One nice example of privacy enhancing technologies is the **zero-knowledge proof** and it's the thing I'm going to talk about you today.

## What actually zero-knowledge proofs are ?

They are basically a protocol which let a prover, let's say me, to prove you a statement about a secret, without actually giving up that secret. So I can prove to you that I know a secret, and something about that secret, without revealing that secret.

## Example

Let’s illustrate it with the help of Bob and Alice who got some chocolate bars for Halloween.

![alice and bob](https://www.wasm.builders/remoteimages/uploads/articles/tr657rv5oydeghdmksfx.png)

They would like to know if they received the same amount of candy, without disclosing their number of chocolates because they don’t want to share. 
Let’s assume they can have exactly 10, 20, 30, or 40 chocolate bars in their trick-or-treat bags.

![bags](https://www.wasm.builders/remoteimages/uploads/articles/040zsc8e899pdte8vp7e.png)

To compare the number of chocolate bars they got without sharing the actual number, Bob gets 4 lockable boxes and puts a label in each that says 10, 20, 30 or 40 (chocolate bars).

Then Bob throws away all the keys except for the key to the box that corresponds to the number of chocolate bars he’s got (let’s say he has 20 chocolate bars) and leaves.

![alice](https://www.wasm.builders/remoteimages/uploads/articles/0w50f5s7izrs8ru4ygkn.png)

Alice takes 4 small pieces of paper and writes “+” on one of them and “-” on all the others. Then she slips the “+” piece through a slot into the box with the number that corresponds to the number of candies she’s got (let’s say she has 30 candy bars) and slips the pieces of paper with “-” on them into the rest of the boxes and also leaves.

![alice](https://www.wasm.builders/remoteimages/uploads/articles/630086cju6pxylrzvt33.png)

Bob returns and opens the one box he still has the key to — the one that corresponds to the amount of candy he’s got — and sees if it contains “+” or “-”. If it is a “plus”, Alice has the same number of chocolate bars in her bag. If the slip of paper says “-”, it means that they have a different amount of candy (but still will not share with each other).

So, here a prover(Alice) has convinced a verifier(Bob) that some statement holds without revealing any information about why it holds. 

Another example can be prover can for example convince a verifier that a confidential transaction is valid without revealing why that is the case, i.e. without leaking the transacted values.
 
An argument is a proof which holds only if the prover is computationally bounded and certain computational hardness assumptions hold. 

## Demo

In this demo, we are implementing a bulletproof range proof that proves the secret value we have is in certain range along with a commitment to the secret so that we can successfully verify the secret. For creating the commitment, I am using Pedersen Commitment which uses a blinding factor and ellliptical function to mask data and still make it possible to verify the secret by using additive properties of the function. And using rangeproof we can verify that the value is within a range and in turn validate the transaction.

The scnenario underlying this demo is that we are carrying out a confidential transaction on a trusted execution environment using zero knowledge proof of knowledge, in order to carry out secure verification.

I am demonstrating it using [https://github.com/shravi24/Enarx-Bulletproof-Demo](https://github.com/shravi24/Enarx-Bulletproof-Demo) which is built in rust on top of Enarx. 

**Steps to perform ZKP**

```
git clone https://github.com/shravi24/Enarx-Bulletproof-Demo
cd Enarx-Bulletproof-Demo/Code
cargo build --target=wasm32-wasi
enarx run target/wasm32-wasi/debug/bp-test1.wasm

```

![Output](https://www.wasm.builders/remoteimages/uploads/articles/vyfketkao8ew1gczgm5y.png)

There are 2 inputs that are required:
Enter Secret:
_This will be the secret number that prover has to verify to verifier_
Enter The Bit Vector: 
_Bit vector is the range in which prover will tell if his secret lies in this range or not. If you enter bit vector as say - 16 then our range will from 0 to 2^16 (65536)_
For eg: 

![Output](https://www.wasm.builders/remoteimages/uploads/articles/madb4tff7bnu7u6uh4uf.png)

Here secret is 7 and our range is 65536. Since 7 lies inside this range, it says proven.
So, this is how ZKP works,prover has proved a statement about his secret to verifier, without actually revealing it.

So you might be wondering where it is actually being used in confidential computing. Zero Knowledge Proof allows us to carry out the confidential transaction in a Trusted Execution Environment. It provides an added layer of security and also reduce number of trusted parties in the system.

## Other applications

**Blockchain**: For instance, the cryptocurrency Zcash is based on Zero-Knowledge Succinct Non-Interactive Argument of Knowledge (Zk-SNAKR), a type of zero-knowledge cryptographic method.

**Finance**: ING uses ZKPs that allow customers to prove that their secret number lies in a known range. For example, a mortgage applicant can prove that their income is in the admissible range without revealing their exact salary.

**Online voting**: ZKPs can allow voters to vote anonymously and to verify that their vote was included in the final tally.

**Authentication**: ZKPs can be used to authenticate users without exchanging secret information such as passwords.

**Machine Learning**: ZKPs can allow the owner of a machine learning algorithm to convince others about the results of the model without revealing any information about the ML model itself.

References 
[https://github.com/shravi24/Enarx-Bulletproof-Demo](https://github.com/shravi24/Enarx-Bulletproof-Demo)
[https://www.youtube.com/watch?v=9GEZ1pnGN0I&t=943s](https://www.youtube.com/watch?v=9GEZ1pnGN0I&t=943s)
[https://hackernoon.com/eli5-zero-knowledge-proof-78a276db9eff](https://hackernoon.com/eli5-zero-knowledge-proof-78a276db9eff)
[https://www.youtube.com/watch?v=HUs1bH85X9I](https://www.youtube.com/watch?v=HUs1bH85X9I)
[Zero knowledge Proof](https://research.aimultiple.com/zero-knowledge-proofs/#:~:text=Zero%2Dknowledge%20proofs%20can%20be,lead%20to%20deanonymization%20of%20users.)

[Blog Post] (https://www.wasm.builders/gunjan_0307/zero-knowledge-proof-confidential-computing-part-4-2mf1)
