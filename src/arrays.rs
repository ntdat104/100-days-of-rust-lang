// Arrrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut nums: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Change value
    nums[2] = 20;

    println!("{:?}", nums);

    // Get single val
    println!("First value: {}", nums[0]);

    // Get array length
    println!("Array length: {}", nums.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&nums));

    // Get slice
    let slice: &[i32] = &nums[1..3];
    println!("Slice: {:?}", slice);
}