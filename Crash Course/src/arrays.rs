// Arrays - Fixed list where elements are the same datatype
pub fn run(){
    let mut numbers : [i32;5] = [1,2,3,4,5];
    println!("{:?}",numbers);
    println!("{}",numbers[0]);

    // Re-assign
    numbers[2] = 20;
    println!("{:?}",numbers);

    // Get array len
    println!("{}",numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}",slice);
}