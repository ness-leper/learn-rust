fn main() {
    let condition = false;
    let number = if condition { 6 } else { 12 };

    if number % 4 == 0 {
        show_divide(4);
    } else if number % 3 == 0 {
        show_divide(3);
    } else if number % 2 == 0 {
        show_divide(2);
    } else {
        println!("number is not divisible by 4, 3, 2");
    }
}

fn show_divide(res: i32) {
    println!("number is divisible by {}", res);
}
