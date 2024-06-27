use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game: Bulls and Cows");
    let secret_number = rand::thread_rng().gen_range(1..11);
    let mut attempts = 0;

    loop {
    println!("Please input a number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Oops! Something went wrong");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please give a Valid input");
            continue;
        }
    };
    attempts += 1;
    
    if guess < 1 || guess > 10 {
        println!("Please enter a valid number between 1 to 10");
        continue;
    }
    
    match guess.cmp(&secret_number){
        Ordering::Less => { 
            println!("too small");
            if attempts > 5{
                println!("You have tried {} times", attempts);
            }
        }
        Ordering::Greater => {
            println!("Greater than the lucky number");
            if attempts > 5 {
                println!("You have tried {attempts}");
            }
        }
        Ordering::Equal => {
            println!("Congratulations! You have guessed the lucky number in {} attempts", attempts);
            break;
        }
}
if attempts == 10 {
    println!("Sorry, you have reached the maximum attempts. The lucky number was {}", secret_number);
    break;
}
}
}