        // Libraries in rust
        use std::io;
        use std::{i32};
fn main() {
    println!("Hello, world!!");
    // Request for entering number 1
          println!("Enter First number ? ");
          let mut input1 = String::new();
          io::stdin().read_line(&mut input1).expect("Failed to read
          line");

          // Request for entering number 2
          println!("Enter second number ? ");
          let mut input2 = String::new();
          io::stdin().read_line(&mut input2).expect("Failed to read
          line");

          // Converting string to integer
          let aint: i32 = input1.trim().parse().ok().expect("Program
          only
          processes numbers, Enter number");
          let bint: i32 = input2.trim().parse().ok().expect("Program
          only
          processes numbers, Enter number");

          // Output of basic operations
          println!("sum is: {}", aint + bint);
          
}
