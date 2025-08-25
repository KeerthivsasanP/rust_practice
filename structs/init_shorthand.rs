struct User {
    name :String,
    email :String,
    active :bool,
    sign_in_count :usize,
}

fn main() {
    let user1 = User{
        name : String::from("name1"),
        email : String::from("name1@email.com"),
        active : true,
        sign_in_count : 1,
    };

    let user2 = User{
        name : String::from("name2"),
        email : String::from("name2@gmail.com"),
        ..user1
    };

    println!("{} {} {} {}", user2.name, user2.email, user2.active, user2.sign_in_count);
}
