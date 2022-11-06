struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("carlo@example.com"),
        username: String::from("carlo"),
        active: true,
        sign_in_count: 1,
    };

    println!("User 1: {}", user1.email);
    println!("active: {}", user1.active);
    println!("sign_in_count: {}", user1.sign_in_count);
    println!("email: {}", user1.email);
    println!("username: {}", user1.username);
}
