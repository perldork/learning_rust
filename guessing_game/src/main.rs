use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Enter a number: ");
        io::stdout().flush().expect("Failed to flush STDOUT");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "panic!" {
            panic!("Panicking!");
        }

        if guess.trim() == "q" || guess.trim() == "quit" {
            println!("Bye!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(your_guess) => your_guess,
            Err(_) => {
                println!("'{}' is not a number, please try again", guess.trim());
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { println!("You win!"); break; }
        }
    }
}
