
fn main() {
    {
        let st = "nima karimian"; // fixed size string
        let mut heap_string = String::from(st);
        heap_string.push_str(" kakolaki");
        println!("{heap_string}");
    }
    println!("{heap_string}");
}



