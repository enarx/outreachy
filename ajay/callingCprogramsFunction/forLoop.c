#include<stdio.h>
#include<string.h>

int main(){
  printf("\n\n So Welcome to Amazing world of WASM\n" );
}


char * greetX(char * name){
  char * greetingMessage = "hello ";
  strcat(greetingMessage, name);

  return greetingMessage;
}

int wCount(int number){
  printf("Here in wcount fuction\n" );
  int count = 0;
  for(int i = 0; i< number; i++){
    count = count + 1;
  }
  return count;
}
