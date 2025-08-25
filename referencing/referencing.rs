fn main() {
    let s = String::from("Hello");
    let len = take_ref(&s);
    println!("length of string {s} is {len}");
}

fn take_ref(s :&String) -> usize {
    s.len()
}