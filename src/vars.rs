pub fn run() {
	//mutable data
	let name = "Mohammed";
	let age = 23;
	
	println!("my name is {} & my age is {}", name, age);

	//const keyword
	const ID: i32 = 001; //ID variable is a 32 bit type of data
	println!("The id is {}", ID);

	//defining multiple varibles
	let (pc_name, pc_price) = ("DELL", 150);
	println!("The {} cost {}", pc_name, pc_price);
}