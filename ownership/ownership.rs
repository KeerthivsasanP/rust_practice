fn main() {
    let x=5;
    let y=x;
    println!("y = {}, x = {}",y,x); 
    // y and x are assigned in the stack and a copy is made

    let s = String::from("Hello");
    let s2=s;
    //s is dropped and s2 is now the new s. This is because s was created in the heap
    println!("{}",s2);

    let s1 = give_ownership(s2);
    println!("{}",s1);

}

fn give_ownership(s :String) -> String{
    s
}