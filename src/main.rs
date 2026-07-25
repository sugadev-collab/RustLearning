enum Message {
    Quit,
    Text(String),
    Move { x: f32, y: f32 }, // ok can you explain ehat is the type on Move? is it a struct?
}

fn handle(message: Message) {
    match message {
        Message::Quit => println!("Exiting...."),
        Message::Text(text) => println!("Your text: {}", text),
        Message::Move { x, y } => println!("Moving to position X : {}, Y: {}", x, y),
    }
}
fn main() {
    handle(Message::Quit); // simple varient
    handle(Message::Text("Lol".to_string())); //parameterized method varient
    handle(Message::Move {
        x: (1.2_f32),
        y: (0.2_f32),
    }); // struct varient   
    // i dont mainly know about enum varients
}
