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

