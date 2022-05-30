// Funtions are used to store blocks of code to re-use.

pub fn run() {
    greeting("hello", "Yassine");
    
    // Bind function values to variables
    let get_sum = add(5,5);
    println!("5 + 5 = {}", get_sum);

    // closure
    let n3: i32 = 3;
    let add_sum = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("closure sum: {}", add_sum(3,5));
}

fn greeting(greet: &str, name: &str) {
    println!("{} Mr {}, nice to mmet you", greet, name);
}

fn add(n1: i32,n2: i32) -> i32 {
    n1 + n2
}