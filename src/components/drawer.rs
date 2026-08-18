/// The edge from which a [`Drawer`] is presented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerPlacement {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// The visual treatment of a drawer's backdrop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerBackdrop {
    #[default]
    Opaque,
    Blur,
    Transparent,
}

/// Options shared by the drawer shell and its backdrop.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawerOptions {
    pub placement: DrawerPlacement,
    pub backdrop: DrawerBackdrop,
    pub close_on_backdrop: bool,
    pub show_close_button: bool,
    pub show_handle: bool,
    pub size: f32,
}

impl DrawerOptions {
    pub const fn placement(mut self, placement: DrawerPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub const fn backdrop(mut self, backdrop: DrawerBackdrop) -> Self {
        self.backdrop = backdrop;
        self
    }

    pub const fn close_on_backdrop(mut self, close_on_backdrop: bool) -> Self {
        self.close_on_backdrop = close_on_backdrop;
        self
    }

    pub const fn show_close_button(mut self, show_close_button: bool) -> Self {
        self.show_close_button = show_close_button;
        self
    }

    pub const fn show_handle(mut self, show_handle: bool) -> Self {
        self.show_handle = show_handle;
        self
    }

    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl Default for DrawerOptions {
    fn default() -> Self {
        Self {
            placement: DrawerPlacement::Bottom,
            backdrop: DrawerBackdrop::Opaque,
            close_on_backdrop: true,
            show_close_button: true,
            show_handle: false,
            size: 360.0,
        }
    }
}

fn drawer_backdrop_style(
    variant: DrawerBackdrop,
    progress: f32,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    let progress = progress.clamp(0.0, 1.0);
    move |_theme, _status| button::Style {
        background: Some(Background::Color(match variant {
            DrawerBackdrop::Opaque => Color::from_rgba(0.0, 0.0, 0.0, 0.42 * progress),
            // Iced does not expose a portable backdrop blur primitive. This
            // lighter tint keeps the variant visually distinct and stable.
            DrawerBackdrop::Blur => Color::from_rgba(
                24.0 / 255.0,
                24.0 / 255.0,
                27.0 / 255.0,
                0.26 * progress,
            ),
            DrawerBackdrop::Transparent => Color::TRANSPARENT,
        })),
        ..button::Style::default()
    }
}

fn drawer_transition_offset(placement: DrawerPlacement, size: f32, progress: f32) -> Vector {
    let distance = size.max(0.0) * (1.0 - progress.clamp(0.0, 1.0));
    match placement {
        DrawerPlacement::Top => Vector::new(0.0, -distance),
        DrawerPlacement::Bottom => Vector::new(0.0, distance),
        DrawerPlacement::Left => Vector::new(-distance, 0.0),
        DrawerPlacement::Right => Vector::new(distance, 0.0),
    }
}

fn drawer_surface_style(placement: DrawerPlacement) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let radius = match placement {
            DrawerPlacement::Bottom => iced::border::Radius {
                top_left: RADIUS_PANEL,
                top_right: RADIUS_PANEL,
                ..iced::border::Radius::default()
            },
            DrawerPlacement::Top => iced::border::Radius {
                bottom_left: RADIUS_PANEL,
                bottom_right: RADIUS_PANEL,
                ..iced::border::Radius::default()
            },
            DrawerPlacement::Left | DrawerPlacement::Right => iced::border::Radius::default(),
        };
        container::Style {
            background: Some(Background::Color(SURFACE)),
            border: Border {
                color: LINE,
                width: 1.0,
                radius,
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.18),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 24.0,
            },
            text_color: Some(INK),
            ..container::Style::default()
        }
    }
}

/// A controlled edge-aligned panel for supplementary content and actions.
pub struct Drawer<'a, Message> {
    title: String,
    description: Option<String>,
    body: Element<'a, Message>,
    footer: Option<Element<'a, Message>>,
    options: DrawerOptions,
    animation_progress: f32,
    on_close: Message,
    on_interact: Message,
}

