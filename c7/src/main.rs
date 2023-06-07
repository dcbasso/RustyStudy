
fn main() {
    println!("Running...");
    test();
    let result = add_number(2, 3);
    println!("{}", result);
}

fn test() {
    println!("Calling test method!");
}

fn add_number(x: i32, y: i32) -> i32 {
    x + y 
}