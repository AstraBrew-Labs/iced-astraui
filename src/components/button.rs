pub fn button_style(variant: ButtonVariant) -> impl Fn(&Theme, button::Status) -> button::Style {
    button_style_animated(variant, 0.0)
}

pub fn button_style_animated(
    variant: ButtonVariant,
    press_progress: f32,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        let disabled = matches!(status, button::Status::Disabled);
        let interactive = hovered || pressed;
        let press_mix = press_progress.max(if pressed { 0.75 } else { 0.0 });
        let outlined = matches!(variant, ButtonVariant::Outline);
        let default_hover = Color::from_rgb8(225, 225, 226);
        let on_surface_hover = Color::from_rgb8(239, 239, 240);
        let (background, text_color) = match variant {
            ButtonVariant::Primary => (Some(if interactive { BLUE_500 } else { BLUE_600 }), WHITE),
            ButtonVariant::Secondary => (
                Some(if interactive {
                    default_hover
                } else {
                    SURFACE_ALT
                }),
                BLUE_700,
            ),
            ButtonVariant::Tertiary => (
                Some(if interactive {
                    default_hover
                } else {
                    SURFACE_ALT
                }),
                INK,
            ),
            ButtonVariant::Ghost => (
                if interactive {
                    Some(on_surface_hover)
                } else {
                    None
                },
                INK,
            ),
            ButtonVariant::Destructive => (
                Some(if interactive {
                    Color::from_rgb8(255, 85, 81)
                } else {
                    DANGER
                }),
                WHITE,
            ),
            ButtonVariant::DangerSoft => (
                Some(if interactive {
                    Color::from_rgba(DANGER.r, DANGER.g, DANGER.b, 0.20)
                } else {
                    Color::from_rgba(DANGER.r, DANGER.g, DANGER.b, 0.15)
                }),
                Color::from_rgb8(164, 53, 50),
            ),
            ButtonVariant::Outline => (None, INK),
        };

        button::Style {
            background: background.map(|color| {
                let color = mix_color(
                    color,
                    Color::from_rgba(0.0, 0.0, 0.0, color.a),
                    0.08 * press_mix,
                );
                Background::Color(if disabled {
                    Color::from_rgba(color.r, color.g, color.b, color.a * 0.5)
                } else {
                    color
                })
            }),
            text_color: if disabled {
                Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.5)
            } else {
                text_color
            },
            border: Border {
                color: if !outlined {
                    Color::TRANSPARENT
                } else if disabled {
                    Color::from_rgba(LINE.r, LINE.g, LINE.b, 0.55)
                } else if press_mix > 0.0 {
                    BLUE_600
                } else if hovered {
                    BLUE_500
                } else {
                    LINE
                },
                width: if outlined { 1.0 } else { 0.0 },
                radius: RADIUS_CONTROL.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.08 * press_mix),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0 + 3.0 * press_mix,
            },
            ..button::Style::default()
        }
    }
}

