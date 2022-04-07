#include <stdio.h>
main(void)
{
float num1, num2;
char op;


printf("Inform a number and simple operations \n");
scanf("%f %c %f", &num1, &op, &num2);


if (op == '+')
printf(" = %.2f", num1 + num2);
else if (op == '-')
printf(" = %.2f", num1 - num2);
else if (op == '/')
printf(" = %.2f", num1 / num2);
else if (op == '*')
printf(" = %.2f", num1 * num2);
else
printf("invalid operator, try again.");
printf("\n");
}