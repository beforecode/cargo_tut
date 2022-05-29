// Arrays are fixed length where the elements are the same data types

use std::mem;
pub fn run() {
    let mut numbers: [i32;5] = [1,2,3,4,5];
    
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
    println!("{}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("slice {:?}", slice);


}