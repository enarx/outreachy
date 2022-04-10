#include<stdio.h>
#include<time.h>

int main(){

  int count = 0;
  double totalTime;
  clock_t startTime = clock();

  for(int i = 0; i<1000000; i++){
    ++count;
  }
  clock_t endTime = clock();
  totalTime = (double)(endTime - startTime) / CLOCKS_PER_SEC;

  printf("Total time taken to count %d : is %f seconds\n", count, totalTime);

return 0;
}