impl<'a, Message> Drawer<'a, Message>
where
    Message: 'a,
{
    /// Creates a drawer that is already open when rendered.
    pub fn new(
        title: impl Into<String>,
        body: impl Into<Element<'a, Message>>,
        on_close: Message,
        on_interact: Message,
    ) -> Self {
        Self {
            title: title.into(),
            description: None,
            body: body.into(),
            footer: None,
            options: DrawerOptions::default(),
            animation_progress: 1.0,
            on_close,
            on_interact,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub const fn placement(mut self, placement: DrawerPlacement) -> Self {
        self.options.placement = placement;
        self
    }

    pub const fn backdrop(mut self, backdrop: DrawerBackdrop) -> Self {
        self.options.backdrop = backdrop;
        self
    }

    pub const fn close_on_backdrop(mut self, close_on_backdrop: bool) -> Self {
        self.options.close_on_backdrop = close_on_backdrop;
        self
    }

    pub const fn show_close_button(mut self, show_close_button: bool) -> Self {
        self.options.show_close_button = show_close_button;
        self
    }

    pub const fn show_handle(mut self, show_handle: bool) -> Self {
        self.options.show_handle = show_handle;
        self
    }

    pub const fn size(mut self, size: f32) -> Self {
        self.options.size = size;
        self
    }

    pub const fn options(mut self, options: DrawerOptions) -> Self {
        self.options = options;
        self
    }

    /// Sets the 0..=1 transition progress for the panel and its backdrop.
    pub const fn animation_progress(mut self, progress: f32) -> Self {
        self.animation_progress = progress;
        self
    }
}

impl<'a, Message> From<Drawer<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(drawer: Drawer<'a, Message>) -> Self {
        let Drawer {
            title,
            description,
            body,
            footer,
            options,
            animation_progress,
            on_close,
            on_interact,
        } = drawer;

        let mut header = iced::widget::column![
            text(title)
                .size(18)
                .font(crate::fonts::MEDIUM)
                .color(INK),
        ]
        .spacing(4)
        .width(Fill);
        if let Some(description) = description {
            header = header.push(
                text(description)
                    .size(12)
                    .font(crate::fonts::REGULAR)
                    .color(INK_MUTED),
            );
        }

        let mut header_row = row![header].spacing(12).align_y(iced::Alignment::Center);
        if options.show_close_button {
            header_row = header_row.push(
                button(centered_button_icon(LucideIcon::X, 17, INK_MUTED))
                    .on_press(on_close.clone())
                    .width(32)
                    .height(32)
                    .padding(0)
                    .style(button_style(ButtonVariant::Ghost)),
            );
        }

        let mut content = iced::widget::column![]
            .spacing(16)
            .width(Fill)
            .height(Fill);
        if options.show_handle {
            content = content.push(
                container(
                    container(space::horizontal().width(36).height(4))
                        .style(|_theme: &Theme| container::Style {
                            background: Some(Background::Color(LINE)),
                            border: Border {
                                radius: RADIUS_CONTROL.into(),
                                ..Border::default()
                            },
                            ..container::Style::default()
                        }),
                )
                .width(Fill)
                .align_x(iced::Alignment::Center),
            );
        }
        content = content.push(header_row);
        content = content.push(scrollable(body).width(Fill).height(Fill));
        if let Some(footer) = footer {
            content = content.push(footer);
        }

        let panel = mouse_area(
            container(content)
                .padding(24)
                .style(drawer_surface_style(options.placement)),
        )
        .on_press(on_interact.clone());
        let panel: Element<'a, Message> = match options.placement {
            DrawerPlacement::Left | DrawerPlacement::Right => {
                container(panel).width(options.size).height(Fill).into()
            }
            DrawerPlacement::Top | DrawerPlacement::Bottom => {
                container(panel).width(Fill).height(options.size).into()
            }
        };

        let backdrop_action = if options.close_on_backdrop {
            on_close
        } else {
            on_interact
        };
        let progress = animation_progress.clamp(0.0, 1.0);
        let backdrop = button(space::Space::new())
            .on_press(backdrop_action)
            .width(Fill)
            .height(Fill)
            .padding(0)
            .style(drawer_backdrop_style(options.backdrop, progress));
        let panel = translated(
            panel,
            drawer_transition_offset(options.placement, options.size, progress),
        );
        let aligned_panel = match options.placement {
            DrawerPlacement::Top => container(panel)
                .width(Fill)
                .height(Fill)
                .align_y(iced::Alignment::Start),
            DrawerPlacement::Bottom => container(panel)
                .width(Fill)
                .height(Fill)
                .align_y(iced::Alignment::End),
            DrawerPlacement::Left => container(panel)
                .width(Fill)
                .height(Fill)
                .align_x(iced::Alignment::Start),
            DrawerPlacement::Right => container(panel)
                .width(Fill)
                .height(Fill)
                .align_x(iced::Alignment::End),
        };

        stack![backdrop, aligned_panel]
            .width(Fill)
            .height(Fill)
            .into()
    }
}

/// Convenience constructor for a default bottom drawer.
pub fn drawer<'a, Message>(
    title: impl Into<String>,
    body: impl Into<Element<'a, Message>>,
    on_close: Message,
    on_interact: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    Drawer::new(title, body, on_close, on_interact).into()
}
