// Primitive Str = Immutable fixed Length string somewhere in memory
// String = Growable, heap-allocated data structure = Use when you need to modify or own string data

pub fn run(){

    let hello = "Hello";
    println!("{}",hello);

    let mut world = String::from("World ");
    //println!("{}",world);

    // Get length
    println!("length of hello : {}\nlength of world : {}",hello.len(),world.len());

    // push() only pushes a charatect 
    // push_str() pushes the string
    world.push('H');
    world.push_str("arshil");
    println!("{}",world);

    // Capacity in bytes
    // Error --> println!("Capactiy : {}, {}",hello.capacity(),world.capacity())
    println!("Capactiy : {}",world.capacity());

    // IsEmpty
    println!("Is Empty ? {}",world.is_empty());

    // Contains
    println!("Contains Harshil ? : {}",world.contains("Harshil"));

    // Replace
    println!("{}",world.replace("Harshil", "Bunny"));

    // Loop through string by whitespace
    for word in world.split_whitespace(){
        println!("{}",word);
    }

    // Assertion Equal
    assert_eq!("hello","hello") // --> Passes
    //assert_eq!("hello","hello") // --> Fails
}