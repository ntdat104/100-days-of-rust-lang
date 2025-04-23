// Functions - Used to store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Alice");
    println!("5 + 4 = {}", add(5, 4));

    // Closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure: 5 + 4 + {n3} = {}", add_nums(5, 4));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}