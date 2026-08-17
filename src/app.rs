use iced::time::{self, Duration, Instant};
use iced::widget::text::LineHeight;
use iced::widget::{
    button, checkbox, column, container, pick_list, progress_bar, radio, row, rule, scrollable,
    space, text, text_input, toggler,
};
use iced::{Alignment, Element, Fill, Pixels, Subscription, Task, Theme};
use lucide_icons::Icon;

use crate::{fonts, icons, ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Page {
    #[default]
    Components,
    Tokens,
    Patterns,
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
            Self::Stable => "Stable / production",
            Self::Preview => "Preview / release candidate",
            Self::Nightly => "Nightly / latest commit",
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Navigate(Page),
    InputChanged(String),
    ToggleChanged(bool),
    CheckChanged(bool),
    SliderChanged(f32),
    ChannelSelected(Channel),
    TabSelected(usize),
    Action(&'static str),
    Tick(Instant),
}

fn button_text<'a>(label: &'a str, size: f32) -> Element<'a, Message> {
    text(label)
        .size(size)
        .font(fonts::MEDIUM)
        .line_height(LineHeight::Absolute(Pixels(20.0)))
        .into()
}

#[derive(Debug, Default)]
pub struct Launcher {
    page: Page,
    input: String,
    toggled: bool,
    checked: bool,
    slider: f32,
    channel: Channel,
    active_tab: usize,
    notice: Option<&'static str>,
    motion: ui::MotionState,
}

impl Launcher {
    pub fn new() -> (Self, Task<Message>) {
        let now = Instant::now();
        let mut motion = ui::MotionState::default();
        motion.start_progress(now);
        (
            Self {
                slider: 68.0,
                motion,
                ..Self::default()
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        "Astra UI / Local Iced component library".to_owned()
    }

    pub fn theme(&self) -> Theme {
        ui::app_theme()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.motion.needs_ticks(Instant::now()) {
            time::every(Duration::from_millis(16)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let now = Instant::now();
        match message {
            Message::Navigate(page) => {
                self.page = page;
                self.motion.press(
                    match page {
                        Page::Components => "nav-components",
                        Page::Tokens => "nav-tokens",
                        Page::Patterns => "nav-patterns",
                    },
                    now,
                );
            }
            Message::InputChanged(value) => self.input = value,
            Message::ToggleChanged(value) => {
                self.toggled = value;
                self.motion.set_toggled(value, now);
            }
            Message::CheckChanged(value) => {
                self.checked = value;
                self.motion.set_checked(value, now);
            }
            Message::SliderChanged(value) => self.slider = value.clamp(0.0, 100.0),
            Message::ChannelSelected(channel) => self.channel = channel,
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                self.motion.press(
                    match tab {
                        0 => "tab-0",
                        1 => "tab-1",
                        _ => "tab-2",
                    },
                    now,
                );
            }
            Message::Action(notice) => {
                self.notice = Some(notice);
                self.motion.press(notice, now);
            }
            Message::Tick(now) => self.motion.tick(now),
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
            container(icons::icon(Icon::Component, 20, ui::WHITE))
                .width(36)
                .height(36)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(ui::BLUE_600)),
                    border: iced::Border {
                        radius: 12.0.into(),
                        ..iced::Border::default()
                    },
                    ..container::Style::default()
                }),
            column![
                text("ASTRA UI").size(16).font(fonts::BLACK),
                text("LOCAL ICED KIT")
                    .size(10)
                    .font(fonts::LIGHT)
                    .color(ui::INK_MUTED),
            ]
            .spacing(1),
        ]
        .align_y(Alignment::Center)
        .spacing(11);

        let navigation = column![
            self.nav_button(
                Page::Components,
                Icon::Blocks,
                "Components",
                "12 primitives"
            ),
            self.nav_button(Page::Tokens, Icon::Palette, "Tokens", "Color + type"),
            self.nav_button(
                Page::Patterns,
                Icon::LayoutTemplate,
                "Patterns",
                "Composed UI"
            ),
        ]
        .spacing(7);

        let footer = column![
            text("DESIGN SYSTEM")
                .size(10)
                .font(fonts::LIGHT)
                .color(ui::INK_MUTED),
            row![
                icons::icon(Icon::Circle, 8, ui::CYAN_500),
                text("v0.1 / Blue + cyan").size(12).color(ui::INK_MUTED)
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text("Iced 0.14 / Rust 2024")
                .size(11)
                .font(fonts::LIGHT)
                .color(ui::INK_MUTED),
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
        .width(246)
        .height(Fill)
        .style(ui::sidebar)
        .into()
    }

    fn nav_button(
        &self,
        page: Page,
        glyph: Icon,
        label: &'static str,
        hint: &'static str,
    ) -> Element<'_, Message> {
        let active = self.page == page;
        button(
            row![
                icons::icon(glyph, 17, if active { ui::BLUE_600 } else { ui::INK_MUTED }),
                column![
                    text(label).size(13).font(fonts::MEDIUM),
                    text(hint).size(10).color(ui::INK_MUTED)
                ]
                .spacing(2),
                space::horizontal(),
                icons::icon(
                    Icon::ChevronRight,
                    15,
                    if active { ui::BLUE_600 } else { ui::INK_SUBTLE }
                ),
            ]
            .spacing(11)
            .align_y(Alignment::Center),
        )
        .on_press(Message::Navigate(page))
        .padding([10, 11])
        .width(Fill)
        .style(ui::nav_button_animated(
            active,
            self.motion.press_progress(
                match page {
                    Page::Components => "nav-components",
                    Page::Tokens => "nav-tokens",
                    Page::Patterns => "nav-patterns",
                },
                Instant::now(),
            ),
        ))
        .into()
    }

    fn content(&self) -> Element<'_, Message> {
        let body = match self.page {
            Page::Components => self.components_page(),
            Page::Tokens => self.tokens_page(),
            Page::Patterns => self.patterns_page(),
        };
        container(scrollable(body).width(Fill).height(Fill))
            .width(Fill)
            .height(Fill)
            .style(ui::canvas)
            .into()
    }

