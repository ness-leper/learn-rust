fn main() {
    println!("72F is {}", f_to_c(72));
    println!("22C is {}", c_to_f(22));
}

fn f_to_c(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

fn c_to_f(c: i32) -> i32 {
    c * 9 / 5 + 32 
}
