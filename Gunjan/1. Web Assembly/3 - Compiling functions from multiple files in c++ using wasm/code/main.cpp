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
