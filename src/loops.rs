// Loops are used to iterate untill a condition is met.

pub fn run() {
    let mut count:u8 = 0;
    
    // Infinit Loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);
    //     if count == 20 {
    //         break;
    //     }
    // }

    // While Loop
    while count <= 30 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz")
        }

        // Inc
        count += 1;
    }

    // for loop
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz")
        }
    }
}