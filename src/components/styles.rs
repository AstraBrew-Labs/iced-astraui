pub fn canvas(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(CANVAS)),
        text_color: Some(INK),
        ..container::Style::default()
    }
}

pub fn sidebar(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        text_color: Some(INK),
        ..container::Style::default()
    }
}

pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            radius: RADIUS_PANEL.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

fn card_variant_style(variant: CardVariant) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let (background, shadow) = match variant {
            CardVariant::Transparent => (None, Shadow::default()),
            CardVariant::Default => (
                Some(Background::Color(SURFACE)),
                Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.07),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 10.0,
                },
            ),
            CardVariant::Secondary => (Some(Background::Color(SURFACE_ALT)), Shadow::default()),
            CardVariant::Tertiary => (
                Some(Background::Color(Color::from_rgb8(226, 240, 253))),
                Shadow::default(),
            ),
        };

        container::Style {
            background,
            border: Border {
                radius: RADIUS_PANEL.into(),
                ..Border::default()
            },
            shadow,
            text_color: Some(INK),
            ..container::Style::default()
        }
    }
}

pub fn flat_card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.937, 0.937, 0.941))),
        border: Border {
            radius: RADIUS_INNER.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

pub fn code_block(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(INK)),
        border: Border {
            radius: RADIUS_FIELD.into(),
            ..Border::default()
        },
        text_color: Some(CYAN_300),
        ..container::Style::default()
    }
}

pub fn tint(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE_ALT)),
        border: Border {
            radius: RADIUS_FIELD.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

/// Outer frame for Figma fields. `text_input::Style` has no shadow field, so
/// the elevation belongs to a zero-padding container around the control.
pub fn field_frame(_theme: &Theme) -> container::Style {
    container::Style {
        border: Border {
            radius: RADIUS_FIELD.into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
        ..container::Style::default()
    }
}

pub fn nav_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    nav_button_animated(active, 0.0)
}

pub fn nav_button_animated(
    active: bool,
    press_progress: f32,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let press_mix = press_progress.clamp(0.0, 1.0);
        button::Style {
            background: if active || hovered {
                Some(Background::Color(if active {
                    SURFACE_ALT
                } else {
                    Color::from_rgb8(245, 245, 245)
                }))
            } else {
                None
            },
            text_color: if active { INK } else { INK_MUTED },
            border: Border {
                radius: RADIUS_INNER.into(),
                ..Border::default()
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.04 * press_mix),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            ..button::Style::default()
        }
    }
}

pub fn text_input_style(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let focused = matches!(status, text_input::Status::Focused { .. });
    let hovered = matches!(
        status,
        text_input::Status::Hovered | text_input::Status::Focused { is_hovered: true }
    );
    let disabled = matches!(status, text_input::Status::Disabled);
    text_input::Style {
        background: Background::Color(if disabled {
            Color::from_rgba(SURFACE_ALT.r, SURFACE_ALT.g, SURFACE_ALT.b, 0.5)
        } else {
            SURFACE
        }),
        border: Border {
            color: if disabled {
                Color::from_rgba(LINE.r, LINE.g, LINE.b, 0.55)
            } else if focused {
                BLUE_600
            } else if hovered {
                BLUE_500
            } else {
                LINE
            },
            width: if focused { 2.0 } else { 1.0 },
            radius: RADIUS_FIELD.into(),
        },
        icon: BLUE_600,
        placeholder: INK_SUBTLE,
        value: INK,
        selection: Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.20),
    }
}

pub fn pick_list_style(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let hovered = matches!(status, pick_list::Status::Hovered);
    let opened = matches!(status, pick_list::Status::Opened { .. });
    pick_list::Style {
        text_color: INK,
        placeholder_color: INK_SUBTLE,
        handle_color: if opened { BLUE_600 } else { INK_MUTED },
        background: Background::Color(SURFACE),
        border: Border {
            color: if opened {
                BLUE_600
            } else if hovered {
                BLUE_500
            } else {
                LINE
            },
            width: if opened { 2.0 } else { 1.0 },
            radius: RADIUS_FIELD.into(),
        },
    }
}

pub fn pick_list_menu_style(_theme: &Theme) -> menu::Style {
    menu::Style {
        background: Background::Color(SURFACE),
        border: Border {
            color: LINE,
            width: 1.0,
            radius: RADIUS_FIELD.into(),
        },
        text_color: INK,
        selected_text_color: BLUE_700,
        selected_background: Background::Color(Color::from_rgba(
            BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.10,
        )),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 20.0,
        },
    }
}

pub fn pick_list_handle() -> pick_list::Handle<Font> {
    let icon = |glyph: LucideIcon| pick_list::Icon {
        font: Font::with_name("lucide"),
        code_point: glyph.into(),
        size: Some(Pixels(16.0)),
        line_height: iced::widget::text::LineHeight::Absolute(Pixels(16.0)),
        shaping: iced::widget::text::Shaping::Basic,
    };

    pick_list::Handle::Dynamic {
        closed: icon(LucideIcon::ChevronDown),
        open: icon(LucideIcon::ChevronUp),
    }
}

pub fn tag_style(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(Color::from_rgba(
            color.r, color.g, color.b, 0.15,
        ))),
        border: Border {
            radius: RADIUS_FIELD.into(),
            ..Border::default()
        },
        text_color: Some(color),
        ..container::Style::default()
    }
}

fn readable_on(color: Color) -> Color {
    fn linear(channel: f32) -> f32 {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }

    fn luminance(color: Color) -> f32 {
        0.2126 * linear(color.r) + 0.7152 * linear(color.g) + 0.0722 * linear(color.b)
    }

    fn contrast(first: Color, second: Color) -> f32 {
        let (lighter, darker) = {
            let first = luminance(first);
            let second = luminance(second);
            if first > second {
                (first, second)
            } else {
                (second, first)
            }
        };
        (lighter + 0.05) / (darker + 0.05)
    }

    if contrast(color, INK) >= contrast(color, WHITE) {
        INK
    } else {
        WHITE
    }
}

fn chip_style(color: Color, variant: ChipVariant) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let (background, text_color) = match variant {
            ChipVariant::Flat => (
                Some(Color::from_rgba(color.r, color.g, color.b, 0.14)),
                color,
            ),
            ChipVariant::Solid => (Some(color), readable_on(color)),
            ChipVariant::Outline => (None, color),
        };

        container::Style {
            background: background.map(Background::Color),
            border: Border {
                radius: 999.0.into(),
                ..Border::default()
            },
            text_color: Some(text_color),
            ..container::Style::default()
        }
    }
}

