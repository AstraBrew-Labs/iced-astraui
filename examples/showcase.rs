#[path = "showcase/app.rs"]
mod app;

use astra_ui::fonts;
use iced::{Size, window};
use lucide_icons::LUCIDE_FONT_BYTES;

const APP_ICON: &[u8] = include_bytes!("../assets/icon/icon.png");

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
            icon: Some(app_icon()),
            ..window::Settings::default()
        })
        .antialiasing(true)
        .run()
}

fn app_icon() -> window::Icon {
    let rgba = image::load_from_memory(APP_ICON)
        .expect("embedded app icon must be a valid image")
        .into_rgba8();
    let (width, height) = rgba.dimensions();

    window::icon::from_rgba(rgba.into_raw(), width, height)
        .expect("embedded app icon dimensions must match its pixels")
}
