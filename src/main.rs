fn main() {
    let rect1 = Rectangle {
        height: 32,
        width: 42,
    };
    //here rec1 is moved to area and is not valid from now on.
    let area = area(rect1);
    //rec1 is not valid since we are borrowing it here and it has been moved in ln 7
    println!("{:#?}",rect1);
    println!("The area of rectangle is {} square pixels", area);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rectangle: Rectangle) -> u32 {
    rectangle.height * rectangle.width
}