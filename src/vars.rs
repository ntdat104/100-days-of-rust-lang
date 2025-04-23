// Variables hold primitive data or references to data
// Variable are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Bob";
    let mut age = 20;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    const PI: f32 = 3.14;
    println!("ID: {}", ID);
    println!("PI: {}", PI);

    // Assign multiple variables
    let (my_name, my_age) = ("Bob", 20);
    println!("My name is {} and I am {}", my_name, my_age);
}
