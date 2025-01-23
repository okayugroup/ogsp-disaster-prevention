#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

use iced::widget::{column, text, Column};

impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            text("Hello World!"),
        ]
    }
}

impl Counter {
    pub fn update(&mut self, _message: Message) {
    }
}

fn main() -> iced::Result {
    iced::run("My app", Counter::update, Counter::view)
}