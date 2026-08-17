mod app;
mod fonts;
mod icons;
pub mod ui;

use iced::{Size, window};
use lucide_icons::LUCIDE_FONT_BYTES;

fn main() -> iced::Result {
    let mut application = iced::application(
        app::Launcher::new,
        app::Launcher::update,
        app::Launcher::view,
    )
    .title(app::Launcher::title)
    .subscription(app::Launcher::subscription)
    .theme(app::Launcher::theme)
    .font(LUCIDE_FONT_BYTES);

    for (_, bytes) in fonts::FONT_MAPPINGS {
        application = application.font(bytes);
    }

    application
        .default_font(fonts::REGULAR)
        .window(window::Settings {
            size: Size::new(1180.0, 760.0),
            min_size: Some(Size::new(940.0, 640.0)),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        .antialiasing(true)
        .run()
}
