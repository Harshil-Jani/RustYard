pub fn run(){
    let age = 19;
    let check_crimes = true;
    if age>=21 && !check_crimes {
        println!("President !");
    }else if age<21 && !check_crimes{
        println!("Student !");
    }else {
        println!("Go to Jail ! ");
    }
}