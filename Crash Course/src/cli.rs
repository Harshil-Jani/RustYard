use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    println!("Args {:?}",args);
    let command = args[1].clone();
    println!("{}",command);

    if(command=="hello"){
        println!("Captured the Flag !");
    }
}