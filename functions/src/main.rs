fn main() {
    println!("Hello, world!");
    println!("{}", another_function(5));
}

fn another_function(x: i32) -> bool {
    println!("Another function. The value of x is: {x}");
    true
}
