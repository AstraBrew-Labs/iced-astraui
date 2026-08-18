/// A compact label for status, category, or lightweight metadata.
pub fn chip<'a, Message>(
    label: &'a str,
    icon: Option<LucideIcon>,
    color: Color,
    variant: ChipVariant,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let foreground = if matches!(variant, ChipVariant::Solid) {
        readable_on(color)
    } else {
        color
    };
    let label = text(label)
        .size(11)
        .font(crate::fonts::MEDIUM)
        .line_height(iced::widget::text::LineHeight::Absolute(Pixels(16.0)))
        .color(foreground);
    let content = if let Some(icon) = icon {
        row![crate::icons::icon(icon, 12, foreground), label]
            .spacing(5)
            .align_y(iced::Alignment::Center)
    } else {
        row![label].align_y(iced::Alignment::Center)
    };

    container(content)
        .height(24)
        .padding([2, 8])
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .style(chip_style(color, variant))
        .into()
}

fn badge_surface(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(color)),
        border: Border {
            radius: 999.0.into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 3.0,
        },
        text_color: Some(WHITE),
        ..container::Style::default()
    }
}

fn badge_offset(position: BadgePosition, extent: f32) -> Vector {
    let horizontal = match position {
        BadgePosition::TopRight | BadgePosition::BottomRight => extent / 2.0,
        BadgePosition::TopLeft | BadgePosition::BottomLeft => -extent / 2.0,
    };
    let vertical = match position {
        BadgePosition::TopRight | BadgePosition::TopLeft => -extent / 2.0,
        BadgePosition::BottomRight | BadgePosition::BottomLeft => extent / 2.0,
    };

    Vector::new(horizontal, vertical)
}

