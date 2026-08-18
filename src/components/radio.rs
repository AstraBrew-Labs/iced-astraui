pub fn radio<'a, V, Message>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_select: impl FnOnce(V) -> Message,
) -> Element<'a, Message>
where
    V: Eq + Copy,
    Message: Clone + 'a,
{
    iced_radio(label, value, selected, on_select)
        .size(16)
        .spacing(8)
        .text_size(12)
        .text_line_height(iced::widget::text::LineHeight::Absolute(Pixels(20.0)))
        .font(crate::fonts::REGULAR)
        .style(radio_style)
        .into()
}

pub fn radio_style(_theme: &Theme, status: iced_radio::Status) -> iced_radio::Style {
    let (selected, hovered) = match status {
        iced_radio::Status::Active { is_selected } => (is_selected, false),
        iced_radio::Status::Hovered { is_selected } => (is_selected, true),
    };
    iced_radio::Style {
        background: Background::Color(SURFACE),
        dot_color: BLUE_600,
        border_width: 1.0,
        border_color: if selected {
            BLUE_600
        } else if hovered {
            BLUE_500
        } else {
            LINE
        },
        text_color: Some(INK),
    }
}

