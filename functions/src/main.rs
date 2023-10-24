fn main() {
    println!("Hello, world!");

    another_function(3, 'd');
    another_function(300_000_000, '$');
    another_function(-13, 'm');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    println!("The value of five() is: {}", five());

    println!("The value of plus_one(5) is: {}", plus_one(5));
}

fn another_function(x: i32, unit_label: char) {
    println!("the value of x is: {}{}", x, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
