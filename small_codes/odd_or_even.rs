use std::io;

fn main() {
    println!("Enter a number");
    let mut num: String = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let num: isize = num.trim().parse().expect("Enter a valid number");
    if num % 2 == 0{
        println!("{num} is even");
    } else {
        println!("{num} is odd");
    }
}
