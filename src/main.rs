use crate::get_first_word::get_first_word;

mod get_first_word;
fn main() {
    let mut string_var = String::from("nimakarimian");
    string_var.push_str(" ,Ms.c Ingenieurinformatik");
    let index = get_first_word(&string_var);
    println!("{index}")
}



