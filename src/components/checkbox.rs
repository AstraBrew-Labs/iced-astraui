pub fn checkbox_style(_theme: &Theme, status: checkbox::Status) -> checkbox::Style {
    let target = match status {
        checkbox::Status::Active { is_checked }
        | checkbox::Status::Hovered { is_checked }
        | checkbox::Status::Disabled { is_checked } => is_checked,
    };
    checkbox_style_with_progress(status, if target { 1.0 } else { 0.0 })
}

pub fn checkbox_style_animated(
    transition_progress: f32,
) -> impl Fn(&Theme, checkbox::Status) -> checkbox::Style {
    move |_theme, status| checkbox_style_with_progress(status, transition_progress)
}

fn checkbox_style_with_progress(
    status: checkbox::Status,
    transition_progress: f32,
) -> checkbox::Style {
    let (hovered, disabled) = match status {
        checkbox::Status::Active { .. } => (false, false),
        checkbox::Status::Hovered { .. } => (true, false),
        checkbox::Status::Disabled { .. } => (false, true),
    };
    let idle_border = if hovered { BLUE_500 } else { LINE };
    let border_color = if transition_progress <= 0.0 {
        idle_border
    } else if transition_progress >= 1.0 {
        BLUE_600
    } else {
        mix_color(idle_border, BLUE_600, transition_progress)
    };
    checkbox::Style {
        background: Background::Color(mix_color(SURFACE, BLUE_600, transition_progress)),
        icon_color: mix_color(INK, WHITE, transition_progress),
        border: Border {
            color: if disabled {
                Color::from_rgba(border_color.r, border_color.g, border_color.b, 0.55)
            } else {
                border_color
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: Some(if disabled { INK_SUBTLE } else { INK }),
    }
}

