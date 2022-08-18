// Tuples group together values of different types
// Max 12 elements

pub fn run(){
    let person: (&str, &str, i8) = ("Harshil", "Open Source", 19);
    println!("{} started doing {} at the age of {}",person.0, person.1, person.2)
}