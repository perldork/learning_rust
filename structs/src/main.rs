
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    structs_exercises_1();
    structs_exercises_2();
    // structs_exercises_3_this_will_not_compile();
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysBlue;

// This will not compile because no lifetimes specified
// for the string slices
// fn structs_exercises_3_this_will_not_compile() {
//     struct InvalidUser {
//         active: bool,
//         username: &str,
//         email: &str,
//         sign_in_count: u64,
//     }
// 
//     let user = InvalidUser {
//         active: true,
//         username: "notgonnaworkhere",
//         email: "notgonnaworkhere@example.com",
//         sign_in_count: 1,
//     }
// }

fn structs_exercises_2() {

    println!("Structs exercises: 2 - Tuple structs");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("\tblack: {:?}", black);
    println!("\torigin: {:?}", origin);

    let silicon_valley_game = AlwaysBlue;
    println!("\tsilicon_valley_game: {:?}", silicon_valley_game);
}

fn structs_exercises_1() {

    println!("Structs exercises: 1 - named structs");

    let user1 = User {
        active: true,
        username: String::from("foobar_blatz"),
        email: String::from("user@example.com"),
        sign_in_count: 1,
    };

    println!("\tuser1: {:?}", user1);
    
    let user2 = build_user(String::from("user2@example.com"), 
                           String::from("user2".to_string()));
    println!("\tuser2: {:?}", user2);

    // ..user2 means use the rest of the fields from user2
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user2
    };
    println!("\tuser3: {:?}", user3);
    println!("\tuser2 is now: {:?}", user2);
        
}

fn build_user(email: String, username: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 0,
    }
}
