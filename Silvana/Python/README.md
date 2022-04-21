 **Python** language is one of the most accessible programming languages available as it has simplified and uncomplicated syntax which puts more emphasis on natural language. Because of its ease of learning and use, **Python**  codes can be easily written and run much faster than other programming languages.

**Python**  was created over 30 years ago, which is a long time for any programming language community to grow and mature adequately to support developers ranging from beginner to expert level. There are plenty of  **Python** language documentation, guides and video tutorials available that the student and developer of any skill level or age can use and receive the support they need to advance their knowledge of the **python** programming language.

Many students are introduced to computer science only through the **Python** language, which is the same language used for in-depth research projects.

Cloud Computing, Machine Learning and Big Data are some of the hottest trends in the computer science world right now, which helps many organizations to transform and improve their processes and workflows. All these trends need to take into account data security, and secure implementation to handle personal data. So the union of **python** and **WebAssembly** is a powerful tool.

  To start using the tool, let's start with the setup. 
First you need to download **docker**.

**Docker**: _Is an open source containerization platform. It enables developers to package applications into containers—standardized executable components combining application source code with the operating system (OS) libraries and dependencies required to run that code in any environment._


Then you have to compile **Python** to **WebAssembly (WASM)**:

You have to  clone the repo https://github.com/singlestore-labs/python-wasi/

-Change directory to python-wasi using the command:

` cd python-wasi`

-Build the docker image using the command: 


`docker build -f docker/Dockerfile -t wasi-build:latest docker`


-Now start the docker container and mount the current directory was working directory inside docker container:

`docker run -it --rm -v $(pwd):$(pwd) -w $(pwd) wasi-build:latest bash`


-To download the CPython source, dependencies and to build the CPython's WASM build, run the command:

`./run.sh`


The build output is saved at opt/wasi-python/.

**As a final step install wasmtime using the instructions at wasmtime.dev**

Then you can have you wasm file: 


![Image description](https://www.wasm.builders/remoteimages/uploads/articles/o4hb7ysrw1xmbu43s8wr.png)

The code in **Python** is about Count **Inversion**: 

  Inversion Count for an array indicates – how far (or close) the array is from being sorted. If the array is already sorted, then the inversion count is 0, but if the array is sorted in the reverse order, the inversion count is the maximum. 

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/akeqgb9z7mcmpejd8rug.png)










1-[https://www.upgrad.com/blog/reasons-why-python-popular-with-developers/]

2-[https://www.pulumi.com/why-is-python-so-popular/]

3-[https://www.mygreatlearning.com/blog/why-is-python-so-popular-for-ai-and-machine-learning/]

4-[https://www.ibm.com/in-en/cloud/learn/docker]

