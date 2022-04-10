![Alt text](/Jennifer/images/4f93d37b538c.jpg?raw=true)

# WebAssembly with Golang - Example

## Environment setup

To compile this demo, you must install the following.

#### Golang

Go to [go.dev](https://go.dev/) and follow the instructions using rustup.

#### tinygo

Go to [tinygo.org](https://tinygo.org/getting-started/install/) and follow the instructions using rustup.

please note: tinygo requires go version 1.15 through 1.17

#### Wasmtime

You will find wasmtime at [wasmtime.dev](https://wasmtime.dev/)


First example, we would create a simple Go program that checks in an integer input is odd or even.

create a folder with a name of your choice, i would be using "Golang-to-WASI" as the name of my folder.

```bash
cd Golang-to-WASI
go mod init Golang-to-WASI
```
create a file `main.go`, add following code into your main.go file and save the file.

```bash
// Simple Program to Check Entered Number is Even or Odd

package main

import "fmt"

func main(){
    fmt.Print("Enter number : ")
    var n int
    fmt.Scanln(&n)
    
    if(n%2==0){
        fmt.Println(n,"is an Even number")
    }else{
        fmt.Println(n,"is Odd number")
    }
}

```
Run your code and provide input to see the result
```
go run main.go
```
![Alt text](/Jennifer/images/Screenshot1.png?raw=true)

We will can compile to WASM using the following command:
```
tinygo build -wasm-abi=generic -target=wasi -o main.wasm main.go
```

Now let us run the WebAssembly generated using wasmtime
```bash
wasmtime main.wasm
```
![Alt text](/Jennifer/images/Screenshot2.png?raw=true)