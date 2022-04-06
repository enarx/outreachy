// Reverse a No.

#include<stdio.h>  
 int main()    
{    
int n, reverse=0, rem;    
printf("Enter a number: \n");    
  scanf("%d", &n);    
  while(n!=0)    
  {    
     rem=n%10;    
     reverse=reverse*10+rem;    
     n/=10;    
  }    
  printf("Reversed Number: %d \n",reverse);    
return 0;  
}  