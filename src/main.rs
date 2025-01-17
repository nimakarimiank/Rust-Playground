fn main() {
    let rect1 = (32u32,45u32);
    println!("The area of rectangle is {} square pixels",area(rect1));
}

    fn area(dimensions:(u32,u32)) -> u32 {
        dimensions.0*dimensions.1
    }