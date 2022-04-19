print('Meal Price Calculator')

child_meal = float(input("What is the price of a child's meal? "))
adult_meal = float(input("What is the price of a adult's meal? "))
children = int(input("How many children are there? "))
adults = int(input("How many adults are there? "))
tax = float(input("What is the sales tax rate? "))
print(1 * '\n' )

meal_total = (child_meal * children) + (adult_meal * adults)
print("Subtotal: $",meal_total)

sales_tax = (meal_total*(tax/100))
print("Sales Tax: ${:.2f}" .format(sales_tax))

total = meal_total + sales_tax
print("Total: ${:.2f}".format(total))
print(1 * '\n' )

payment = float(input("What is the payment amount? "))
change = (payment - total)
print("Change: ${:.2f}".format(change))