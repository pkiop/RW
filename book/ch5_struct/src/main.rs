// mod ownership_of_struct_data;
// mod example_program;
mod example_program_impl;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit like struct
#[derive(Debug)]
struct AlwaysEqual;

// tuple vs struct
// wanna named field
// or not (example RGB)
fn main() {
    example_program_impl::run();
}
// fn main() {
//     let tp = ('a', 'b', 1);

//     // rust doesn't allow to mark only certain fields as mutable
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("username"),
//         active: true,
//         sign_in_count: 3,
//     };

//     println!("{:?}", tp);
//     println!("{}", tp.0);

//     println! {"{:?}", user1.email};
//     user1.email = String::from("nextEmail@example.com");
//     println! {"{:?}", user1.email};

//     let mut user2 = build_user(
//         String::from("builduseremail@example.com"),
//         String::from("builduser"),
//     );

//     let mut user3 = User {
//         email: String::from("user3@example.com"),
//         ..user1
//     };
//     println!("user3 : {} and username : {}", user3.email, user3.username);

//     let alwaysEqual = AlwaysEqual;
//     println!("what is alwaysEqual ? : {:?}", alwaysEqual);
//     ownership_of_struct_data::run();
// }
