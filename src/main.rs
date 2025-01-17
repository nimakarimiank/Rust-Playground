fn main() {
    let square = Rectangle::square(32);
    let square_area = square.area();
    println!("squares area is {:#?}",square_area);

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32)->Self{
        Self {
            height: size,
            width: size
        }
    }
    fn area(&self)-> u32{
        self.height*self.width
    }
    fn can_hold(&self, other: &Rectangle)-> bool{
        self.width > other.width && self.height > other.height
    }
}