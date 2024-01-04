// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
        Quit,
        Echo(String),
        Move,
        ChangeColor
    // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    // println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);

    let msg = Message::Echo("Way".to_string());
    match msg {
        Message::Echo(msg) => {
            println!("This is the {}", msg);
        }
        Message::Quit => {
            return
        }
        Message::Move => {
            return
        }
        Message::ChangeColor => {
            return
        }
    }
}
