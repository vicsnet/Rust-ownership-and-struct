fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();

    use_struct();

    let email = String::from(" ");
    let name = String::from(" ");
    build_user(email, name);

    calculate_the_area_rectangle();
    method_synthax();
}

// slices

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// struct

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn use_struct() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("anotherusername456"),
        email: String::from("someone2@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}   
// Example of program using structs

fn calculate_the_area_rectangle() {
    let width1 = 30;
    let height1 = 50;



    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);
    println!("The area2 of the rectangle is {} square pixels.", area2(rect1));
    let rect_struct = Rectangle {
        width: 30,
        height: 50,
    };

    println!(" the struct area is {} ", area_struct(&rect_struct));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// refractoring with tuples
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// refractoring with struct

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// method syntax
#[derive(Debug)]
struct Rect{
    width: u32,
    height: u32,  
}

impl Rect{
    fn area(&self)->u32{
        self.width * self.height

    }
}

fn method_synthax(){
    let rect1 = Rect{
        width: 30,
        height: 50,
    };

    println!(" the area with method syntax is {} square pixels ", rect1.area());
}