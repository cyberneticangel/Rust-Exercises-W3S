// 3. Write a Rust program that declares a mutable variable counter and initializes it with 0.
// Then increase it by 1 and decrease it by 1.
// At the end, print the variable value for each stage.
// TODO: Refactor to use for loop
fn main() {
    let mut counter = 0;
    loop {
        println!("Counter initialized at: {counter}");
        counter += 1;
        println!("Counter after += 1: {counter}");
        counter -= 1;
        println!("Counter after -= 1: {counter}");
    }
}
