use crate::HASHING_SALT;
use std::io;
pub fn chapter_three_exec() {
    let _first_name = "nima";
    let last_name = "karimian kakolaki";

    //age can change but name can't (hypothetically)
    let mut age = 27;
    println!("{}", HASHING_SALT); //const var

    //smart use case of shadowing -> modifying a immutable variable and make it immutable again
    let full_name = _first_name; // first variable overshadowed by the second full_name
    let full_name = _first_name.to_owned() + last_name;
    // and full_name is still immutable
    println!("your full name is {full_name}");
    //we also can use it to make a variable change it's type without casting, or declaring new var
    let full_name = full_name.len();
    println!("your full name's length is {full_name}");
    // full name is now of type usize

    //DTYPES
    //SCALAR DTYPES
    //INTEGER, FLOAT, BOOLEAN, CHAR
    let integer_number: i8 = 123; //signed
    let unsigned_integer_number: u8 = 223; //unsigned
    //
    let floating_number: f32 = 12.312; //doesn't have u or i variant
    //operations
    let truncated = -5 / 3; // Upper Bracket of the result
    println!("{truncated}");
    let quotient = 56.7 / 32.2;
    println!("{quotient}");
    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");
    let is_valid = true;
    let specified_bool: bool = false;
    //CHAR
    let sticker = '👅';
    let specified_char = 'ŵ';
    let char = 'f';
    //COMPOUND DTYPE
    let mut tup: (i8, &str, f32);
    tup = (23, "hi", 12.12);
    println!("{:#?}", tup);
    tup = (27, "bye", 121.12);
    println!("{:#?}", tup);
    //pattern matching
    let cert_tuple: (&str, i8) = ("BSc of Software engineering", 16);
    let (certificate_name, certificate_score) = cert_tuple;
    println!("Dear {_first_name} {last_name}!. You got you {certificate_name} \
    with average grade of {certificate_score} and you're {age} years old");
    //access elements by index
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    //ARRAYS
    //Arrays are useful when you want your data allocated on the stack rather than the heap TODO R&D
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    //You write an array’s type using square brackets with the type of each element,
    // a semicolon, and then the number of elements in the array ⬇️
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // initialize an array to contain the same value x times in this case [3,3,3,3,3] ⬇️
    let a = [3; 5];
    //test index out of range
    out_of_range_index();

}
fn out_of_range_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}