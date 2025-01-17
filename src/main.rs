fn main() {
    let width:u32 = 30;
    let height:u32 = 50;
    println!("The area of rectangle is {} square pixels",area(width, height));
}

    fn area(height:u32, width:u32) -> u32 {
        width*height
    }