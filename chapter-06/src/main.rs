/*
enum Options{
    None,
    Some(i32)
}
enum Options<T = i32>{
    None,
    Some(T)
}
fn main() {
    let absent_number: Options<T> = None;
    println!("Let see if this is working {}", absent_number);
}
*/
// giving example of IDE showing errors when using wrong match expersion

// this is a good example of using  option module + enum + generic types
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
fn main() {
    // Pattern match to retrieve the value
    match divide(2.0, 3.0) {
        // The division was valid
        Some(x) => println!("Result: {x}"),
        // The division was invalid
        _ => println!("something else")
    }
    // to create a syntax sugar for the above match statement you can just do something like this
    if let
        Some(x) = divide(2.0, 3.0){
        println!("Result: {x}")
    }
}
/*
// this is js code // this is called syntax sugar for if else
max > 20 ? "ok" : "not ok" // similar to if let statement
if { // similar to match statement
}
else{
}
 */
