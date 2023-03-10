use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("kazu ate");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("secret_number is {}", secret_number);
    println!("kazu irete");

    loop{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed");
    let guess:u32 = match guess.trim().parse() {
        Ok(num)=>num,
        Err(_)=>continue,
    }; 
    
    println!("you guessed:{}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("small"),
        Ordering::Greater => println!("big"),
        Ordering::Equal => {
            println!("win");
            break;
            }
        }
    }
}
