use iced::widget::{button, container};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

pub const INK: Color = Color::from_rgb(0.075, 0.086, 0.094);
pub const INK_SOFT: Color = Color::from_rgb(0.34, 0.37, 0.38);
pub const SIDEBAR: Color = Color::from_rgb(0.055, 0.067, 0.071);
pub const CANVAS: Color = Color::from_rgb(0.955, 0.961, 0.953);
pub const SURFACE: Color = Color::from_rgb(0.995, 0.995, 0.985);
pub const LINE: Color = Color::from_rgb(0.83, 0.84, 0.81);
pub const COPPER: Color = Color::from_rgb(0.73, 0.31, 0.15);
pub const COPPER_DARK: Color = Color::from_rgb(0.58, 0.22, 0.10);
pub const MINT: Color = Color::from_rgb(0.18, 0.58, 0.47);
pub const WHITE: Color = Color::WHITE;
pub const WHITE_MUTED: Color = Color::from_rgb(0.66, 0.70, 0.69);

pub fn app_theme() -> Theme {
    Theme::custom(
        "AstraBrew".to_owned(),
        iced::theme::Palette {
            background: CANVAS,
            text: INK,
            primary: COPPER,
            success: MINT,
            warning: Color::from_rgb(0.87, 0.58, 0.15),
            danger: Color::from_rgb(0.74, 0.18, 0.17),
        },
    )
}

pub fn canvas(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(CANVAS)),
        text_color: Some(INK),
        ..container::Style::default()
    }
}

pub fn sidebar(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SIDEBAR)),
        text_color: Some(WHITE),
        ..container::Style::default()
    }
}

pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: LINE,
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.04, 0.05, 0.05, 0.08),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 12.0,
        },
        ..container::Style::default()
    }
}

pub fn tint(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.91, 0.92, 0.89))),
        border: Border {
            radius: 5.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

pub fn nav_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let background = if active {
            Some(Background::Color(Color::from_rgb(0.12, 0.14, 0.14)))
        } else if matches!(status, button::Status::Hovered) {
            Some(Background::Color(Color::from_rgb(0.09, 0.11, 0.11)))
        } else {
            None
        };

        button::Style {
            background,
            text_color: if active { WHITE } else { WHITE_MUTED },
            border: Border {
                radius: 5.0.into(),
                ..Border::default()
            },
            ..button::Style::default()
        }
    }
}

pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => COPPER_DARK,
        button::Status::Pressed => Color::from_rgb(0.47, 0.16, 0.07),
        button::Status::Disabled => Color::from_rgb(0.63, 0.57, 0.53),
        button::Status::Active => COPPER,
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: WHITE,
        border: Border {
            radius: 5.0.into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.3, 0.08, 0.02, 0.16),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..button::Style::default()
    }
}

pub fn ghost_button(_theme: &Theme, status: button::Status) -> button::Style {
    let background = matches!(status, button::Status::Hovered | button::Status::Pressed)
        .then_some(Background::Color(Color::from_rgb(0.89, 0.90, 0.87)));

    button::Style {
        background,
        text_color: INK,
        border: Border {
            color: LINE,
            width: 1.0,
            radius: 5.0.into(),
        },
        ..button::Style::default()
    }
}
