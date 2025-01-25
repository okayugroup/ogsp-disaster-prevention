mod font;

use iced::widget::{button, column, text, Container};
use iced::{Fill, Settings, Size, Theme};

fn main() -> iced::Result {
    iced::application("OGSP Disaster Prevention", App::update, App::view)
        .window_size(Size::new(800.0, 600.0))
        .theme(App::theme)
        .settings(App::settings())
        .run()
}

/// アプリケーション全体 (Model)
#[derive(Default)]
struct App {
}

/// アプリケーションの操作を表すメッセージ (Update)
#[derive(Debug, Clone, Copy)]
enum Message {
    OpenLink(&'static str),
}

impl App {
    pub fn settings() -> Settings {
        Settings {
            fonts: font::load_fonts(),
            default_font: font::default_font().into(),
            ..Settings::default()
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::Light
    }

    pub fn update(&mut self, _message: Message) {
        match _message {
            Message::OpenLink(url) => {
                println!("Open link: {}", url);
                if let Err(error) = opener::open(url) {
                    eprintln!("Failed to open link: {}", error);
                }
            }
        }
    }

    pub fn view(&self) -> Container<Message> {
        Container::new(
            column!(
                text("OGSP Disaster Prevention")
                .font(font::default_font().black())
                .size(30), // ヘッダーになるところ
                column!(
                    text("Hello, world!"),
                    text("Icedはクールですね"),
                    text("赤色テキスト").color([1.0, 0.0, 0.0]),
                    text("大きなテキスト").size(20),
                    text("太字テキスト").font(font::default_font().bold()),
                    button("ボタンを押すとリンクに飛ぶ").on_press(Message::OpenLink("https://google.com"))
                )
                .width(Fill)
                .align_x(iced::Center)
                .padding(20)
            )
        ).padding(20)
    }
}