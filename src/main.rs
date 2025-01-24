#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

use iced::widget::shader::Program;
use iced::widget::{column, text, Column};

impl Counter {
    pub fn view(&self) -> Column<Message> {
        // 縦に並べるやつ
        column![
            text("Hello World!"),
        ]
    }
    pub fn update(&mut self, _message: Message) {
        // ここにイベント処理とかを書く。
    }
}

fn main() -> iced::Result {
    iced::application("OGSP防災アプリ", Counter::update, Counter::view)
        .run()
}