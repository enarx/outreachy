JavaScript is found to be the most beloved language by web developers and Compiling it with webassembly will be a great approach.

Javascript is an extremely dynamic language and it was expected it would have large runtime for compilation if done to webassembly.

Before we begin you have to install certain toolchains to compile your javascript code.

# Environment Setup

## 1. Install [Javy](https://github.com/kirteeprajapati/javy) Toolchain

- First fork the Javy repo and then clone it to your desktop 
```
$ https://github.com/<username>/javy.git
```
The username will be filled once you fork the repo.

- Navigate to javy directory
```
$ cd javy
```
- Install dependencies

**Build**

1. [Rustup installer](https://rustup.rs/) for Rust programing language
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. wasm32-wasi
```
$ rustup target add wasm32-wasi
```
3. wasi-sdk
```
$ make download-wasi-sdk
```

**Development**

1. wasmtime-cli
```
$ cargo install wasmtime-cli
```
2. cargo-wasi
```
$ cargo install cargo-wasi
```
**Building**
```
# access to the executable in target/release/javy
$ make

# alternatively for global installation.
$ make && cargo install --path crates/cli
```
Now come out of the directory
```
$ cd ~/
```
## 2. get msgpack-tools
Are command-line utilities, that Convert MessagePack to JSON and vice-versa.
- `msgpack2json` -- Convert MessagePack to JSON
- `json2msgpack` -- Convert JSON to MessagePack
we'll be using these commands while compiling.

1. **[Installation](https://github.com/ludocode/msgpack-tools#installation)**
Can be installed directly via git-repo or navigate the link provided above for alternative methods
```
$ git clone https://github.com/ludocode/msgpack-tools.git
```
Inside masgpack-tools run the following and return the home directory
```
$ cd msgpack-tools
$ ./configure && make && sudo make install
$ cd ~/
```
3. Install [Wasmtime](https://wasmtime.dev/)
```
curl https://wasmtime.dev/install.sh -sSf | bash
```
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/3ldqfhlnpianf4lv1cz8.png)
###### at your home directory you'll be having this tools installed

# Compile JavaScript Code

Note: If you run `make` while **Building** globally then you can run any `.js` folder outside of it 
but in my case, I run it locally with the command `make` 

navigate to the release folder and create a .js file for code snippets and 

.json file for providing input.

> the default implementation of Javy expects a msgpack input to be sent through stdin and so we created a .json file.
```
$ cd /javy/target/release
$ touch index.js
$ touch input.json
```
inside your .js file paste your code snippet

**Pascal's Triangle**
```
function generatePascal(n) {
    //2D array
    var arr = [];
    var tmp;
    for (var i = 0; i < n; i++) {
        //Each element in array is in turn an array
        // The index is the row number and the array values are the row values
        arr[i] = [];
        for (var j = 0; j <= i; j++) {
            //If index is last element the value is always 1
            if (j == i) {
                arr[i].push(1);
            } else {
                //The following line adds up the two numbers directly above this element.
                tmp = (!!arr[i - 1][j - 1] ? arr[i - 1][j - 1] : 0) + (!!arr[i - 1][j] ? arr[i - 1][j] : 0);
                arr[i].push(tmp);
            }
        }
    }
    return arr;
}
```
Inside .json file provide your input
I provided `15`.

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/1bi4b44imi8ba4vczs1i.png) 

### Convert JavaScript(.js) to WebAssembly(.wasm)
```
$ javy index.js -o index.wasm
```
### Convert `JSON` to `msgpack` format
```
$ json2msgpack -i input.json | wasmtime run index.wasm | msgpack2json
```
Output in the terminal would be 
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/ofm4ivlylpiihy4s9itj.png)


## References:
1. https://enarx.dev/docs/WebAssembly/JavaScript
2. https://www.wasm.builders/deepanshu1484/javascript-and-wasi-24k8
3. https://github.com/ludocode/msgpack-tools#installation
4. https://wasmtime.dev/
5. https://github.com/ludocode/msgpack-tools/releases/tag/v0.6
6. https://github.com/WebAssembly/design/issues/219