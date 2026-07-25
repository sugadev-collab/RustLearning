enum Message {
    Quit,
    Text(String),
    Move { x: f32, y: f32 },
    Resize(u32, u32),
}

fn handle(message: &Message) {
    match message {
        Message::Quit => println!("Exiting...."),
        Message::Text(text) => println!("Your text: {}", text),
        Message::Move { x, y } => println!("Moving to position X : {}, Y: {}", x, y),
        Message::Resize(height, width) => println!("Height: {}, width: {}", height, width),
    }
}

fn find_message(id: u32) -> Option<Message> {
    match id {
        0 => Some(Message::Quit),
        1 => Some(Message::Text("Accepted".to_string())),
        2 => Some(Message::Move { x: 3_f32, y: 3_f32 }),
        3 => Some(Message::Resize(3_u32, 3_u32)),
        _ => None, // i need to know what is this _
    }
}
fn main() {
    for i in 0..5_u32 {
        match find_message(i) {
            Some(m) => handle(&m),
            None => println!("Nothing accepted."),
        };
    }
}
