I won't wonder if you said you never heard of this language before, ya that's because it's extreamly latest, compact, and damn simple to understand.

[Grain](https://grain-lang.org/) is a programming language that brings wonderful features from academic and functional programming languages to the 21st century. 

One of the most exciting things about Grain is that it compiles to WebAssembly. As such, Grain can run in the browser, on your computer, on a server, and potentially elsewhere. This guide will largely focus on the browser and terminal.

Grain is strongly typed (with a typechecker from OCaml), and its type inference significantly reduces the need for type annotations. Besides the WebAssembly core data types (e.g., i32 becomes Int32), Grain also provides composite types that are commonly used in high-level typed languages. 

Moving ahead let's setup the environment first.

1. [Install Grain Locally](https://grain-lang.org/docs/getting_grain).

2. [Install Wasmtime](https://wasmtime.dev/) for Altenative/manual method visit the site or simply run this command regardless of your operating system. to execute .wasm file. 
```
curl https://wasmtime.dev/install.sh -sSf | bash
```
Verify installation 
` wasmtime --version`

Yuss! so with this you are all set to run your First Grain program.

2. Progaram
    
    Open your preferred text editor and make a file with .gr extension. Grain recommends [VS Code](https://code.visualstudio.com/docs/setup/setup-overview) to work with .gr files. Installation of grain extension for VS code is highly recommended.
    
    The Fibonacci code in Grain is as follows :
    
    ```
    let rec fibonacci = (n) => {
      if (n == 0 || n == 1) {
        n
      } else {
        fibonacci(n - 1) + fibonacci(n - 2)
      }
    }
    
    print(fibonacci(7))
    ```
    
    We need to mention that a function is recursive with the keyword `rec`in Grain.

    Let's try somthing intresting ! What about taking  your Food orders 
```
    enum Topping { Cheese, Pepperoni, Peppers, Pineapple }
    enum Order { Pizza(Topping), Calzone(Topping) }

    record Person { name: String, order: Order }

    let person = { name: "Kirtee", order: Calzone(Pepperoni) }

    match (person) {
    { order: Pizza(_), _ } => print("All pizzas are great here."),
    { order: Calzone(Peppers), _ } => print("Someone with great taste!"),
    { order: _, _ } => print("Yep, that's an order.")
  }
```
  
3. Compiling the code
    
    To compile your Grain code, simply run :
    
    ```
    grain fibonacci.gr
    ```
    
    This would print `13` on your terminal and generate a `fibonacci.gr.wasm` file.
    

4. Running .wasm file in Wasmtime
    
    To run our .wasm file in wasmtime, run the following command
    
    ```
    wasmtime fibonacci.gr.wasm
    ```
    And you are good to go, this too will return the same output as above i.e. `13`

