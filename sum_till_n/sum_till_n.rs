use std::io;

fn count(num: i32){
    let mut counter = 0;
    let mut sum = 0;
    while counter <= num {
        if counter == 0 {
            println!("Started from {counter}");
        }
        println!("{counter}");
        sum += counter;
        counter += 1;
    }
    println!("Sum up to {num} is {sum}");
}
fn main(){
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line");
    let num: i32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("{} is not a valid number", input.trim());
            0
        }
    };
    count(num);
}