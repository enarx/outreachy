#include<stdio.h>
#include<string.h>

int main(){
  printf("\n\n So Welcome to Amazing world of WASM\n" );
}

int callX(int number){
  printf("Called by JS\n");
  return 1;
}
