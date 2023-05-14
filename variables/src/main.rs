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

    println!("3 hrs: {THREE_HOURS_IN_SECONDS}");
}
