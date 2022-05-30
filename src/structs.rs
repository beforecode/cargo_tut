// Structds are used to create custom data types

// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Colore(u8, u8, u8);

// Struct: Preson
struct Person  {
    first_name: String,
    last_name: String
}
impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    // Get full name
   fn full_name(&self) -> String {
       format!("{} {}", self.first_name, self.last_name)
   }

   // Set last name
   fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
   }
   
   // name to Tuple
   fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
   }
}

pub fn run() {
   let mut c = Color {
       red: 255,
       green: 0,
       blue: 0,
   };
   c.red = 200;
   println!("color: {} {} {}", c.red, c.green, c.blue);

   let mut d = Colore(130, 130 ,130);
   d.1 = 50;
   println!("colore: {} {} {}", d.0, d.1, d.2);

   let mut p = Person::new("Ainkouir", "Yassine");
   println!("{} {}", p.first_name, p.last_name);
    p.set_last_name("Samir");
   println!("{}",p.full_name());
   println!("{:?}",p.to_tuple()); 
}