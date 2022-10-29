use std::io;

struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut input = String::from("Hello");
    print_string(&input);
    let rectangle = Rectangle {
        height: 10,
        width: 20,
    };
    println!("The rectangle's area is {}", rectangle.area());
}

fn calculate_area(r: &Rectangle) -> u32 {
    r.width * r.height
}

fn print_string(s: &String) {
    println!("{}", s);
}