fn main() {
    //constants
    const SECONDS_IN_MINUTES: u32 = 3000;

    println!("SECONDS_IN_MINUTES: {} ", SECONDS_IN_MINUTES);

    //static and strong typed
    let x = 4; //implicity type
    println!("x is: {}", x);

    let mut y = 5;
    println!("y is: {}", y);
    y = 10;
    println!("y is: {}", y);

    let x = 11;
    println!("x is: {}", x);

    //scope: naming shadowing
    {
        let x = 2;
        println!("x is: {}", x);
    }
    {
        let x = x + 12;
        println!("x is: {}", x);
    }

    let x = x + 1;
    println!("x is: {}", x);

    let x = "hello";
    println!("x is: {}", x);
}
