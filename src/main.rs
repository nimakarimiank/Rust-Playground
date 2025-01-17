fn main() {
    let rect1 = Rectangle {
        height: 32,
        width: 42,
    };
    //now we didn't move rec1 and just passed a reference to the method
    let area = area(&rect1);
    // since we didnt move the rect1 to the area method we can use it since it is not out of scope
    println!("{:#?}",rect1);

    println!("The area of rectangle is {} square pixels", area);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}