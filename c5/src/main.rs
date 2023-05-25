use std::io;


fn main() {
    let x: i8 = 9;
    let y: i8 = 10;

    let z = x + y; // keep the same type i8
    println!("{}", z);


    let xx: f64 = 255.0;
    let yy: f64 = 10.0;
    let zz = xx / yy; //keep the same type f64
    println!("{}", zz); 

    let xxx: u8 = 255;
    let yyy: u8 = 10;

    let zzz = xxx / yyy; // keep the same type u8
    println!("{}", zzz);

    // List of Arithmetic:
    //  + - * / % 

    //Type declarations:
    //let one = 1.0f32;
    //let two = 2.0_f32;
    //three two = 3.0 as f32;

    //CASTING
    let a: i64 = 127_000;
    let b: i32 = 10;

    let c = a / (b as i64);
    println!("{}", c);



    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading user input");
    let int_input : i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 10);
}
