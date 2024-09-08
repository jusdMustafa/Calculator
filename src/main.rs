use std::io;
fn main() {
    
    // intro
    println!("------------------");
    println!("The Calculator Program");
    println!("------------------");

    // inputs
    println!("Please enter the first Number");

    let mut number1 = String::new();
    io::stdin()
        .read_line(&mut number1)
        .expect("Failed to read number");

    println!("Please enter the Second Number");

    let mut number2 = String::new();
    io::stdin()
        .read_line(&mut number2)
        .expect("Failed to read number");
    
    // Results
    println!("Select your process (| x | - | + | / |)");
    let mut process = String::new();
    io::stdin()
        .read_line(&mut process)
        .expect("Failed to read process");
    

    



}