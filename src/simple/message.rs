#[allow(dead_code)]
pub fn using_message() {
    let message_quit = Message::Quit;
    message_quit.call();
    let message_move = Message::Move { x: 3, y: 4 };
    message_move.call();
    let message_write = Message::Write(String::from("hello"));
    message_write.call();
    let message_change_color = Message::ChangeColor(1, 2, 3);
    message_change_color.call();
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
            Message::Quit => {
                println!("Message some instance called: {:?}", &self);
            }
            Message::Move {
                x: self_x,
                y: self_y,
            } => {
                println!("Message some instance called: x: {}, y: {}", self_x, self_y);
            }
            Message::Write(value) => {
                println!("Message some instance called(Write): value: {}", value);
            }
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Message some instance called(Write): r: {}, g: {}, b: {}",
                    r, g, b
                );
            }
        }
    }
}
