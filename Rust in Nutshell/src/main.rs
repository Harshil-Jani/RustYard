fn main() {
    
    //Shadowing Variables. Re-defining the same variable with either same or different datatype.

    let x = 12;
    println!("The value of x is {}",x);
    let x = 13;
    println!("The value of x is {}",x);

    //Constant declaration. It will save you from shadowing in case you never want your const value to get shadowed.

    const LOVE : u32  = 14;
    println!("The love constant in my life is {}",LOVE);

    //  Scaler DataTypes
    //1. Integers
    //2. Floating-point numbers
    //3. Booleans
    //4. Character

    let a: u8 = 25_5; // 8 Bit Unsigned integers, Rng: [0,255] beyond it the code will panic
    println!("{}",a); // In Rust we can write digits with _ in between for more clearity 100000 ==> 100_000
    let b: f32 = 23.456;
    println!("{}",b);
    let c: bool = true;
    println!("{}",c);
    let d: char = 'H';
    println!("{}",d);

    //  Compound DataTypes
    //1. Tuples
    //2. Arrays

    let tup = ("Harshil Jani","SVNIT",28,14);
    println!("{} {}",tup.0,tup.3);
    let array = ["Turkey", "17th Feb", "Amour"];
    println!("{} is Happy Birthday of my {}",array[1],array[2]);
    
    //functions
    let sum = addition(17,28);
    println!("The sum of given numbers is : {}",sum);

    //conditional statements - It must always have a boolean comparision in he conditons
    let h = 14;
    if h<10{
        println!("The Smaller Number");
    }else if h==14 {
        println!("The Perfect Number");
    }else{
        println!("The Bigger Number");
    }
    let flag = false;
    let num = if flag {5} else {7};
    println!("{}",num); 

    //loops
    let mut  counter = 0;
    let final_count = loop {
        counter += 1;
        if counter==10{
            break counter; // It returns counter
        }
    };
    println!("{}",final_count);
    while counter>0 {
        counter -= 1;
    }
    println!("{}",counter);

    let b_array = [6,7,14,17,28];
    for something in b_array.iter(){
        println!("{}",something);
    }

    for number in 1..11 {
        print!("{} ",number);
    }
    println!("");
    // Single Line Comment
    /* Block Comment*/
    
}

fn addition(x: u32, y: u32) -> u32{
    println!("The x and y are equal to {} and {}",x,y);
    return x+y;
}
