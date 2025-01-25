use std::borrow::Cow;

pub const DEFAULT_FONT: &[u8] = include_bytes!("../resources/fonts/NotoSansJP-Regular.ttf").as_slice();

pub fn load_fonts() -> Vec<Cow<'static, [u8]>> {
    vec![
        DEFAULT_FONT.into(),
        include_bytes!("../resources/fonts/NotoSansJP-Bold.ttf").as_slice().into(),
    ]
}