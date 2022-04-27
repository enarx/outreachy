# 2 - Implementing Quick Sort on server-side

In my previous post, I created [C++ in the web browser using web assembly](https://www.wasm.builders/gunjan_0307/c-in-the-web-browser-using-web-assembly-4b7e), in this post I will be implementing it server-side.

So, let's first install all the dependencies required.

1. [C++](https://docs.microsoft.com/en-us/cpp/build/vscpp-step-0-installation?view=msvc-170)
2. [Wasmer](https://docs.wasmer.io/) and [Wasienv](https://github.com/wasienv/wasienv)
3. [Wasmtime](https://wasmtime.dev/)

There are 3 steps 
1. Compile the C++ code
2. Compile C++ code to WASM Binary
3. Execute it using WASM Runtime

So, lets start.

## Compile the C++ code

- make the folder at any preferred location in your machine
- open your favorite code editor and write Quick Sort C++ 
  code. 
- Save the file with .cpp extension.
  (Quick sort code can be found [here](https://www.geeksforgeeks.org/quick-sort/))



![Terminal-ss](https://www.wasm.builders/remoteimages/uploads/articles/ovzb24lxcx6ks1kfyctl.png)

- complile and execute C++ code.

`g++ quick-sort.cpp` followed by `./a.out`

![Terminal-ss](https://www.wasm.builders/remoteimages/uploads/articles/4ce4p4by7sleb6os0k5e.png)

## Compile C++ code to WASM Binary
It can be done using following command. 
We have now converted our c++ code to web assembly code.

`wasic++ quick-sort.cpp -o quick-sort.wasm`

## Execute it using WASM Runtime
Execute the following command to see the results.
`wasmtime quick-sort.wasm`

![Terminal-ss](https://www.wasm.builders/remoteimages/uploads/articles/dlp9y3u6rrjbkd41lzkb.png)

Happy Hacking :)

[Tutorial for implementation](https://www.wasm.builders/gunjan_0307/implementing-quick-sort-on-server-side-4f33)

