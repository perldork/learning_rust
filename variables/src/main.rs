fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let mut x: i32 = 6;
    println!("The value of x is {:^5}", x);
    x = 6;
    println!("The value of x is {}", x);

    let y = 6;

    println!("Outer: the value of y is {}", y);

    {
        let y = y * 2;
        println!("Inner 1: the value of y is now {}", y);

        {
            let y = y * 3;
            println!("Inner 2: the value of y is now {}", y);
        }

        println!("Inner 1: the value of y is now back to {}", y);
    }

    println!("Outer: the value of y is now back to {}", y);
}

