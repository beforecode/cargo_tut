//there are two types of strings:
// premitive stringd
// fised length strings

pub fn run() {
    //a premitive string
    let hello = "hello";

    //a fixed length string
    let mut day = String::from("Day ");
    println!("Length is : {}", day.len());
    
    // push a char
    day.push('1');
    println!("today is : {}", day);

    // push a string
    day.push_str(" is nice");
    println!("today is {}", day);

    //capacity : number of bytes it can store
    println!("capacity is {}", day.capacity());

    //check if sting is empty
    println!("is empty : {}", day.is_empty());

    //does it contain substring ?
    println!("does is contain 'nice' ? : {}", day.contains("nice"));

    //replace
    println!("replace: {}", day.replace("nice", "axsome"));

   let new_day = day.replace("nice", "awsome");

    //loop
   for token in new_day.split_whitespace() {
    println!("{}", token);
   }

   //Create string with specific capacity
   let mut s = String::with_capacity(10);
   s.push('a');
   s.push('b');

   //Asserting testing
   assert_eq!(2, s.len());
   assert_eq!(10, s.capacity());

   println!("{}", s);
}