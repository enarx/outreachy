****How to run Rust and WebAssembly on Windows****

I really went through a whole lot before i finally got this to work simply because i use windows, so i feel it would be helpful to share the process with other window users.

STEPS
1. The first thing is to install WSL (Windows sub-linux) This helps your windows behaves like linux that was why it was called windows sub-linux. Without this, any operation that is meant for linux won't work on windows. In essence, it gives you a way of simulating linux ,yet windows. So that stuffs that are for linux can convieniently run on windows.

You can now install everything you need to run Windows Subsystem for Linux (WSL) by entering this command in PowerShell or Windows Command Prompt and then restarting your machine.

```
wsl --install
```

2. Secondly, you need to get a linux distribution installed, for example i used ubuntu and you may decide to use any other linux distribution of your choice. You will have to download it and get it installed, the installation takes a little while. If you encounter no problem , you would see installation successful on your ubuntu terminal. After this you can now start running commands one after the other to get rust installed.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```
```
sudo apt update
```
```
sudo apt install git curl gcc pkg-config libssl-dev musl-tools python3-minimal
```
```
source $HOME/.cargo/env
```
```
rustup toolchain install nightly --allow-downgrade -t x86_64-unknown-linux-musl,x86_64-unknown-linux-gnu
```

So, what is next is to install the web Assembly runtime(wasmtime) by running the following command.


```
curl https://wasmtime.dev/install.sh -sSf | bash
```

If you encounter no error, you would see that Rust is successfully installed. Since the installation is successful, Now, the next thing to do is to create hello world in rust, compile it to web Assembly and run it using wasmtime.
First, reopen your terminal and create a file with the following command. It creates hello-world file for you in rust.

```
cargo new hello-world
```

The hello-world folder has a template for the hello world. Which you can make any changes to main.rs.In the target folder,A webassembly file is there.
The next thing is to cd into hello-world and let's compile to web assembly

```
rustup target add wasm32-wasi
```
The above command downloads some component for rustup target, when it's done then run this command again

```
cargo build --target wasm32-wasi --release
```

Now, let's move to the last step. Run the command :

```
wasmtime ./target/wasm32-wasi/release/hello-world.wasm
```

Congratulations! There you see Hello-world!. You can always make changes to the file and always print. So, because it's a windows OS, there is a special way to navigate to where your file is. And You'll have to run cargo build again after making changes to the file, so it would update it. Open your VS code ,open folder and type this:


```
\\wsl$
```

Meanwhile, your ubuntu or any other linux distribution that you have installed has to be opened because the above command  takes you there and you would see your linux system. Then navigate to that hello-world directory that you created earlier on (the root directory is home ). so you can make changes to the file. After that, go to ubuntu and  run the following command:

```
cargo build --target wasm32-wasi --release
```

Run this to print out:

```
wasmtime ./target/wasm32-wasi/release/hello-world.wasm
```

Now you would see that the file is updated. Remember any time you make changes you have to run cargo build .... to update it then you print out.

Congratulations! 
You are now running Rust and WebAssembly!
And I hope this tutorial helps!
Thank you!