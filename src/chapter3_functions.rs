pub fn entry() {
    greet_user("Nima Karimian");
    println!("{}", implicit_return());
    println!("{}", explicit_return());
    //
}

///this is a documentation comment
fn greet_user(users_name: &str) {
    //this is a comment
    println!("Greetings Dear {users_name}!");
    /*this
        is
        a multi-line
        comment
        */
}
fn implicit_return() -> &'static str {
    "nimakarimian"
}
fn explicit_return() -> &'static str {
    return "Explicit Return";
    println!("val");
}

