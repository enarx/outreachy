// C program to find the Minimum length Unsorted Subarray,
// sorting which makes the complete array sorted
#include<stdio.h>

void printUnsorted(int arr[], int n)
{
int s = 0, e = n-1, i, max, min;

for (s = 0; s < n-1; s++)
{
	if (arr[s] > arr[s+1])
	break;
}
if (s == n-1)
{
	printf("The complete array is sorted");
	return;
}

for(e = n - 1; e > 0; e--)
{
	if(arr[e] < arr[e-1])
	break;
}

max = arr[s]; min = arr[s];
for(i = s + 1; i <= e; i++)
{
	if(arr[i] > max)
	max = arr[i];
	if(arr[i] < min)
	min = arr[i];
}

for( i = 0; i < s; i++)
{
	if(arr[i] > min)
	{
	s = i;
	break;
	}	
}

for( i = n -1; i >= e+1; i--)
{
	if(arr[i] < max)
	{
	e = i;
	break;
	}
}
	
printf(" The unsorted subarray which makes the given array "
		" sorted lies between the indees %d and %d", s, e);
return;
}

int main()
{
int arr[] = {10, 12, 20, 30, 25, 40, 32, 31, 35, 50, 60};
int arr_size = sizeof(arr)/sizeof(arr[0]);
printUnsorted(arr, arr_size);
getchar();
return 0;
}
