// Vectors are resizable arrays
pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    numbers.push(6);
    numbers.push(10);
    numbers.pop();
    println!("{:?}",numbers);

    // Loop Through Vector Values
    for x in numbers.iter(){
        println!("Number : {}", x);
    } 

    // Loop and Mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("{:?}",numbers);
}