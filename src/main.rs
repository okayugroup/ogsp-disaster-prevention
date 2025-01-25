mod font;

use iced::widget::{center, column, text, Container};
use iced::{Settings, Size, Task, Theme};

fn main() -> iced::Result {

    iced::application("OGSP Disaster Prevention", App::update, App::view)
        .window_size(Size::new(800.0, 600.0))
        .theme(App::theme)
        .settings(App::settings())
        .run_with(App::new)
}

#[derive(Default)]
struct App {
}

#[derive(Debug, Clone, Copy)]
enum Message {
    FontLoaded(Result<(), iced::font::Error>),
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let task = iced::font::load(include_bytes!("../resources/fonts/ZenKakuGothicNew-Regular.ttf")).map(Message::FontLoaded);
        (Self::default(), task)
    }
    pub fn update(&mut self, _message: Message) {
        match _message {
            Message::FontLoaded(Ok(())) => {
                println!("Font loaded successfully!");
            }
            Message::FontLoaded(Err(error)) => {
                eprintln!("Failed to load font: {:?}", error);
            }
        }
    }
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
    pub fn settings() -> Settings {
        Settings {
            fonts: font::load_fonts(),
            default_font: iced::Font::with_name("Zen Kaku Gothic New"),
            ..Settings::default()
        }
    }

    pub fn view(&self) -> Container<Message> {
        center(
            column!(
                text("Hello, world!"),
                text("Icedはクールですね（アイスだけに）"),
                text("赤色テキスト").color([1.0, 0.0, 0.0]),
                text("大きなテキスト").font(
                    iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..iced::font::Font::with_name("Noto Sans JP")
                    }
                ).size(20),
            )
        )
    }
}