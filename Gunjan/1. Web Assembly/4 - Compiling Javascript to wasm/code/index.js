
//Simple Program to calculate Fibonacci Sequence of an Integer Input
function fibonacci(num){
  var a = 1, b = 0, temp;

  while (num >= 0){
    temp = a;
    a = a + b;
    b = temp;
    num--;
  }
console.log("Fibonacci Term is ",b);

}

var Shopify = {
  main: fibonacci
};