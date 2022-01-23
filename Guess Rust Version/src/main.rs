use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
   println!("Guess Game");
   let secret_number = rand::thread_rng().gen_range(0,1001);
   loop{
      println!("Enter your Guess : ");
      let mut guess = String::new();
      io::stdin().read_line(&mut guess).expect("Failed to read line!");
      let guess: u32 = match guess.trim().parse(){
         Ok(num) => num,
         Err(_) => {
            println!("{}","Dumb! You must have entered number. \nExiting Guess.....\nExited Successfully.".blue());
            break;
         },
      };
      println!("Your Guessed number was {}",guess);
      match guess.cmp(&secret_number){
      Ordering::Less => println!("{}","Small Number".red()),
      Ordering::Equal => {
         println!("{}","Yayy !You Won".green());
         break;
      },
      Ordering::Greater => println!("{}","Big Number".red()),
   }
   }
}
