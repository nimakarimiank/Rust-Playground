use std::io;


fn guess(){
    //Chapter 2
    println!("Enter a random number please:\t");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("*Failed to read line\t");

    //
}
fn main() {
    guess();
}
