/*
Primitive Types :
Integers : u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats : f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/
pub fn run(){

    // Find max size
    println!("{}",std::i32::MAX);
    println!("{}",std::f32::MAX);

    // Boolean
    let is_active = true;
    // println!("{:?}", ("Harshil",19,is_active));
    
    // Character
    let a1 = 'H';
     println!("{:?}", ("Harshil",19,is_active,a1));

    // Unicodes
    let face = '\u{1F600}';
    println!("{}",face);

}