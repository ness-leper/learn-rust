fn main() {
    println!("Hello, world!");

    another_function(6);

    print_labeled_measurement(5, 'h');
    println!("The value of x is: {}", five());
    println!("The value of 6 + 1 is: {}",plus_one(6));
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
