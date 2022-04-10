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
