# 1 - C++ in the web browser using web assembly

Your project is written in C++, but you want to ship it to the web. Impossible right!!🤔
No, that's definitely possible with [web assembly](https://enarx.dev/) in just simple 3 steps.🥳

1. Create a web assembly file(.wasm) using your C++ code(.cpp).
2. Load that file into web browser using JS.
3. Call a function contained in web assembly from JS code and 
   get the return value.

So, lets get started.
I am using Ubuntu 21.10 (64 bit).

---

## Create web assembly file

**Environment Setup**
To compile this demo, you must install the following:

1. [C++](https://docs.microsoft.com/en-us/cpp/build/vscpp-step-0-installation?view=msvc-170)
2. [Wasmer](https://docs.wasmer.io/) and [Wasienv](https://github.com/wasienv/wasienv)
3. [Wasmtime](https://wasmtime.dev/)

**C++ Code Snippet**
We will create a Simple C++ Program that will return us the factorial of an Integer Input.
Save it with any file name of your choice, I used Factorial.cpp.

```
int factorial(int n)
{
  if (n == 0)
    return 1;
  else
    return n * factorial(n-1);
}
```
**Compiling the C++ Code**

1. Compiling Using g++

```
g++ Factorial.cpp
```
2. Compile to WASM Binary

```
 wasic++ Factorial.cpp -o FactorialBinary.wasm
```
Hurray🎉 We have our C++ code converted to binary. Now, you just need to load the .wasm file and you can start executing functions from the .wasm file directly using Javascript.
So, lets do it.

## Load the file into web browser using JS and calling our factorial function

Create a HTML file and name it Factorial.html 

```
<!DOCTYPE html>
<html>
  <head></head>
  <body>
    <script type="module">
      (async() => {
        const response = await fetch('./FactorialBinary.wasm');
        const bytes = await response.arrayBuffer();
        const { instance } = await WebAssembly.instantiate(bytes);
        document.write("<h1>Your factorial is: ", instance.exports.factorial(8), "</h1>");
      })();
    </script>
  </body>
</html>
```
You can now open Factorial.html in your web browser and your C++ code is shipped on the web.🏁



![Result](https://www.wasm.builders/remoteimages/uploads/articles/idve9pyhketb74m4p659.png)

Happy Coding!! :)

[Tutorial for implementation](https://www.wasm.builders/gunjan_0307/c-in-the-web-browser-using-web-assembly-4b7e)

