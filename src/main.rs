fn main() {
    let mut name = String::from("nima karimian,");
    add_age(&mut name);
    println!("{name}");
}


fn add_age(s: &mut String) {
    s.push_str(" 27 years old");
}



