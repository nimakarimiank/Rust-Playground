pub fn entry() {
    greet_user("Nima Karimian");
    println!("{}", implicit_return());
    println!("{}", explicit_return());
    //
}

fn greet_user(users_name: &str) {
    println!("Greetings Dear {users_name}!");
}
fn implicit_return() -> &'static str {
    "nimakarimian"
}
fn explicit_return() -> &'static str {
    return "Explicit Return";
    println!("val");
}

