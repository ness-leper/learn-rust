use std::io;

fn main() {
    let mut x:f64 = 5.05;
    println!("The value of x is: {x}");
    x = 6.0;
    println!("The value of x is: {x}");

    let t: bool = true;
    println!("The value of t is: {t}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x,y,_z) = tup;
    println!("The value of y is {y}");
    println!("The third value of the tuple is {}",tup.2);

    println!("3 hrs: {THREE_HOURS_IN_SECONDS}");

    let a: [i32;5] = [1,2,3,4,5];
    println!("The array has {}", a[1]);

    println!("Enter an array index:");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at {index} is: {element}");
}
