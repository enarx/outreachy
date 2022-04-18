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


### Golang program for implementation of Insertion Sort

```bash
cd WebAssembly_with_golang
go mod init Golang-to-WASI
```

create a file `main.go`, add following code into your main.go file and save the file.

// simple programm on Insertion sort.


```go
package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {

	slice := generateSlice(20)
	fmt.Println("\n--- Unsorted --- \n\n", slice)
	insertionsort(slice)
	fmt.Println("\n--- Sorted ---\n\n", slice, "\n")
}

// Generates a slice of size, size filled with random numbers
func generateSlice(size int) []int {

	slice := make([]int, size, size)
	rand.Seed(time.Now().UnixNano())
	for i := 0; i < size; i++ {
		slice[i] = rand.Intn(999) - rand.Intn(999)
	}
	return slice
}
 
func insertionsort(items []int) {
    var n = len(items)
    for i := 1; i < n; i++ {
        j := i
        for j > 0 {
            if items[j-1] > items[j] {
                items[j-1], items[j] = items[j], items[j-1]
            }
            j = j - 1
        }
    }
}
```


Run your code and provide input to see the result
```
go run main.go
```



(add picture here)


We will can compile to WASM using the following command:
```
tinygo build -wasm-abi=generic -target=wasi -o main.wasm main.go
```

Now let us run the WebAssembly generated using wasmtime
```bash
wasmtime main.wasm
```

add pictre here -- 








