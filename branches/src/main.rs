fn main() {
    let number = 3;

    let result = if number < 5 {
                      println!("condition was true");
                      true
                  } else {
                      println!("condition was false");
                      false
                  };
    

    let result2 = number < 5;

    println!("The value of result is: {}", result);
    println!("The value of result2 is: {}", result2);


    {
        let number = 6;

        if number % 4 == 0 {
            println!("{} is divisible by 4", number);
        } else if number % 3 == 0 {
            println!("{} is divisible by 3", number);
        } else if number % 2 == 0 {
            println!("{} is divisible by 2", number);
        } else {
            println!("{} is not divisible by 4, 3, or 2", number);
        }
    };
}
