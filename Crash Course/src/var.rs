// Variable hold primitive data or references to data
// Variable are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let _name = "Harshil";
    let age = 19;
    println!("{}",age);
    // age = 20; By Default Rust String is Immutable
    println!("{}",age);
    let mut new_age = 19;
    println!("{}",new_age);
    new_age = 20;
    println!("{}",new_age);

    // Define constant
    const PI: f32 = 3.14;
    println!("{}",PI);

    // Assign multiple varibales at once
    let (my_name, my_age) = ("Harshil",19);
    println!("{} is {}", my_name, my_age)
}