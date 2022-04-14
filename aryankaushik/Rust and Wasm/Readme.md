# Calling a Rust program using Wasm

![1.1](https://github.com/aryankaushik-git/outreachy/blob/main/aryankaushik/media/WebAssembly_1%20(2).png)

Hi Folks, Welcome to another Blog. Today, we'll learn about how to call a rust program using Wasm.
Getting Started:

#### 1. We need rust and wasm-pack in our system, lets install them:



a. For Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
b. For wasm-pack

```
cargo install wasm-pack
```


![1](https://www.wasm.builders/remoteimages/uploads/articles/enhbe2zwpb2ztmjxms2x.png)


![2](https://www.wasm.builders/remoteimages/uploads/articles/7rnjlg0chtgeezzmc9st.png)

> 
Cargo is a Rust package manager. Cargo downloads your Rust package's dependencies, compiles your packages, makes distributable packages, and uploads them to crates.io, the Rust community's package registry.





#### 2. Create a Library, here I've named it 
` "ary-kaush"`

```
cargo new --lib ary-kaush
```


![3](https://www.wasm.builders/remoteimages/uploads/articles/7u7tzi2k9uj6ynp7syr5.png)

#### 3. Move to the library, 
`cd ary-kaush/`


![4](https://www.wasm.builders/remoteimages/uploads/articles/bvsn2p8o8pnxipd4k2v5.png)

and open vs code by using `code .` command.

we can see that there is a folder named src and a file named cargo.toml.


![z](https://www.wasm.builders/remoteimages/uploads/articles/p8vpt4eqotnbay4c0td2.png)


#### 4. In order to create a wasm module, we need to specify the type of package.
For adding such specification we need to modify `Cargo.toml` file. By adding:
a. crate type:
```
[lib]
crate-type = ["cdylib"]
```
b. dependancies:

```
wasm-bindgen = "0.2.78"
```


![x](https://www.wasm.builders/remoteimages/uploads/articles/9gh0rw9eyobxzkuv047u.png)


#### 5. Change default code in src/lib.rs to the code that we wan't to run 

> 
here we are making a function to add two numbers.


![5](https://www.wasm.builders/remoteimages/uploads/articles/lsnfhzgrvnmifytk38gn.png)

#### 6. Now building the package

```
wasm-pack build
```

![6](https://www.wasm.builders/remoteimages/uploads/articles/943n8ti76s30mb7g4pmf.png)

after build, a lot of files has been created to support the package.

#### 7. In order to run this program, run:

```
wasm-pack build --target nodjs
```
 

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/hbpsx9yooz81mmp8gal4.png)

A pkg folder will be created which includes .wasm,.ts, js and .json files.

#### 8. To run our program we need to create a index.js file and where we will make a math library.

```
const math  = require('./ary_kaush.js')

console.log(math.add2numbers(10,20));
```
In VScode
![8.1](https://www.wasm.builders/remoteimages/uploads/articles/vgtkgklsoiyyhriog34c.png)

#### 9. move in pkg by `cd pkg/` and run 

```
node index.js
```
![8]
(https://www.wasm.builders/remoteimages/uploads/articles/nfhc8b4hbgf48inez6cb.png)

and Finally we have the required output.


![Image description](https://www.wasm.builders/remoteimages/uploads/articles/x79it0dmwj8owxsm2qxh.jpg)

This was a basic program in rust, whereas we can create multiple projects e.g. a calculator, fibonacci series, to find whether the number is prime or not etc. .

Please check out [blog](https://www.wasm.builders/moksh_pathak/how-i-made-a-calculator-in-rust-and-ran-it-in-an-enarx-keep-part-1-4lic) by @moksh_pathak , where he created a calculator using Rust and ran it in Enarx.

Stay tuned for more stuff!

Do comment your ideas and suggestions related to the blog and please share if you found it useful.
Write your queries in comment section, we'll help you to resolve your errors.
