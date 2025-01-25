use std::borrow::Cow;

pub const FONT_NAME: &str = "Zen Kaku Gothic New";

pub fn load_fonts() -> Vec<Cow<'static, [u8]>> {
    vec![
        include_bytes!("../resources/fonts/ZenKakuGothicNew-Regular.ttf").as_slice().into(),
    ]
}

pub fn default_font() -> iced::Font {
    iced::Font::with_name(FONT_NAME) // ここで指定したフォントがデフォルトフォントになる
}