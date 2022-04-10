<h1>Let's do a "Hello world" program in Rust and Compile it into WebAssembly and run using Wasm-pack</h1>

## Pre-requisites :
Please ensure that you have installed [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
<p>&nbsp;</p>

## Tool Setup

We need to install WASM Pack
```bash
cargo install wasm-pack
```

You can see like this when you execute above command :

<img src="../../../images/Pasted image 20211031123450.png">

<p>&nbsp;</p>

## Rust Code
Let's create a new Rust project for the "Hello world":

```bash
cargo new --lib hello-world-wasm
```
Here my project name is hello-world-wasm
  
You can also refer to below picture :

 <img src="../../../images/Pasted image 20211031124526.png">

After above step, if you go to the folder where you have created a Rust project, you can see the files that got created.
In this step, you can use any IDE of your choice. Here I have used Visual Studio. In visual studio, open your Rust project folder. In Cargo.toml file we are going to add the following

```rust
[package]
name = "hello_world-wasm"
version = "0.1.0"
authors = ["Shraddha Inamdar <shraddha.inamdar95@gmail.com>"]
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.74"

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Oz", "--enable-mutable-globals"]

```

- `cdylib` :  lib for wasm final artifacts.
- `wasm-bindgen`: dependency to facilitate high-level interactions between Wasm modules and JavaScript.
-`--enable-mutable-globals` : in order to work with Strings.

WebAssembly only supports the 4 value types (i32, u32, i64, and u64). If you want to work with other types, such as String or Objects, you must first encode them. However, `wasm-bindgen` does these bindings for us. There's no need to worry about it anymore. That said, let's create our helloworld function to return a String in src/lib.rs:
  
  ```rust
  
  use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn helloworld() -> String {
    String::from("Hello world from Rust!")
}

  
  ```
 <p>&nbsp;</p> 
 
## Compiling Rust Code
Let us compile code with:

```bash
wasm-pack build --target web
```

We are using the web target, however, there are different targets we can use depending on how we want to use that wasm file:

   ```bash
    --target bundler - for bundlers like Webpack, Parcel, or Rollup.
    --target web - for the web as ECMAScript module.
    --target no-modules - for the web without ECMAScript module.
    --target nodejs - for Node.js
	
   ```

After executing the above command, a pkg directory will have been created with our JavaScript library containing the code we have made in Rust! It even generates the "types" files of TypeScript.

<img src="../../../images/Pasted image 20211031125040.png">


Now, we can use JavaScript package in our project.

The .js file contains the necessary "glue" code that works with other packages and carries out bindings and other stuff.

Let us use the compiled code on our JS project

In order to use the wasm file in our JavaScript, we need to import the generated pkg module to our project. Let's create an index.html in the Rust project (in my case it is in the /src folder from the root directory) :

```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="UTF-8"/>
        <title>"Hello world" in Rust + WebAssembly</title>
        <base href=".">
        <script type="module">
            import init, {helloworld} from '../pkg/hello_world_wasm.js'

            async function run() {
                await init()
                document.body.textContent = helloworld()
            }

            run()
        </script>
    </head>
</html>

```

Now we only need to render this file on the browser.
Most browsers give error when file:// is used to open the html files(directly through folder).
So we can use Python Server to render the html file on browser :

You can  run a python local server using below command :

```bash
python -m http.server 8080
```

<img src="../../../images/Pasted image 20211031113332.png">

<p>&nbsp;</p>

Now if you go to browser and type in localhost:8080 or http://127.0.0.1:8080/ , you can see the files that are created in directory  :

<p>&nbsp;</p>

<img src="../../../images/Pasted image 20211031125604.png">

<p>&nbsp;</p>

After this just click on `hello_world-wasm/` and then click on `src/` to see the output.

<p>&nbsp;</p>

<img src="../../../images/Pasted image 20211031220716.png">

<p>&nbsp;</p>

Hurray!! We got the output :

<p>&nbsp;</p>

<img src="../../../images/Pasted image 20211031121707.png">

	