    fn components_page(&self) -> Element<'_, Message> {
        let now = Instant::now();
        let press = |id| self.motion.press_progress(id, now);
        let header = row![
            column![
                text("Astra UI").size(32).font(fonts::BLACK),
                text("A local component language for Iced apps.")
                    .size(14)
                    .color(ui::INK_MUTED)
            ]
            .spacing(5),
            space::horizontal(),
            container(
                row![
                    icons::icon(Icon::Sparkles, 15, ui::BLUE_600),
                    text("HeroUI V3 / Figma source")
                        .size(12)
                        .font(fonts::MEDIUM)
                ]
                .spacing(8)
                .align_y(Alignment::Center)
            )
            .padding([2, 8])
            .style(ui::badge(ui::BLUE_600)),
        ]
        .align_y(Alignment::Center);

        let buttons = self.component_card(
            "Buttons",
            "Semantic variants with shared hover, pressed, disabled, and focus treatment.",
            column![
                row![
                    button(button_text("Primary", 14.0))
                        .on_press(Message::Action("Primary action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Primary,
                            press("Primary action fired"),
                        )),
                    button(button_text("Secondary", 14.0))
                        .on_press(Message::Action("Secondary action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Secondary,
                            press("Secondary action fired"),
                        )),
                    button(button_text("Tertiary", 14.0))
                        .on_press(Message::Action("Tertiary action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Tertiary,
                            press("Tertiary action fired"),
                        )),
                    button(button_text("Ghost", 14.0))
                        .on_press(Message::Action("Ghost action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Ghost,
                            press("Ghost action fired")
                        )),
                    button(button_text("Danger", 14.0))
                        .on_press(Message::Action("Destructive action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Destructive,
                            press("Destructive action fired"),
                        )),
                    button(button_text("Danger soft", 14.0))
                        .on_press(Message::Action("Danger soft action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::DangerSoft,
                            press("Danger soft action fired"),
                        )),
                    button(button_text("Outline", 14.0))
                        .on_press(Message::Action("Outline action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Outline,
                            press("Outline action fired"),
                        )),
                ]
                .spacing(8)
                .wrap(),
                row![
                    button(
                        container(icons::icon(Icon::Plus, 16, ui::WHITE))
                            .width(Fill)
                            .height(Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                    )
                    .on_press(Message::Action("Icon action fired"))
                    .width(ui::CONTROL_HEIGHT_MD)
                    .height(ui::CONTROL_HEIGHT_MD)
                    .padding(0)
                    .style(ui::button_style_animated(
                        ui::ButtonVariant::Primary,
                        press("Icon action fired"),
                    )),
                    button(
                        container(icons::icon(Icon::Download, 16, ui::BLUE_700))
                            .width(Fill)
                            .height(Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                    )
                    .on_press(Message::Action("Download action fired"))
                    .width(ui::CONTROL_HEIGHT_MD)
                    .height(ui::CONTROL_HEIGHT_MD)
                    .padding(0)
                    .style(ui::button_style_animated(
                        ui::ButtonVariant::Secondary,
                        press("Download action fired"),
                    )),
                    button(
                        container(icons::icon(Icon::X, 16, ui::INK_MUTED))
                            .width(Fill)
                            .height(Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                    )
                    .on_press(Message::Action("Close action fired"))
                    .width(ui::CONTROL_HEIGHT_MD)
                    .height(ui::CONTROL_HEIGHT_MD)
                    .padding(0)
                    .style(ui::button_style_animated(
                        ui::ButtonVariant::Ghost,
                        press("Close action fired")
                    )),
                    text("Icon-only actions stay compact and discoverable.")
                        .size(11)
                        .color(ui::INK_MUTED),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            ]
            .spacing(18),
        );

        let fields = self.component_card(
            "Fields & controls",
            "Inputs use the same radius, focus ring, and semantic accent as every other control.",
            column![
                row![
                    column![
                        text("Email address")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        container(
                            text_input("you@example.com", &self.input)
                                .on_input(Message::InputChanged)
                                .padding([8, 12])
                                .size(14)
                                .line_height(LineHeight::Absolute(Pixels(20.0)))
                                .style(ui::text_input_style)
                        )
                        .style(ui::field_frame)
                    ]
                    .spacing(7)
                    .width(Fill),
                    column![
                        text("Release channel")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        container(
                            pick_list(Channel::ALL, Some(self.channel), Message::ChannelSelected)
                                .padding([8, 12])
                                .text_size(14)
                                .width(Fill)
                                .style(ui::pick_list_style)
                        )
                        .style(ui::field_frame)
                    ]
                    .spacing(7)
                    .width(Fill),
                ]
                .spacing(16),
                row![
                    toggler(self.toggled)
                        .label("Enable notifications")
                        .on_toggle(Message::ToggleChanged)
                        .size(20)
                        .spacing(10)
                        .text_size(12)
                        .style(ui::toggler_style_animated(
                            self.motion.toggled_progress(now),
                        )),
                    checkbox(self.checked)
                        .label("I agree to the design contract")
                        .on_toggle(Message::CheckChanged)
                        .size(16)
                        .spacing(9)
                        .text_size(12)
                        .style(ui::checkbox_style_animated(
                            self.motion.checked_progress(now),
                        )),
                ]
                .spacing(28)
                .align_y(Alignment::Center),
                column![
                    row![
                        text("Density")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        space::horizontal(),
                        text(format!("{:.0}%", self.slider))
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::BLUE_700)
                    ]
                    .width(Fill),
                    ui::slider(0.0..=100.0, self.slider, Message::SliderChanged)
                ]
                .spacing(7),
                row![
                    text("Color mode")
                        .size(11)
                        .font(fonts::MEDIUM)
                        .color(ui::INK_MUTED),
                    radio("Blue", 0, Some(self.active_tab), Message::TabSelected)
                        .size(16)
                        .style(ui::radio_style)
                        .text_size(12),
                    radio("Cyan", 1, Some(self.active_tab), Message::TabSelected)
                        .size(16)
                        .style(ui::radio_style)
                        .text_size(12)
                ]
                .spacing(15)
                .align_y(Alignment::Center),
            ]
            .spacing(16),
        );

        let feedback = self.component_card(
            "Feedback",
            "Status, progress, and inline messaging keep system state visible without noise.",
            column![
                row![
                    self.badge("READY", ui::SUCCESS),
                    self.badge("PREVIEW", ui::BLUE_600),
                    self.badge("BETA", ui::WARNING),
                    self.badge("DEPRECATED", ui::DANGER),
                    self.chip("component-kit"),
                    self.chip("iced-0.14")
                ]
                .spacing(8)
                .wrap(),
                row![
                    column![
                        text("Install progress")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        progress_bar(0.0..=100.0, 72.0 * self.motion.progress_progress(now),)
                            .girth(8)
                            .style(ui::progress_style)
                    ]
                    .spacing(8)
                    .width(Fill),
                    column![
                        text("Circular / indeterminate")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        container(text("...").size(18).font(fonts::BOLD).color(ui::BLUE_600))
                            .width(42)
                            .height(28)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .style(ui::tint)
                    ]
                    .spacing(8),
                ]
                .spacing(18)
                .align_y(Alignment::End),
                row![
                    container(
                        row![
                            icons::icon(Icon::Info, 16, ui::BLUE_600),
                            text("Info: styles are local and themeable.").size(11)
                        ]
                        .spacing(9)
                        .align_y(Alignment::Center)
                    )
                    .height(64)
                    .padding([0, 16])
                    .width(Fill)
                    .style(ui::alert(ui::AlertKind::Info)),
                    container(
                        row![
                            icons::icon(Icon::CircleCheck, 16, ui::SUCCESS),
                            text("Saved").size(11)
                        ]
                        .spacing(9)
                        .align_y(Alignment::Center)
                    )
                    .height(64)
                    .padding([0, 16])
                    .style(ui::alert(ui::AlertKind::Success)),
                ]
                .spacing(10),
            ]
            .spacing(16),
        );

        column![header, buttons, fields, feedback, self.notice_banner()]
            .spacing(22)
            .padding([34, 42])
            .width(Fill)
            .into()
    }

    fn component_card<'a>(
        &self,
        title: &'a str,
        description: &'a str,
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        container(
            column![
                text(title).size(17).font(fonts::BOLD),
                text(description).size(12).color(ui::INK_MUTED),
                rule::horizontal(1),
                content.into()
            ]
            .spacing(9),
        )
        .padding(16)
        .width(Fill)
        .style(ui::card)
        .into()
    }

    fn notice_banner(&self) -> Element<'_, Message> {
        self.notice.map_or_else(
            || space::vertical().height(0).into(),
            |notice| {
                container(
                    row![
                        icons::icon(Icon::CircleCheck, 16, ui::SUCCESS),
                        text(notice).size(12).font(fonts::MEDIUM)
                    ]
                    .spacing(9)
                    .align_y(Alignment::Center),
                )
                .padding([11, 14])
                .width(Fill)
                .style(ui::alert(ui::AlertKind::Success))
                .into()
            },
        )
    }

