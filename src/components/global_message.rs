fn message_kind_style(kind: MessageKind) -> (Color, LucideIcon) {
    match kind {
        MessageKind::Info => (BLUE_600, LucideIcon::Info),
        MessageKind::Success => (SUCCESS, LucideIcon::CircleCheck),
        MessageKind::Warning => (WARNING, LucideIcon::TriangleAlert),
        MessageKind::Danger => (DANGER, LucideIcon::CircleX),
    }
}

fn global_message_surface(_accent: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
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

fn centered_button_icon<'a, Message: 'a>(
    glyph: LucideIcon,
    size: u32,
    color: Color,
) -> Element<'a, Message> {
    container(crate::icons::icon(glyph, size, color))
        .width(Fill)
        .height(Fill)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .into()
}

/// A compact global message suitable for action feedback and status updates.
pub fn global_message<'a, Message>(
    title: &'a str,
    description: &'a str,
    kind: MessageKind,
    on_close: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    global_message_animated(title, description, kind, on_close, 1.0)
}

/// A progress-driven global message with a short slide transition.
pub fn global_message_animated<'a, Message>(
    title: &'a str,
    description: &'a str,
    kind: MessageKind,
    on_close: Message,
    animation_progress: f32,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    global_message_animated_with_phase(
        title,
        description,
        kind,
        on_close,
        animation_progress,
        false,
    )
}

/// A progress-driven global message that retraces its entry direction while closing.
#[allow(clippy::too_many_arguments)]
pub fn global_message_animated_with_phase<'a, Message>(
    title: &'a str,
    description: &'a str,
    kind: MessageKind,
    on_close: Message,
    animation_progress: f32,
    closing: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    global_message_animated_with_placement(
        title,
        description,
        kind,
        on_close,
        ToastPlacement::default(),
        animation_progress,
        closing,
    )
}

/// A progress-driven global message whose travel direction follows its placement.
#[allow(clippy::too_many_arguments)]
pub fn global_message_animated_with_placement<'a, Message>(
    title: &'a str,
    description: &'a str,
    kind: MessageKind,
    on_close: Message,
    placement: ToastPlacement,
    animation_progress: f32,
    closing: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let (accent, glyph) = message_kind_style(kind);
    let content = container(
        row![
            container(crate::icons::icon(glyph, 18, accent))
                .width(36)
                .height(36)
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center)
                .style(tag_style(accent)),
            iced::widget::column![
                text(title).size(13).font(crate::fonts::BOLD).color(INK),
                text(description)
                    .size(11)
                    .font(crate::fonts::REGULAR)
                    .color(INK_MUTED)
            ]
            .spacing(3)
            .width(Fill),
            button(centered_button_icon(LucideIcon::X, 15, INK_MUTED))
                .on_press(on_close)
                .width(32)
                .height(32)
                .padding(0)
                .style(button_style(ButtonVariant::Ghost)),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
    )
    .width(360)
    .padding([12, 14])
    .style(global_message_surface(accent));
    translated(
        content,
        placement.transition_offset(animation_progress, closing),
    )
}
