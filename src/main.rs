fn main() {
    println!("Hello, world!");
let mut s  = String::from("hello world");
    let word = first_word(&s);
    s.clear();

    use_struct();

    let email = String::from(" ");
    let name = String::from(" ");
    build_user(email, name);
}

// slices

fn first_word(s: &String)->usize{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;   
        }
    }
    s.len()
}

// struct

struct User{
    active: bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

fn use_struct(){
    let user1 = User{
        active: true,
        username:String::from("someusername123"),
        email:String::from("someone@example.com"),
        sign_in_count:1,
    };

    let mut user2 = User{
        active: true,
        username: String::from("anotherusername456"),
        email:String::from("someone2@example.com"),
        sign_in_count:1,    
    };

    user2.email = String::from("anotheremail@example.com")

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}