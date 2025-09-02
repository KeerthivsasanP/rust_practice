use std::io;

fn main() {
    println!("Enter the user name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Name is not read");
    println!("Hello {name}");
}
