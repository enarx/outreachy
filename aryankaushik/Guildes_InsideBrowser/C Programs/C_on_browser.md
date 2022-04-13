# Executing a C/Cpp program to WebAssembly inside Browser.



Hi There! Hope you are doing well. It is always exciting to learn new stuff and experimenting. Today, we are going to execute a C/Cpp program to Webassembly inside a browser.

Here we go!

#### 1. First, let's set up the required development environment.
[Emscripten](https://emscripten.org/) Environment Setup:

> Emscripten is an LLVM/Clang-based compiler that compiles C and C++ source code to WebAssembly, primarily for execution in web browsers.

The core Emscripten SDK (emsdk) driver is a Python script. You can get it for the first time with:

#### a. Clone the repo from github

```
git clone https://github.com/emscripten-core/emsdk.git
```
=>Move to emsdk:
`cd emsdk`


![1](https://www.wasm.builders/remoteimages/uploads/articles/n5jv6b5mg7vds19fsrvw.png)

#### b. Fetch the latest version of the emsdk (not needed the first time you clone)
`git pull`

#### c. Download and install the latest SDK tools.

```
./emsdk install latest
```

#### d. Make the "latest" SDK "active" for the current user. (writes .emscripten file)

```
./emsdk activate latest
```

#### e. Activate PATH and other environment variables in the current terminal

```
source ./emsdk_env.sh
```
#### For me it worked like
```
source "/home/aryan/emsdk/emsdk_env.sh"
```

#### Configure emsdk in your shell startup scripts by running:
```
echo 'source "/home/aryan/emsdk/emsdk_env.sh"' >> $HOME/.bash_profile
```


![2.](https://www.wasm.builders/remoteimages/uploads/articles/5cdjlg5jyyjhle29eemq.png)


![3.](https://www.wasm.builders/remoteimages/uploads/articles/2c7y5rw909971ej05myy.png)

You can also install a specific version by specifying it, for example, it is not necessary for beginners.
`./emsdk install 1.38.45`

=> Python is not provided by emsdk. So, the user is expected to install this beforehand with the system package manager:

Install Python

```
sudo apt-get install python3
```
Install CMake (optional, only needed for tests and building Binaryen or LLVM)

```
sudo apt-get install cmake
```

#### 2. Now Create a Hello World .c program and save it as HelloWorld.c


```
#include<stdio.h> // define the header file  
void main()   // define the main function  
{  
    printf("Hello World \n");  // print the statement.  
}  
```

![c](https://www.wasm.builders/remoteimages/uploads/articles/0j4treunjciofblvlhhz.png)


Now, we have to compile it using below command :

```
emcc HelloWorld.c -o hello.html -s WASM=1
```

![4.](https://www.wasm.builders/remoteimages/uploads/articles/v8qbho2door4g16de7ji.png)

After compilation, if you have tried to open the file hello.html with certain browsers , You may get an error message instead of output that you want.

That is because the operation of loading of the WASM module is asynchronous, and some browsers (for security reasons) do not allow you to do this if the URL is of the kind file://path/to/file/file.html.

In order to solve that issue you can run a local server this way using below command :

```
python3 -m http.server 8080
```



![5.](https://www.wasm.builders/remoteimages/uploads/articles/zzbfpe6dswitd1rmmutv.png)




#### 3. Go to browser and type in localhost:8080 or http://127.0.0.1:8080/

![6.](https://www.wasm.builders/remoteimages/uploads/articles/imlb5ijtt1y40rz9za7b.png)



#### 4. Now Open hello.html
![7.](https://www.wasm.builders/remoteimages/uploads/articles/f5mqadb2k1w9ydtgd3v4.png)


Finally, After a long hustle we have been able to Execute a  C program to WebAssembly inside a Browser.

Same steps for cpp, just save the prgram with .cpp extension and the rest of the process is same as it is.


References:

1. https://developer.mozilla.org/enUS/docs/WebAssembly/C_to_wasm
2. https://emscripten.org/docs/getting_started/downloads.html
3. https://github.com/enarx/outreachy/tree/main/shravi24/Demos/Inside%20Browser/C


