use iced::{Sandbox, widget::text};
fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum Message {
    
}
struct Editor;

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("The Editor")
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&self) -> iced::Element<'_, Message> {
        text("Hello, iced!").into()
    }
}