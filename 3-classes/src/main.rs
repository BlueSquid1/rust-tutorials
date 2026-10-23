#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        return Self {
            width: width,
            height: height,
        };
    }
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);

    // Print out all values in a struct that has #[derive(Debug)]
    println!("rect1={rect1:#?}");

    // Can print results from functions like this:
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
