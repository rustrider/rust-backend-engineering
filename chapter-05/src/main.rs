
// progress of stream#04 is page 133
// this is the example of struct in rust
/*
struct User {
    email: String,
    password: String,
    active: bool
}
fn main () {
    //
    let user = User {
        email: String::from("rust@gmail.com"),
        password: String::from("randompass"),
        active: true,
    };
    println!("password: {}",user.password)
}

// this the struct tuple example

fn main(){
    struct Color(i32,i32,i32);
    let blackColor = Color(0,0,0); // black color
}

//this is the example of refactoring using struct in rust

// Example 1: which is a none refactored code
fn area (width: u32, height: u32) -> u32 {
     width * height
}
fn main() {
    let height: u32= 10;
    let width: u32 = 2;
    println!("the total is: {}" ,area(height,width));
}

// Example 3: which is a refactored code
fn area(dm: (u32,u32) ) -> u32{
    dm.0 * dm.1
}
fn main () {
    let dimension = (10,2);
    println!("the total is: {}" ,area(dimension));
}
// example 4: trying out debug macro with rust

#[derive(Debug)]
fn area(dm: (u32,u32) ) -> u32{
    dm.0 * dm.1
}
fn main () {
    let dimension = (10,2);
    println!("this element 1: {}", area(dimension.0, dimension.1));
}
*/


































