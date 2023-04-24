fn main() {
    //Tuples
    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.2);

    let mut modtup: (i32, bool, char) = (1, true, 's');
    println!("{}", modtup.2);
    modtup = (10, false, 'a');
    println!("{}", modtup.2);

    //arrays
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[4] = 3;
    println!("{}", arr[4])

}
