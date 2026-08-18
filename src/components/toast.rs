fn toast_variant_style(variant: ToastVariant) -> (Color, LucideIcon) {
    match variant {
        ToastVariant::Default => (INK_MUTED, LucideIcon::Info),
        ToastVariant::Accent => (BLUE_600, LucideIcon::Sparkles),
        ToastVariant::Success => (SUCCESS, LucideIcon::CircleCheck),
        ToastVariant::Warning => (WARNING, LucideIcon::TriangleAlert),
        ToastVariant::Danger => (DANGER, LucideIcon::CircleX),
    }
}

fn toast_surface(_accent: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            radius: RADIUS_INNER.into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.16),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 26.0,
        },
        ..container::Style::default()
    }
}

/// An interactive, temporary notification displayed by a toast region.
pub fn toast<'a, Message>(
    title: &'a str,
    description: &'a str,
    variant: ToastVariant,
    action: Option<(&'a str, Message)>,
    on_close: Message,
    on_interact: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    toast_animated(
        title,
        description,
        variant,
        action,
        on_close,
        on_interact,
        1.0,
    )
}

/// A progress-driven toast with a short slide transition.
pub fn toast_animated<'a, Message>(
    title: &'a str,
    description: &'a str,
    variant: ToastVariant,
    action: Option<(&'a str, Message)>,
    on_close: Message,
    on_interact: Message,
    animation_progress: f32,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    toast_animated_with_placement(
        title,
        description,
        variant,
        action,
        on_close,
        on_interact,
        ToastPlacement::default(),
        animation_progress,
        false,
    )
}

/// A progress-driven toast whose travel direction follows its placement.
#[allow(clippy::too_many_arguments)]
pub fn toast_animated_with_placement<'a, Message>(
    title: &'a str,
    description: &'a str,
    variant: ToastVariant,
    action: Option<(&'a str, Message)>,
    on_close: Message,
    on_interact: Message,
    placement: ToastPlacement,
    animation_progress: f32,
    closing: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let (accent, glyph) = toast_variant_style(variant);
    let mut trailing = row![].spacing(4).align_y(iced::Alignment::Center);

    if let Some((label, on_action)) = action {
        trailing = trailing.push(
            button(
                container(
                    text(label)
                        .size(11)
                        .font(crate::fonts::MEDIUM)
                        .line_height(iced::widget::text::LineHeight::Absolute(Pixels(16.0))),
                )
                .height(Fill)
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center),
            )
            .on_press(on_action)
            .height(30)
            .padding([0, 10])
            .style(button_style(ButtonVariant::Secondary)),
        );
    }

    trailing = trailing.push(
        button(centered_button_icon(LucideIcon::X, 14, INK_MUTED))
            .on_press(on_close)
            .width(30)
            .height(30)
            .padding(0)
            .style(button_style(ButtonVariant::Ghost)),
    );

    let content = mouse_area(
        container(
            row![
                container(crate::icons::icon(glyph, 17, accent))
                    .width(28)
                    .height(28)
                    .align_x(iced::Alignment::Center)
                    .align_y(iced::Alignment::Center),
                iced::widget::column![
                    text(title).size(13).font(crate::fonts::MEDIUM).color(
                        if matches!(variant, ToastVariant::Default) {
                            INK
                        } else {
                            accent
                        }
                    ),
                    text(description)
                        .size(11)
                        .font(crate::fonts::REGULAR)
                        .color(INK_MUTED),
                ]
                .spacing(2)
                .width(Fill),
                trailing,
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center),
        )
        .width(420)
        .padding([11, 13])
        .style(toast_surface(accent)),
    )
    .on_press(on_interact);
    translated(
        content,
        placement.transition_offset(animation_progress, closing),
    )
}

/// Positions a stack of toasts at one of six viewport edges.
pub fn toast_region<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    placement: ToastPlacement,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let (horizontal, vertical) = placement.alignment();
    container(content)
        .width(Fill)
        .height(Fill)
        .align_x(horizontal)
        .align_y(vertical)
        .padding(24)
        .into()
}

fn modal_backdrop(progress: f32) -> impl Fn(&Theme, button::Status) -> button::Style {
    let alpha = 0.38 * progress.clamp(0.0, 1.0);
    move |_theme, _status| button::Style {
        background: Some(Background::Color(Color::from_rgba(
            INK.r, INK.g, INK.b, alpha,
        ))),
        border: Border::default(),
        shadow: Shadow::default(),
        ..button::Style::default()
    }
}

fn modal_surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            radius: RADIUS_PANEL.into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.20),
            offset: Vector::new(0.0, 16.0),
            blur_radius: 40.0,
        },
        ..container::Style::default()
    }
}
