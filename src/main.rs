fn main() {
    // Defining and Instantiating Structs
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // change the value in the email field of a mutable User instance
    // Note that the entire instance must be mutable;
    // Rust doesn’t allow us to mark only certain fields as mutable.
    user1.email = String::from("anotheremail@example.com");

    let user1 = build_user(String::from("sdfsd"), String::from("sdfds"));

    // Creating Instances from Other Instances with Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Using Tuple Structs Without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    let subject = AlwaysEqual;
}

struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // Using the Field Init Shorthand
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
