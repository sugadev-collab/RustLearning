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
fn main() {
    handle(&Message::Quit);
    handle(&Message::Text("Lol".to_string()));
    handle(&Message::Move {
        x: (1.2_f32),
        y: (0.2_f32),
    });

    let msg = Message::Text("Hello".to_string());
    handle(&msg);
    handle(&msg); // now this is usable
    let resize = Message::Resize(800_u32, 600_u32);

    handle(&resize);
    handle(&resize);
}
