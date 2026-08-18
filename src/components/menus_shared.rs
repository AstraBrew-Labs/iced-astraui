pub struct MenuItem<'a, Message> {
    label: &'a str,
    icon: Option<LucideIcon>,
    on_press: Message,
    danger: bool,
}

impl<'a, Message> MenuItem<'a, Message> {
    pub fn new(label: &'a str, icon: Option<LucideIcon>, on_press: Message) -> Self {
        Self {
            label,
            icon,
            on_press,
            danger: false,
        }
    }

    pub fn danger(label: &'a str, icon: Option<LucideIcon>, on_press: Message) -> Self {
        Self {
            label,
            icon,
            on_press,
            danger: true,
        }
    }
}

fn popup_surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            radius: RADIUS_FIELD.into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.14),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        ..container::Style::default()
    }
}

fn menu_item_style(danger: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        let foreground = if danger { DANGER } else { INK };
        let background = if pressed {
            Some(Color::from_rgba(
                foreground.r,
                foreground.g,
                foreground.b,
                0.14,
            ))
        } else if hovered {
            Some(Color::from_rgba(
                foreground.r,
                foreground.g,
                foreground.b,
                0.08,
            ))
        } else {
            None
        };

        button::Style {
            background: background.map(Background::Color),
            text_color: foreground,
            border: Border {
                radius: 8.0.into(),
                ..Border::default()
            },
            ..button::Style::default()
        }
    }
}

fn menu_panel<'a, Message>(items: Vec<MenuItem<'a, Message>>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let items = items
        .into_iter()
        .fold(iced::widget::column![].spacing(2).width(Fill), |menu, item| {
            let foreground = if item.danger { DANGER } else { INK };
            let mut content = row![].spacing(9).align_y(iced::Alignment::Center);
            if let Some(icon) = item.icon {
                content = content.push(crate::icons::icon(icon, 15, foreground));
            }
            content = content.push(
                text(item.label)
                    .size(12)
                    .font(crate::fonts::REGULAR)
                    .color(foreground),
            );

            menu.push(
                button(content)
                    .on_press(item.on_press)
                    .width(Fill)
                    .height(36)
                    .padding([8, 10])
                    .style(menu_item_style(item.danger)),
            )
        });

    container(items)
        .width(208)
        .padding(6)
        .style(popup_surface)
        .into()
}

fn dropdown_button_style(expanded: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        button::Style {
            background: Some(Background::Color(if pressed {
                Color::from_rgb8(218, 218, 220)
            } else if hovered || expanded {
                Color::from_rgb8(228, 228, 230)
            } else {
                SURFACE_ALT
            })),
            text_color: INK,
            border: Border {
                radius: RADIUS_CONTROL.into(),
                ..Border::default()
            },
            ..button::Style::default()
        }
    }
}

