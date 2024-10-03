mod chapter3_functions;
mod control_flow;

use crate::chapter3_functions::entry;
use crate::control_flow::{is_even, is_odd};

//noinspection SpellCheckingInspection
const HASHING_SALT: &str = "MASDNASJCKMASCOASNNOASKDM";


fn main() {
    let number = 12;
    if is_odd(number) { println!("number is odd"); }
    else if is_even(number) { println!("number is even"); }
    else if is_even(number)&&is_odd(number) { println!("number is both even and odd"); }
}