    fn badge(&self, label: &'static str, color: iced::Color) -> Element<'_, Message> {
        container(
            text(label)
                .size(12)
                .font(fonts::REGULAR)
                .line_height(LineHeight::Absolute(Pixels(20.0)))
                .color(color),
        )
        .height(24)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .padding([2, 8])
        .style(ui::badge(color))
        .into()
    }

    fn chip(&self, label: &'static str) -> Element<'_, Message> {
        container(
            row![
                icons::icon(Icon::Hash, 12, ui::BLUE_700),
                text(label)
                    .size(12)
                    .font(fonts::REGULAR)
                    .line_height(LineHeight::Absolute(Pixels(20.0)))
                    .color(ui::BLUE_700)
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .height(24)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .padding([2, 8])
        .style(ui::badge(ui::BLUE_700))
        .into()
    }

    fn tokens_page(&self) -> Element<'_, Message> {
        let swatches = [
            ("blue-700", ui::BLUE_700),
            ("blue-600", ui::BLUE_600),
            ("blue-500", ui::BLUE_500),
            ("cyan-500", ui::CYAN_500),
            ("cyan-300", ui::CYAN_300),
            ("navy-950", ui::NAVY_950),
        ];
        let swatch_row = swatches
            .into_iter()
            .map(|(name, color)| {
                container(
                    column![
                        container(space::vertical().height(44))
                            .width(Fill)
                            .style(move |_| container::Style {
                                background: Some(iced::Background::Color(color)),
                                border: iced::Border {
                                    radius: 8.0.into(),
                                    ..iced::Border::default()
                                },
                                ..container::Style::default()
                            }),
                        text(name).size(10).font(fonts::MEDIUM),
                        text(format!(
                            "#{:02X}{:02X}{:02X}",
                            (color.r * 255.0) as u8,
                            (color.g * 255.0) as u8,
                            (color.b * 255.0) as u8
                        ))
                        .size(10)
                        .color(ui::INK_MUTED),
                    ]
                    .spacing(7),
                )
                .padding(8)
                .width(Fill)
                .style(ui::flat_card)
            })
            .fold(row![].spacing(10).width(Fill), |row, swatch| {
                row.push(swatch)
            });

        column![
            text("Tokens").size(32).font(fonts::BLACK),
            text("A compact source of truth for color, surface, spacing, and type.")
                .size(14)
                .color(ui::INK_MUTED),
            rule::horizontal(1),
            container(
                column![text("Color palette").size(17).font(fonts::BOLD), swatch_row].spacing(14)
            )
            .padding(20)
            .width(Fill)
            .style(ui::card),
            row![
                self.token_card(
                    "TYPE SCALE",
                    column![
                        text("Display / 32").size(26).font(fonts::BLACK),
                        text("Heading / 20").size(20).font(fonts::BOLD),
                        text("Body / 14").size(14),
                        text("Caption / 11").size(11).color(ui::INK_MUTED)
                    ]
                    .spacing(9)
                ),
                self.token_card(
                    "RADIUS",
                    column![
                        text("12 px")
                            .size(25)
                            .font(fonts::BLACK)
                            .color(ui::BLUE_700),
                        text("Fields / chips / progress")
                            .size(11)
                            .color(ui::INK_MUTED),
                        text("24 px").size(18).font(fonts::BOLD).color(ui::CYAN_500),
                        text("Buttons / cards / alerts / tabs")
                            .size(11)
                            .color(ui::INK_MUTED)
                    ]
                    .spacing(8)
                ),
                self.token_card(
                    "SPACING",
                    column![
                        text("4 / 8 / 12 / 16 / 24").size(16).font(fonts::BOLD),
                        text("Use the same rhythm for every layout.")
                            .size(11)
                            .color(ui::INK_MUTED),
                        container(space::vertical().height(8))
                            .width(Fill)
                            .style(|_| container::Style {
                                background: Some(iced::Background::Color(ui::CYAN_500)),
                                border: iced::Border {
                                    radius: 4.0.into(),
                                    ..iced::Border::default()
                                },
                                ..container::Style::default()
                            })
                    ]
                    .spacing(12)
                ),
            ]
            .spacing(16),
        ]
        .spacing(22)
        .padding([34, 42])
        .width(Fill)
        .into()
    }

    fn token_card<'a>(
        &self,
        title: &'a str,
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        container(
            column![
                text(title)
                    .size(10)
                    .font(fonts::MEDIUM)
                    .color(ui::INK_MUTED),
                rule::horizontal(1),
                content.into()
            ]
            .spacing(12),
        )
        .padding(18)
        .width(Fill)
        .style(ui::card)
        .into()
    }

    fn patterns_page(&self) -> Element<'_, Message> {
        let now = Instant::now();
        let press = |id| self.motion.press_progress(id, now);
        let labels = ["Overview", "Usage", "Accessibility"];
        let tabs =
            labels
                .into_iter()
                .enumerate()
                .fold(row![].spacing(3), |tabs, (index, label)| {
                    tabs.push(
                        button(button_text(label, 12.0))
                            .on_press(Message::TabSelected(index))
                            .height(32)
                            .padding([6, 12])
                            .style(ui::tab_animated(
                                self.active_tab == index,
                                press(if index == 0 {
                                    "tab-0"
                                } else if index == 1 {
                                    "tab-1"
                                } else {
                                    "tab-2"
                                }),
                            )),
                    )
                });
        let content = match self.active_tab {
            0 => column![text("Composable patterns").size(18).font(fonts::BOLD), text("Cards, toolbars, and settings rows are assembled from the same primitives shown on the Components page.").size(12).color(ui::INK_MUTED), row![self.pattern_tile(Icon::Command, "Command bar", "Search, actions, and keyboard hints"), self.pattern_tile(Icon::PanelTop, "Settings row", "Label, description, and trailing control")].spacing(14)].spacing(14),
            1 => column![text("Usage recipe").size(18).font(fonts::BOLD), container(text("button(text(\"Save\")).style(ui::button_style(ui::ButtonVariant::Primary))").size(12).font(fonts::MEDIUM).color(ui::CYAN_300)).padding(16).width(Fill).style(ui::code_block), text("Import the module once, then use semantic variants in each screen.").size(12).color(ui::INK_MUTED)].spacing(14),
            _ => column![text("Accessibility defaults").size(18).font(fonts::BOLD), text("Focus borders, disabled colors, readable contrast, and explicit labels are part of the local styles. Keep labels attached to controls and use icons with tooltips in product screens.").size(12).color(ui::INK_MUTED)].spacing(14),
        };
        column![
            text("Patterns").size(32).font(fonts::BLACK),
            text("Small compositions that make product screens feel coherent.")
                .size(14)
                .color(ui::INK_MUTED),
            container(tabs).padding(4).style(ui::flat_card),
            container(content).padding(20).width(Fill).style(ui::card)
        ]
        .spacing(22)
        .padding([34, 42])
        .width(Fill)
        .into()
    }

    fn pattern_tile(
        &self,
        glyph: Icon,
        title: &'static str,
        description: &'static str,
    ) -> Element<'_, Message> {
        container(
            row![
                container(icons::icon(glyph, 18, ui::BLUE_600))
                    .padding(10)
                    .style(ui::tint),
                column![
                    text(title).size(13).font(fonts::MEDIUM),
                    text(description).size(11).color(ui::INK_MUTED)
                ]
                .spacing(4)
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .padding(14)
        .width(Fill)
        .style(ui::flat_card)
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::{Launcher, Message, Page};

    #[test]
    fn controls_update_without_side_effects() {
        let (mut launcher, _) = Launcher::new();
        let _ = launcher.update(Message::ToggleChanged(true));
        let _ = launcher.update(Message::InputChanged("hello".to_owned()));
        assert!(launcher.toggled);
        assert_eq!(launcher.input, "hello");
    }

    #[test]
    fn navigation_is_retained() {
        let (mut launcher, _) = Launcher::new();
        let _ = launcher.update(Message::Navigate(Page::Tokens));
        assert_eq!(launcher.page, Page::Tokens);
    }

    #[test]
    fn slider_updates_without_endpoint_lock() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::SliderChanged(0.0));
        assert_eq!(launcher.slider, 0.0);

        let _ = launcher.update(Message::SliderChanged(1.0));
        assert_eq!(launcher.slider, 1.0);

        let _ = launcher.update(Message::SliderChanged(99.0));
        assert_eq!(launcher.slider, 99.0);

        let _ = launcher.update(Message::SliderChanged(100.0));
        assert_eq!(launcher.slider, 100.0);

        let _ = launcher.update(Message::SliderChanged(98.0));
        assert_eq!(launcher.slider, 98.0);
    }
}
