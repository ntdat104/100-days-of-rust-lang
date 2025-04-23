pub fn run() {
    // Print to console
    println!("Hello from the console!");

    // Basic Formatting
    println!("My name is {}", "Bob");

    // Positional Arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named Arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
    
}