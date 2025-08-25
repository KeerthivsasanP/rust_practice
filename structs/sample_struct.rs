struct User {
    name :String,
    email :String,
    active :bool,
    sign_in_count :usize
}

fn main () {
    let user1 = User {
        name : String::from("name1"),
        email : String::from("name1@email.com"),
        active : true,
        sign_in_count : 1,
    };
    println!("{} {} {} {}", user1.name, user1.email, user1.active, user1.sign_in_count);
}
