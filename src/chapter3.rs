use crate::HASHING_SALT;

pub fn chapter_three_exec() {
    let _first_name = "nima";
    let last_name = "karimian kakolaki";

    //age can change but name can't (hypothetically)
    let mut age = 27;
    println!("{}", HASHING_SALT); //const var

    let cert_tuple:(&str,i8)=("BSc of Software engineering",16);
    let (certificate_name,certificate_score) = cert_tuple;
    println!("You got you {certificate_name} with average grade of {certificate_score}");

}