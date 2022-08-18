// Structs - Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct a Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self)-> String{
        return format!("{} {}",self.first_name, self.last_name);
    }

    fn set_last_name(&mut self,last: &str){
        self.last_name = last.to_string()
    }

    fn to_tuple(self) -> (String, String){
        (self.first_name,self.last_name)
    }
}
pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;
    println!("Color : {} {} {}", c.red, c.green, c.blue);
    let mut ct = ColorTuple(255, 0, 0);
    ct.0 = 200;
    println!("Color : {} {} {}", ct.0, ct.1, ct.2);

    let mut p = Person::new("Harshil","Jani");
    println!("Person : {}",p.full_name());
    p.set_last_name("Jain");
    println!("Person : {}",p.full_name());
    println!("Person : {:?}",p.to_tuple());
}
