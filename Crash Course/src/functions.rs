pub fn run(){
    greetings("Morning", "Harshil")
}

fn greetings(greet: &str, name: &str){
    println!("Good {}, {}",greet,name);

    // Bind function values to varibales
    let get_sum = add(1,19);
    println!("Sum : {}",get_sum);

    // Closure
    let z = 4;
    let add_nums = |x: i32, y : i32 | x+y+z;
    println!("Closure Sum : {}",add_nums(3,3));
}

fn add (x: i32, y: i32) -> i32 {
    return x + y;
}