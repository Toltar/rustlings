// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
    Echo(String),
    Move(String),
    ChangeColor(String),
}

fn main() {
    println!("{:?}", Message::Quit(String::from("Do you want to quit?")));
    println!("{:?}", Message::Echo(String::from("ECHO! ECHO!")));
    println!("{:?}", Message::Move(String::from("Moving")));
    println!("{:?}", Message::ChangeColor(String::from("Changing color")));
}
