// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4];

    // Change value
    nums[2] = 20;

    // Add on to vector
    nums.push(5);
    nums.push(6);

    // Pop off last value
    nums.pop();

    println!("{:?}", nums);

    // Get single val
    println!("First value: {}", nums[0]);

    // Get vector length
    println!("Vector length: {}", nums.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&nums));

    // Get slice
    let slice: &[i32] = &nums[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in nums.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in nums.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec: {:?}", nums);
}
