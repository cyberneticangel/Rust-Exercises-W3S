use std::io;
// Accept two numbers from the user, do arithmetic operations on them and then print them

fn main() {
    println!("Enter a number: ");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please type a number!");
    println!("Enter another number: ");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please type a number!");

    add_inputs(num1, num2);
    sub_inputs(num1, num2);
    mul_inputs(num1, num2);
    div_inputs(num1, num2);
}

fn add_inputs(a: f64, b: f64) -> f64 {
    let sum: f64 = a + b;
    println!("sum: {}", sum);
    return sum;
}

fn sub_inputs(a: f64, b: f64) -> f64 {
    let sum: f64 = a - b;
    println!("sum: {}", sum);
    return sum;
}

fn mul_inputs(a: f64, b: f64) -> f64 {
    let sum: f64 = a * b;
    println!("sum: {}", sum);
    return sum;
}

fn div_inputs(a: f64, b: f64) -> f64 {
    let sum: f64 = a / b;
    println!("sum: {}", sum);
    return sum;
}
