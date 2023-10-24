fn main() {
    println!("Ownership in Rust");

    {                      // s is not valid here, it's not yet declared
        println!("Ownership 1: declare variable in a scope");
        let s = "hello";   // s is valid from this point forward
        println!("\tNow s is declared and initialied with {} and we can use it", s);
    }                      // this scope is now over, and s is no longer valid
    
    {
        println!("Ownership 2: declare variable in a scope and mutate it with push_str()");
        let mut s = String::from("Hello");

        s.push_str(", world!"); // push_str() appends a literal to a String
        
        println!("\t{}", s); // This will print 'hello, world!'
    }

    {
        println!("Ownership 3: Heap Data: Copy - types with unknown sizes at compile time");
        let s1 = String::from("String move");
        let s2 = s1; // This invalidates s1 (variable is "moved")

        println!("\ts2: {} (s1 is no longer valid)", s2);
    }

    {
       println!("Ownership 4: Stack-Only Data: Copy - types with known sizes at compile time");
        let x = 5;
        let y = x;

        println!("\tx: {}, y: {}", x, y);
    }

    {
        println!("Ownership 5: basics of ownership - String and integer");

        let s = String::from("hello"); // s comes into scope
        
        takes_ownership(s);           // s's value moves into the function...
                                      // ... and so is no longer valid here
        
        let x = 5;                    // x comes into scope
         

        makes_copy(x);                // x would move into the function,
                                      // but i32 is Copy, so it's okay to still
                                      // use x afterward.
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    {
        let s1 = gives_ownership();        // gives ownerhip moves its return
                                           // value into s1

        let s2 = String::from("hello");    // s2 comes into scope
        
        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
        println!("\ts1: {}, s3: {}", s1, s3);
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    {
        println!("Ownership 6: return multiple values with tuple");
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("\tThe length of '{}' is {}.", s2, len);
    }
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn gives_ownership() -> String {
    String::from("bye")
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("\t{} - dropped after scope ends", some_string);
} // Here, some_string_goes out of scope and `drop` is called. the backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("\t{} - not dropped as it lives on stack and copy is made in fn", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

