mod font;

use iced::widget::{center, column, text, Container};
use iced::{Settings, Size, Theme};

fn main() -> iced::Result {
    iced::application("OGSP Disaster Prevention", App::update, App::view)
        .window_size(Size::new(800.0, 600.0))
        .theme(App::theme)
        .settings(App::settings())
        .run()
}

#[derive(Default)]
struct App {
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl App {
    pub fn update(&mut self, _message: Message) {
    }
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
    pub fn settings() -> Settings {
        Settings {
            fonts: font::load_fonts(),
            default_font: font::default_font(),
            ..Settings::default()
        }
    }

    pub fn view(&self) -> Container<Message> {
        column!(
            text("Hello, world!")
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..font::default_font()
                }).size(20),
            center(
                column!(
                    text("Hello, world!"),
                    text("Icedはクールですね（アイスだけに）"),
                    text("赤色テキスト").color([1.0, 0.0, 0.0]),
                    text("大きなテキスト").size(20),
                )
            )
        ).into()
    }
}