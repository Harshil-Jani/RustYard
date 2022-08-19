use std::io::stdin;
pub fn run(){
    // let mut name = String::new();
    // stdin().read_line(&mut name).expect("Unable to read name");
    // println!("{}",name);

    // Inputing the Numbers in Rust
    let mut n = String::new();
    stdin().read_line(&mut n).expect("Unable to read N");
    
    let n : i32 = n.trim().parse().expect("Not a Number");
    println!("{}",n);

    // Inputing a Vector in Rust
    let mut A : Vec<i32> = vec![]; 
    for i in 0..n{
        let mut n1 = String::new();
        stdin().read_line(&mut n1).expect("");
        let n1: i32 = n1.trim().parse().expect("");
        A.push(n1);
    }
    println!("{:?}",A);
    for i in A.iter(){
        println!("Element is {}",*i);
    }
}