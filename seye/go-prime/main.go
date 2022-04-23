// Go program to check prime numbers

package main

import (
	"fmt"
	"math"
)

func Primenumber(number int) {
	if number < 2 {
		fmt.Println("Number too small for a prime numberber.")
		return
	} else {
		root := int(math.Sqrt(float64(number)))
		for i := 2; i <= root; i++ {
			if number%i == 0 {
				fmt.Println("NUmber is not prime.")
				return
			}
		}
		fmt.Println("NUmber is prime")
	}

}

//main function
func main() {
	Primenumber(0)
	Primenumber(2)
	Primenumber(16)

}
