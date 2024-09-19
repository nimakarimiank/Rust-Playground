use crate::HASHING_SALT;

pub fn chapter_three_exec() {
    let _first_name = "nima";
    let last_name = "karimian kakolaki";

    //age can change but name can't (hypothetically)
    let mut age = 27;
    println!("{}", HASHING_SALT); //const var

    let cert_tuple:(&str,i8)=("BSc of Software engineering",16);
    let (certificate_name,certificate_score) = cert_tuple;
    println!("Dear {_first_name} {last_name}!. You got you {certificate_name} \
    with average grade of {certificate_score} and you're {age} years old");

    //smart use case of shadowing -> modifying a immutable variable and make it immutable again
    let full_name = _first_name; // first variable overshadowed by the second full_name
    let full_name = _first_name.to_owned()+last_name;
    // and full_name is still immutable
    println!("your full name is {full_name}");
    //we also can use it to make a variable change it's type without casting, or declaring new var
    let full_name = full_name.len();
    println!("your full name's length is {full_name}");
    // full name is now of type usize

    //DTYPES
    //SCALAR DTYPES
    //INTEGER, FLOAT, BOOLEAN, CHAR
    let integer_number :i8 = 123; //signed
    let unsigned_integer_number:u8 = 223; //unsigned
    //
    let floating_number :f32 = 12.312; //doesn't have u or i variant
    //operations
    let truncated = -5 /3; // Upper Bracket of the result
    println!("{truncated}");
    let quotient = 56.7 / 32.2;
    println!("{quotient}");
    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");
    let is_valid = true;
    let specified_bool : bool = false;
    //CHAR
    let sticker = '👅';
    let specified_char = 'ŵ';
    let char = 'f';

}