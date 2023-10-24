use io::Write;
use std::io;

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("counter = {}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    multiple_loop();
    while_loop();
    for_loop();
    iterator_loop();
    range_loop();
    convert_far_to_cel(0.0);
    convert_far_to_cel(32.0);
    convert_far_to_cel(-40.0);
    convert_far_to_cel(5000.0);
    generate_fibonacci(184);
}

fn multiple_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn while_loop() {
    let mut number = 3;

    print!("BEGIN COUNTDOWN: ");
    io::stdout().flush().expect("Error flushing stdout.");

    while number != 0 {
        print!("{} ... ", number);
        io::stdout().flush().expect("Error flushing stdout.");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// Faster than while with index because no runtime checks
// needed to determine if the index is within the bounds of the array
fn iterator_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}

fn range_loop() {
    println!("Countdown:");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn convert_far_to_cel(farenheight: f32) -> f32 {
    let celcius = (farenheight - 32.0) * (5.0 / 9.0);
    println!("{} degrees F = {} degrees C", farenheight, celcius);
    celcius
}

fn generate_fibonacci(n: u32) {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128;

    for digit in 0..(n + 1) {
        c = a + b;
        a = b;
        b = c;
        if a > 0 && a != b {
            println!("{}: {}", digit, a);
        }
    }
} 
