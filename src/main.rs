#[derive(Default)]
struct App {
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

use iced::widget::{column, text, Column};
use iced::{Settings, Size, Theme};

impl App {
    pub fn view(&self) -> Column<Message> {
        // 縦に並べるやつ
        column![
            text("Hello World!"),
        ]
    }
    pub fn update(&mut self, _message: Message) {
        // ここにイベント処理とかを書く。
    }
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
    pub fn settings() -> Settings {
        let fonts = vec![
            include_bytes!("../resources/fonts/NotoSansJP-Regular.ttf").as_slice().into(),
            include_bytes!("../resources/fonts/NotoSansJP-Bold.ttf").as_slice().into(),
        ];
        Settings {
            fonts,
            ..Settings::default()
        }
    }
}

fn main() -> iced::Result {
    iced::application("OGSP Disaster Prevention", App::update, App::view)
        .window_size(Size::new(800.0, 600.0))
        .theme(App::theme)
        .settings(App::settings())
        .run()
}