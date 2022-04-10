<h1>Let's do a "Hello world" C program and compile it to WebAssembly</h1>

## Tool Setup

We need to get [emsdk](https://emscripten.org/docs/getting_started/downloads.html) which is Emscripten's tool to get the compiler and all the tools you need. We will focus on how to install it for Linux, but you can find documentation for other OSes [here](https://kripken.github.io/emscripten-site/docs/getting_started/downloads.html).

it's easy to get Emscripten SDK from github using git. 

Step 1:   

Clone the emsdk repo : git clone [https://github.com/emscripten-core/emsdk.git](https://github.com/emscripten-core/emsdk.git) and Enter inside the directory emsdk.

```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
```

Step 2:

Next, We will install the necessary tools like java, python using the below command.

```bash
./emsdk install latest
```

<img src="../../../images/Pasted image 20211031111809.png">

Step 3:

To activate latest SDK execute following command in your terminal


```bash
./emsdk activate latest
```

<img src="../../../images/Pasted image 20211031111827.png">

Step 4:

To activate PATH and other environment variables run following command in your terminal.

```bash
source ./emsdk_env.sh
```

 <img src="../../../images/Pasted image 20211031111901.png">


To permanently add this path for EMSDK, use below command :

Note : Make sure to give the proper to `emsdk_env.sh file`

```bash
echo 'source "/home/kali/emsdk/emsdk_env.sh"' >>$HOME/.bashrc 
```

Now the installation of emsdk is completed and can now can now compile C code.

The process of compiling C to WebAssembly helps us to understand a basic process of how to use WebAssembly.

-   Write code in a language
-   Compile to “.wasm“
-   Load “.wasm“ in a browser
-   Use WebAssembly JavaScript API to compile and instantiate the module
-   Call exported WebAssembly functions in JavaScript

  

Let’s create a simple program like below :

  ```c
#include <stdio.h>

int main() {
  printf("  Hello there! You made it to first step towards learning WebAssembly! Cheers!! :) \n ");
  return 0;
}
```
  
Now, we have to compile it using below command :

  ```bash
emcc hello.c -o hello.html -s WASM=1
```

<img src="../../../images/Pasted image 20211031113310.png">

 After compilation, if you have tried to open the file hello.html with certain browsers , You may get an error message instead of output that you want.
 
That is because the operation of loading of the WASM module is asynchronous, and some browsers (for security reasons) do not allow you to do this if the URL is of the kind `file://path/to/file/file.html`.  

In order to solve that issue you can  run a local server this way using below command :

```bash
python -m http.server 8080
```

<img src="../../../images/Pasted image 20211031113332.png">

Go to browser and type in localhost:8080 or http://127.0.0.1:8080/

<img src="../../../images/Pasted image 20211031113525.png">

Now you should be able to see other three files in the same directory: `Hello.html`, `Hello.js` and `Hello.wasm`.

-   `Hello.wasm` is the file that contains the Webassembly code (the compiled code).
-   `Hello.js` is the "glue code" needed to allow JavaScript to call and "communicate" with WASM compiled code. Emscripten generates this automatically and it is absolutely needed in order to run WASM modules. If you compile without the `-s WASM=1` flag this file will contain also the content of hello.wasm (but makes no difference in reality).
-   `Hello.html` is just a web page automatically generated that shows the result of your Webassembly code in a user friendly way. You don't actually need this file, but it is a cool way to quickly visualize what are you doing. You can tell Emscripten not to generate it by just using `-o Hello.js` instead of `-o Hello.html` (everything else remains as before).
	
Output :



<img src="../../../images/Pasted image 20211031120708.png">



	
	
