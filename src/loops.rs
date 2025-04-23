// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 10 {
            break;
        }
    }

    // While Loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    // For range
    for x in 0..5 {
        println!("Number: {}", x);
    }
}