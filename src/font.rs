use std::borrow::Cow;

/// フォントをロードします。
pub fn load_fonts() -> Vec<Cow<'static, [u8]>> {
    vec![
        include_bytes!("../resources/fonts/ZenKakuGothicNew-Regular.ttf").as_slice().into(),
        include_bytes!("../resources/fonts/ZenKakuGothicNew-Bold.ttf").as_slice().into(),
        include_bytes!("../resources/fonts/ZenKakuGothicNew-Black.ttf").as_slice().into(),
    ]
}

/// Zen角ゴシック Newのフォントを提供します。
pub fn zen_kaku_gothic_new() -> Font {
    Font::new("Zen Kaku Gothic New")
}

/// デフォルトのフォント
pub fn default_font() -> Font {
    // デフォルトフォント、略してデフォント(?)
    // デフォルトでは角ゴを使う
    zen_kaku_gothic_new()
}

/// フォントを表す構造体
/// フォント名とフォントの情報を持ちます。
pub struct Font {
    font: iced::Font,
}

impl Font {
    fn new(name: &'static str) -> Self {
        Self {
            font: iced::Font::with_name(name),
        }
    }

    /// 太さを太めにします。
    pub fn bold(&mut self) -> &mut Self {
        self.font.weight = iced::font::Weight::Bold;
        self
    }

    /// 太さを極太にします。
    pub fn black(&mut self) -> &mut Self {
        self.font.weight = iced::font::Weight::Black;
        self
    }

    /// 太さを通常に戻します。
    pub fn regular(&mut self) -> &mut Self {
        self.font.weight = iced::font::Weight::Normal;
        self
    }
}

impl From<Font> for iced::Font {
    fn from(font: Font) -> Self {
        font.font
    }
}

impl From<&mut Font> for iced::Font {
    fn from(font: &mut Font) -> Self {
        font.font
    }
}