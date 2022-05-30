pub fn run() {
    let age: u8 = 16;
    let check_id:bool = true;
    let knows_person_of_age:bool = true;

    // if/else & opearators
    if age >= 18 && check_id || knows_person_of_age {
        println!("Police Officer: you are allowed to drive.");
    } else if age < 18 && check_id {
        println!("Police Officer: I need to see your ID");
    } else {
        println!("Police Officer: Sorry! your are not allowed to dirve");
    } 

    // Shorthand If
    let is_of_age = if age >= 18 { true } else { false };
    println!("id of age {}", is_of_age)

}