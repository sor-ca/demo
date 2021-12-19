#[derive(Debug)]
struct Rectangle {
    width: u32,
    hight: u32,
    }
impl Rectangle {
    fn area(&self) -> u32 {
        self.width*self.hight
        }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width>other.width && self.hight>other.hight
        }
    fn square(size: u32)-> Rectangle {
        Rectangle { width: size, hight: size,}
        }
    }   
fn main() {
let rect1 = Rectangle {width: 30,hight: 50,};
let rect2 = Rectangle {width: 10,hight: 40,};
let rect3 = Rectangle {width: 60,hight: 45,};
println!("This is {:?}1", rect1);
println!
    ("The area of rectangle1 is {}", rect1.area());
println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
let sqr3 = Rectangle::square(3);
println!("This is {:?}1", sqr3);
}

