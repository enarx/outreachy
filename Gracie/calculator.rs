use std::io::{stdin,stdout,Write};

fn read(input: &mut String){
    stdout().flush()
            .expect("failed to flush");
    stdin().read_line(input)
           .expect("failed to read");
}
fn main(){
    println("CALCULATOR")
    println!("--------")

    loop{
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        print!("Enter your first number...")
        read(&mut num1)

        print!("Enter your second number...")
        read(&mut num2)

        print!("What operation would you like to do [+-*/]: ");
        read(&mut operator);
        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        let operators = String::from("+-*/");

        if !operators.contains(operator){
            println!("unknown operator")
            continue;

        }

        let result = match operator{
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 - num2,
            '/' => num1 - num2,
            => panic("error in operator")

        };

        println!("the result of {} {} {} = {}", num1,operator,num2,result);
    }
}
