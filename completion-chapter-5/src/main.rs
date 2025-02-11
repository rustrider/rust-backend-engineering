// this example is gonna be related to using impl instead of using normal function
struct Rectangle{
    width: u32,
    height: u32
}
impl Rectangle{
    // GETTER NUMBER 1
    fn area(&self) -> u32{
        self.width * self.height
    }
    // GETTER NUMBER 2
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}
fn main () {
    let init_struct = Rectangle {
        width: 20,
        height: 2
    };
    let rect2 = Rectangle {
        width: 23,
        height: 10
    };
    println!("the area result is: {}", init_struct.can_hold(&rect2))
}