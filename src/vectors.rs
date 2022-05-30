// Vectors are resizable Arrays
// Array

use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4];
    //loop
    // for token in numbers {
    //     println!("{}", token);
    // }

    //Get single value
    println!("{}", numbers[0]);
    
    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);
    
    // Array length
    println!("Vector length : {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Vector slice {:?}", slice);

    // Push to Vector
    numbers.push(5);
    numbers.push(100);
    println!("After pushing: {:?}", numbers);

    // Pop off the last value
    numbers.pop();
    println!("After poping off: {:?}", numbers);

    // Loop thrw Vector value
    // for token in numbers.iter() {
    //     println!("token: {}", token);
    // }

    // Loop & mutats values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("After looping and mutating values: {:?}", numbers);
    
}