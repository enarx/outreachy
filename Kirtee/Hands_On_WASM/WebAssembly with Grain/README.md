I won't wonder if you said you never heard of this language, ya that's because it's extremely latest, compact, and it's damn simple to understand.

[Grain](https://grain-lang.org/) is a programming language that brings wonderful features from academic and functional programming languages to the 21st century. 

One of the most exciting things about Grain is that it compiles to WebAssembly. As such, Grain can run in the browser, on your computer, on a server, and potentially elsewhere. This guide will largely focus on the browser and terminal.

Grain is strongly typed (with a type checker from OCaml), and its type inference significantly reduces the need for type annotations. Besides the WebAssembly core data types (e.g., i32 becomes Int32), Grain also provides composite types that are commonly used in high-level typed languages. 

Moving ahead let's set up the environment first.

1. [Install Grain Locally](https://grain-lang.org/docs/getting_grain).

2. [Install Wasmtime](https://wasmtime.dev/) for an Alternative/manual method visit the site or simply run this command regardless of your operating system. to execute the .wasm file. 
```
curl https://wasmtime.dev/install.sh -sSf | bash
```
Verify installation 
` wasmtime --version`

Yuss! so with this you are all set to run your First Grain program.

2. Program
    
Open your preferred text editor and make a file with the .gr extension. Grain recommends [VS Code](https://code.visualstudio.com/docs/setup/setup-overview) to work with .gr files. Installation of grain extension for VS code is highly recommended.
    
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

Let's try something interesting! What about taking  your Food orders 
```
 enum Topping { Cheese, Pepperoni, Peppers, Pineapple }
 enum Order { Pizza(Topping), Calzone(Topping) }

 record Person { name: String, order: Order }

 let person = { name: "Steve", order: Calzone(Pepperoni) }

 match (person) {
 { order: Pizza(_), _ } => print("All pizzas are great here."),
 { order: Calzone(Peppers), _ } => print("Someone with great taste!"),
 { order: _, _ } => print("Yep, that's an order.")
}
```
  
3. Compiling the code
To compile your Grain code, simply run :

- For order.gr
    ```
    grain Order.gr
    ```
This would print `Yep, that's an order.` on your terminal and generate a `Order.gr.wasm` file.
- For Fibonacci.gr
    ```
    grain fibonacci.gr
    ```
 
This would print `13` on your terminal and generate a `fibonacci.gr.wasm` file.

4. Running .wasm file in Wasmtime
    
To run our .wasm file in wasmtime, run the following command
for Order.gr.wasm
```
wasmtime order.gr.wasm
```
For Fibonacci.gr.wasm 
```
wasmtime fibonacci.gr.wasm
```
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/h195o2lfnszvyx1wg5cx.png)
 
And you are good to go, this too will return the same output as above i.e. `13`

 

