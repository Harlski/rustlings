// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    Move {x: i32, y:i32},
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit
    // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }

}

fn main() {
    let the_message = "hello world";
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from(the_message)), // Just tested around with String::from to understand it a bit better
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
