use std::io;
use std::io::Write;

fn main() {
    
    let a = [1, 2, 3, 4, 5];

    print!("Please enter an array index.\n>> ");
    io::stdout().flush().expect("Error flushing stdout.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
