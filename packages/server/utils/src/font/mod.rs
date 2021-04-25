use rusttype::Font;

static FONT_DATA: &[u8] = include_bytes!("Inter-SemiBold.otf");

pub fn get_default_font() -> Font<'static> {
    Font::try_from_bytes(FONT_DATA).expect("Couldn't load font.")
}
