use iced::{Color, Element};
use lucide_icons::Icon;

pub fn icon<'a, Message: 'a>(glyph: Icon, size: u32, color: Color) -> Element<'a, Message> {
    let icon: iced::widget::Text<'a> = glyph.into();

    icon.size(size).color(color).into()
}
