package main
import "fmt"
func main(){
   var n int
   fmt.Print("Enter a number: ")
   fmt.Scanf("%d", &n)
   for i:=0; i<=n; i++{
      for j:=0; j<n-i; j++{
         fmt.Printf(" ")
      }
      for k:=0; k<i; k++{
      fmt.Printf("*")
   }
   fmt.Println()
}
}