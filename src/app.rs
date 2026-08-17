use iced::widget::{button, column, container, pick_list, row, rule, scrollable, space, text};
use iced::{Alignment, Element, Fill, Task, Theme};
use lucide_icons::Icon;

use crate::{fonts, icons, theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Page {
    #[default]
    Home,
    Library,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Channel {
    #[default]
    Stable,
    Preview,
    Nightly,
}

impl Channel {
    const ALL: [Self; 3] = [Self::Stable, Self::Preview, Self::Nightly];
}

impl std::fmt::Display for Channel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Stable => "Stable  /  1.0.0",
            Self::Preview => "Preview  /  1.1.0-rc.2",
            Self::Nightly => "Nightly  /  2026.08.17",
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Navigate(Page),
    ChannelSelected(Channel),
    ToggleLaunch,
    CheckUpdates,
}

#[derive(Debug, Default)]
pub struct Launcher {
    page: Page,
    channel: Channel,
    running: bool,
    checking_updates: bool,
}

impl Launcher {
    pub fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn title(&self) -> String {
        "AstraBrew Launcher".to_owned()
    }

    pub fn theme(&self) -> Theme {
        theme::app_theme()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => self.page = page,
            Message::ChannelSelected(channel) => self.channel = channel,
            Message::ToggleLaunch => self.running = !self.running,
            Message::CheckUpdates => self.checking_updates = !self.checking_updates,
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        row![self.sidebar(), self.content()]
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn sidebar(&self) -> Element<'_, Message> {
        let brand = row![
            container(icons::icon(Icon::Coffee, 22, theme::WHITE))
                .width(36)
                .height(36)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(theme::COPPER)),
                    border: iced::Border {
                        radius: 5.0.into(),
                        ..iced::Border::default()
                    },
                    ..container::Style::default()
                }),
            column![
                text("ASTRABREW").size(16).font(fonts::BLACK),
                text("LAUNCHER")
                    .size(10)
                    .font(fonts::LIGHT)
                    .color(theme::WHITE_MUTED),
            ]
            .spacing(1)
        ]
        .align_y(Alignment::Center)
        .spacing(10);

        let navigation = column![
            self.nav_button(Page::Home, Icon::LayoutDashboard, "Overview"),
            self.nav_button(Page::Library, Icon::Library, "Library"),
            self.nav_button(Page::Settings, Icon::Settings2, "Settings"),
        ]
        .spacing(5);

        let footer = column![
            text("SYSTEM")
                .size(10)
                .font(fonts::LIGHT)
                .color(theme::WHITE_MUTED),
            row![
                icons::icon(Icon::Circle, 8, theme::MINT),
                text("Services operational")
                    .size(12)
                    .color(theme::WHITE_MUTED),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text("macOS / arm64")
                .size(11)
                .font(fonts::LIGHT)
                .color(theme::WHITE_MUTED),
        ]
        .spacing(10);

        container(
            column![
                brand,
                space::vertical(),
                navigation,
                space::vertical(),
                footer
            ]
            .padding([28, 18])
            .height(Fill),
        )
        .width(228)
        .height(Fill)
        .style(theme::sidebar)
        .into()
    }

    fn nav_button(&self, page: Page, glyph: Icon, label: &'static str) -> Element<'_, Message> {
        let active = self.page == page;
        let color = if active {
            theme::WHITE
        } else {
            theme::WHITE_MUTED
        };

        button(
            row![
                icons::icon(glyph, 17, color),
                text(label).size(13).font(fonts::MEDIUM)
            ]
            .spacing(11)
            .align_y(Alignment::Center),
        )
        .on_press(Message::Navigate(page))
        .padding([11, 12])
        .width(Fill)
        .style(theme::nav_button(active))
        .into()
    }

    fn content(&self) -> Element<'_, Message> {
        let body = match self.page {
            Page::Home => self.home(),
            Page::Library => self.library(),
            Page::Settings => self.settings(),
        };

        container(scrollable(body).width(Fill).height(Fill))
            .width(Fill)
            .height(Fill)
            .style(theme::canvas)
            .into()
    }

    fn home(&self) -> Element<'_, Message> {
        let heading = row![
            column![
                text("Good afternoon")
                    .size(13)
                    .font(fonts::LIGHT)
                    .color(theme::INK_SOFT),
                text("Ready when you are.").size(31).font(fonts::BOLD),
            ]
            .spacing(5),
            space::horizontal(),
            button(
                row![
                    icons::icon(Icon::RefreshCw, 15, theme::INK),
                    text(if self.checking_updates {
                        "Up to date"
                    } else {
                        "Check for updates"
                    })
                    .size(12),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(Message::CheckUpdates)
            .padding([10, 13])
            .style(theme::ghost_button),
        ]
        .align_y(Alignment::Center);

        let hero = container(
            column![
                row![
                    container(icons::icon(Icon::Rocket, 20, theme::COPPER))
                        .width(42)
                        .height(42)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(theme::tint),
                    column![
                        text("PRIMARY INSTALLATION")
                            .size(10)
                            .font(fonts::LIGHT)
                            .color(theme::INK_SOFT),
                        text("AstraBrew Desktop").size(22).font(fonts::BOLD),
                    ]
                    .spacing(3),
                    space::horizontal(),
                    self.status_badge(),
                ]
                .spacing(13)
                .align_y(Alignment::Center),
                rule::horizontal(1),
                row![
                    self.metric(Icon::Package, "BUILD", self.channel.to_string()),
                    self.metric(Icon::HardDrive, "INSTALL SIZE", "184 MB".to_owned()),
                    self.metric(Icon::Monitor, "RUNTIME", "Native Apple silicon".to_owned()),
                ]
                .spacing(34),
                row![
                    column![
                        text("Release channel").size(11).color(theme::INK_SOFT),
                        pick_list(Channel::ALL, Some(self.channel), Message::ChannelSelected)
                            .width(245),
                    ]
                    .spacing(7),
                    space::horizontal(),
                    button(
                        row![
                            icons::icon(
                                if self.running {
                                    Icon::Square
                                } else {
                                    Icon::Play
                                },
                                17,
                                theme::WHITE,
                            ),
                            text(if self.running {
                                "Stop AstraBrew"
                            } else {
                                "Launch AstraBrew"
                            })
                            .size(13)
                            .font(fonts::MEDIUM),
                        ]
                        .spacing(9)
                        .align_y(Alignment::Center),
                    )
                    .on_press(Message::ToggleLaunch)
                    .padding([13, 18])
                    .style(theme::primary_button),
                ]
                .align_y(Alignment::End),
            ]
            .spacing(24),
        )
        .padding(24)
        .width(Fill)
        .style(theme::card);

        let recent = column![
            row![
                text("Recent activity").size(16).font(fonts::BOLD),
                space::horizontal(),
                text("TODAY")
                    .size(10)
                    .font(fonts::LIGHT)
                    .color(theme::INK_SOFT),
            ],
            rule::horizontal(1),
            self.activity_row(
                Icon::CircleCheck,
                "Runtime verified",
                "All required components passed integrity checks.",
                "13:42",
            ),
            rule::horizontal(1),
            self.activity_row(
                Icon::Download,
                "Manifest refreshed",
                "Stable channel metadata is current.",
                "12:18",
            ),
        ]
        .spacing(14);

        column![heading, hero, recent]
            .spacing(30)
            .padding([34, 42])
            .width(Fill)
            .into()
    }

    fn library(&self) -> Element<'_, Message> {
        column![
            text("Library").size(31).font(fonts::BOLD),
            text("Installed builds and optional components.")
                .size(13)
                .color(theme::INK_SOFT),
            rule::horizontal(1),
            self.library_row("AstraBrew Desktop", "1.0.0", "Ready", Icon::PackageCheck),
            self.library_row("Java Runtime", "21.0.4", "Managed", Icon::Coffee),
            self.library_row("Assets bundle", "2026.08", "Current", Icon::Archive),
        ]
        .spacing(18)
        .padding([34, 42])
        .width(Fill)
        .into()
    }

    fn settings(&self) -> Element<'_, Message> {
        column![
            text("Settings").size(31).font(fonts::BOLD),
            text("Launcher preferences for this Mac.")
                .size(13)
                .color(theme::INK_SOFT),
            rule::horizontal(1),
            self.setting_row(
                Icon::FolderOpen,
                "Installation folder",
                "~/Library/Application Support/AstraBrew",
                "Managed automatically",
            ),
            self.setting_row(
                Icon::Bell,
                "Update notifications",
                "Notify when a new stable build is available",
                "Enabled",
            ),
            self.setting_row(
                Icon::Gauge,
                "Hardware acceleration",
                "Use the native Metal renderer when available",
                "Enabled",
            ),
        ]
        .spacing(18)
        .padding([34, 42])
        .width(Fill)
        .into()
    }

    fn status_badge(&self) -> Element<'_, Message> {
        let (label, color) = if self.running {
            ("RUNNING", theme::MINT)
        } else {
            ("READY", theme::INK_SOFT)
        };

        container(
            row![
                icons::icon(Icon::Circle, 8, color),
                text(label).size(10).color(color)
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .padding([7, 10])
        .style(theme::tint)
        .into()
    }

    fn metric(&self, glyph: Icon, label: &'static str, value: String) -> Element<'_, Message> {
        row![
            icons::icon(glyph, 17, theme::INK_SOFT),
            column![
                text(label).size(9).color(theme::INK_SOFT),
                text(value).size(12).font(fonts::MEDIUM),
            ]
            .spacing(3),
        ]
        .spacing(9)
        .align_y(Alignment::Center)
        .into()
    }

    fn activity_row(
        &self,
        glyph: Icon,
        title: &'static str,
        description: &'static str,
        time: &'static str,
    ) -> Element<'_, Message> {
        row![
            icons::icon(glyph, 17, theme::MINT),
            column![
                text(title).size(13).font(fonts::MEDIUM),
                text(description).size(11).color(theme::INK_SOFT)
            ]
            .spacing(4),
            space::horizontal(),
            text(time).size(11).color(theme::INK_SOFT),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .into()
    }

    fn library_row(
        &self,
        name: &'static str,
        version: &'static str,
        status: &'static str,
        glyph: Icon,
    ) -> Element<'_, Message> {
        container(
            row![
                icons::icon(glyph, 19, theme::COPPER),
                column![
                    text(name).size(14).font(fonts::MEDIUM),
                    text(version).size(11).color(theme::INK_SOFT)
                ]
                .spacing(4),
                space::horizontal(),
                text(status).size(11).color(theme::MINT),
                icons::icon(Icon::ChevronRight, 16, theme::INK_SOFT),
            ]
            .spacing(14)
            .align_y(Alignment::Center),
        )
        .padding(18)
        .width(Fill)
        .style(theme::card)
        .into()
    }

    fn setting_row(
        &self,
        glyph: Icon,
        name: &'static str,
        description: &'static str,
        value: &'static str,
    ) -> Element<'_, Message> {
        row![
            icons::icon(glyph, 18, theme::COPPER),
            column![
                text(name).size(14).font(fonts::MEDIUM),
                text(description).size(11).color(theme::INK_SOFT)
            ]
            .spacing(4),
            space::horizontal(),
            text(value).size(11).color(theme::INK_SOFT),
        ]
        .spacing(14)
        .align_y(Alignment::Center)
        .padding([12, 0])
        .width(Fill)
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::{Channel, Launcher, Message};

    #[test]
    fn launch_action_toggles_runtime_state() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::ToggleLaunch);
        assert!(launcher.running);

        let _ = launcher.update(Message::ToggleLaunch);
        assert!(!launcher.running);
    }

    #[test]
    fn selected_channel_is_retained() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::ChannelSelected(Channel::Nightly));

        assert_eq!(launcher.channel, Channel::Nightly);
    }
}
