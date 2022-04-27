# 3 - Compling functions from multiple files in C++ using wasm

In my earlier post which was about "[Implementing Quick sort on server-side](https://www.wasm.builders/gunjan_0307/implementing-quick-sort-on-server-side-4f33)" we compiled functions from our c++ file itself, but in this post, we will be calling functions from multiple other files, and hence compile them together in our main C++ file.

So, for this, I will be demonstrating simple calculator, in which all the functions will be imported from custom header files.

For setting up the dev environment, you can follow my previous [post](https://www.wasm.builders/gunjan_0307/implementing-quick-sort-on-server-side-4f33)

So, lets start with directory structure you need to make

```
- Calculator
      - main.cpp
      - add.h
      - subtract.h
      - multiply.h
      - division.h
```
So, lets start with writing some code in these files -

1) main.cpp - This will be our main file having main function, which will call all the functions. 
_Keep in mind we can have only one main function in our all files together, otherwise wasm will throw error_

```
#include<iostream>
#include"add.h"
#include"subtract.h"
#include"multiply.h"
#include"division.h"
using namespace std;

void calculate(int choice,int a,int b){
  switch(choice) {
  case 1:
    cout<<"Added ans is - "<<add(a,b)<<endl;
    break;
  case 2:
    cout<<"Subtracted ans is - "<<subtract(a,b)<<endl;
    break;
  case 3:
    cout<<"Multiplied ans is - "<<multiply(a,b)<<endl;
    break; 
  case 4:
    cout<<"Divided ans is - "<<division(a,b)<<endl;
    break;   
  default:
    cout<<"Enter valid choice";
 }
}
int main()
{
    int a,b,choice;

    cout << "To stop enter any character";
    cout << "Press : 1 for add | 2 for subtract | 3 for multiply | 4 for divide"<<endl;
    cout<<"Enter your choice - ";

    while (cin >> choice){
    cout<<"Enter 2 numbers - ";
    cin>>a>>b;
    calculate(choice,a,b);
    cout<<"Enter another choice - ";
    }
	return 0;
}
```
2) add.h file

```
int add(int a, int b)
{
    return(a + b);
}  
```
3) subtract.h file

```
int subtract(int a, int b)
{
    if(a<b) return 0;
    return(a - b);
}
```
4) multiply.h file

```
int multiply(int a, int b)
{
    return(a * b);
}
```
5) division.h file

```
int division(int a, int b)
{
    if(a<b) return 0;
    return(a / b);
}
```
Now, we will convert our code to wasm binary code using - 

```
wasic++ main.cpp -o calculator.wasm
```
Now, we can execute this wasm file using - 

```
wasmtime calculator.wasm
```

![calculator](https://www.wasm.builders/remoteimages/uploads/articles/8x197gq3e4qpywqc5lvd.png)

It was that easy 😇!!

[Tutorial for implementation](https://www.wasm.builders/gunjan_0307/compling-functions-from-multiple-files-in-c-using-wasm-2bfb)

