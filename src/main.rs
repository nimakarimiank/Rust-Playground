mod control_flow;
use crate::control_flow::{calculate_tax, is_even, is_odd};

//noinspection SpellCheckingInspection
const HASHING_SALT: &str = "MASDNASJCKMASCOASNNOASKDM";


fn main() {
    let number = 31;
    if is_even(number)&&is_odd(number) { println!("number is both even and odd"); }
    else if is_odd(number) { println!("number is odd"); }
    else if is_even(number) { println!("number is even"); }
    else { println!("NUMBER IS PRIME"); }

    println!("{}",calculate_tax(1423f64));
}

