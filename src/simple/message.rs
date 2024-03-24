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
        // method body would be defined here
        // メソッド本体はここに定義される
        println!("Message some instance called: {:?}", &self);
    }
}
