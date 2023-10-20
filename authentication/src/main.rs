mod authentication;

fn main() {
    let mut user = authentication::User::new("jondoe", "very secure");
    println!("The username is: {}", user.get_username());
    println!("The password hash is: {}", user.get_password_hash());

    user.set_password("even more secure");
    println!("The new password hash is: {}", user.get_password_hash());
}
