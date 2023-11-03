fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let l1 = String::from("woohoo");

    let l2 = change_it(&l1);

    println!("l1 is {}, l2 is {}", l1, l2);

    let mut s3 = String::from("hallo");
    println!("s3 is {}", s3);

    change(&mut s3);
    println!("s3 is {}", s3);

    let reference_to_nothing = dangle();
    println!("reference_to_nothing is {}", reference_to_nothing);

    slice_exercises();

}

fn slice_exercises() {

    println!("Slices:");
    let slice_string = String::from("'Which' is the first word?");
    let the_first_word = first_word(&slice_string);

    println!("\tthe_first_word is {}", the_first_word);

    let slice1 = String::from("frogs rocks");

    let hello = &slice1[..5];
    let world = &slice1[6..];
    let hello_world = &slice1[..];
    println!("\thello is {}, world is {}", hello, world);
    println!("\thello_world is {}", hello_world);

    let my_string = String::from("slice exercise 3");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[..6]);
    let _word = first_word(&my_string[..]);

    // `first_word` also works on references to `String`s, which are 
    // equivalent to whole slices of `String`s
    let word = first_word(&my_string);
    println!("\tword is {}", word);

    let my_string_literal = "This is a string literal";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[..4]);
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("\tword is {}", word);

    let a = [1, 2, 3, 4, 5];
    println!("\ta is {:?}", a);

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    println!("\tslice (&a[1..3]) is {:?}", slice);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_it(a_string: &String) -> &String {
    println!("a_string is {}", a_string);
    a_string
}

fn change(some_string: &mut String) {
    some_string.push_str(", varld");
}

fn dangle() -> String {
    let s = String::from("nothing is something");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // enumerate returns a tuple
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
