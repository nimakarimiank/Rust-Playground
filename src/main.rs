fn main() {
    let admin = Users {
        is_active: true,
        email: String::from("niimak97@gmail.com"),
        password: String::from("MUNINTHINKNUMJUICE"),
        user_name: String::from("Admin"),
        full_name: String::from("nima karimian"),
    };
    println!("{}",admin.email);
}


struct Users {
    is_active: bool,
    user_name: String,
    email: String,
    password: String,
    full_name: String,
}