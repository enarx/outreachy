<h1>Let's do a "Hello world" and "Fibonacci Series" program and compile it to WebAssembly</h1>

## Tool Setup
For tool setup , [refer to tutorial](https://github.com/enarx/outreachy/tree/main/shravi24/Demos/Inside%20Browser/C)
<p>&nbsp;</p>

## Writing Code
 Let’s create a simple C++ code to print Hello World and fibonacci series :
  
  ```c++
#include <iostream>
using namespace std;


int fibonacci(int p)
{
    if(p==0 || p==1)
    return 1;
    else
    return fibonacci(p-1)+fibonacci(p-2);
}


int main()
{
    int i=0;
    cout<< "Hello World from C++! " <<endl;
    cout<<"Fibonacci Series is : "<<fibonacci(5)<<endl;    
    
    return 0;
}

```
  
Now, we have to compile it using below command :
  
  ```bash
emcc Hello.cpp -o hello.html -s WASM=1 
```


<img src="../../../images/Pasted image 20211031113310.png">

 After compilation, if you have tried to open the file hello.html with certain browsers, you may get an error message instead of output that you want.
 
 In order to solve that issue you can  run a local server this way using below command :

```bash
python -m http.server 8080
```

 <img src="../../../images/Pasted image 20211031113332.png">
 
Go to browser and type in localhost:8080 

Now you should be able to see other three files in the same directory: `Hello.html`, `Hello.js` and `Hello.wasm`. Here click on `Hello.html` 

<img src="../../../images/Pasted image 20211031115849.png">
	
 

Hurray !! We got the output : 

<img src="../../../images/Pasted image 20211031120145.png">






