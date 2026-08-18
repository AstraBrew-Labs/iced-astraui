impl AlertKind {
    const fn accent(self) -> Color {
        match self {
            Self::Info => BLUE_600,
            Self::Success => SUCCESS,
            Self::Warning => WARNING,
            Self::Danger => DANGER,
        }
    }

    const fn icon(self) -> LucideIcon {
        match self {
            Self::Info => LucideIcon::Info,
            Self::Success => LucideIcon::CircleCheck,
            Self::Warning => LucideIcon::TriangleAlert,
            Self::Danger => LucideIcon::CircleX,
        }
    }
}

pub fn alert(kind: AlertKind) -> impl Fn(&Theme) -> container::Style {
    let accent = kind.accent();
    move |_theme| container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: Color::from_rgba(accent.r, accent.g, accent.b, 0.24),
            width: 1.0,
            radius: RADIUS_INNER.into(),
        },
        text_color: Some(INK),
        ..container::Style::default()
    }
}

/// An inline status message with a semantic indicator, optional action, and dismissal control.
pub struct Alert<'a, Message> {
    title: String,
    description: Option<String>,
    kind: AlertKind,
    indicator: Option<Element<'a, Message>>,
    action: Option<Element<'a, Message>>,
    on_close: Option<Message>,
}

impl<'a, Message> Alert<'a, Message>
where
    Message: 'a,
{
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            kind: AlertKind::Info,
            indicator: None,
            action: None,
            on_close: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub const fn kind(mut self, kind: AlertKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn indicator(mut self, indicator: impl Into<Element<'a, Message>>) -> Self {
        self.indicator = Some(indicator.into());
        self
    }

    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.action = Some(action.into());
        self
    }

    pub fn on_close(mut self, on_close: Message) -> Self {
        self.on_close = Some(on_close);
        self
    }
}

impl<'a, Message> From<Alert<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(notice: Alert<'a, Message>) -> Self {
        let accent = notice.kind.accent();
        let indicator: Element<'a, Message> = notice.indicator.unwrap_or_else(|| {
            container(crate::icons::icon(notice.kind.icon(), 17, accent))
                .width(32)
                .height(32)
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center)
                .style(tag_style(accent))
                .into()
        });
        let mut content = iced::widget::column![
            text(notice.title)
                .size(13)
                .font(crate::fonts::MEDIUM)
                .color(accent)
        ]
        .spacing(2)
        .width(Fill);
        if let Some(description) = notice.description {
            content = content.push(
                text(description)
                    .size(12)
                    .font(crate::fonts::REGULAR)
                    .color(INK_MUTED),
            );
        }

        let mut layout = row![indicator, content]
            .spacing(12)
            .align_y(iced::Alignment::Start);
        if let Some(action) = notice.action {
            layout = layout.push(action);
        }
        if let Some(on_close) = notice.on_close {
            layout = layout.push(
                button(centered_button_icon(LucideIcon::X, 14, INK_MUTED))
                    .on_press(on_close)
                    .width(30)
                    .height(30)
                    .padding(0)
                    .style(button_style(ButtonVariant::Ghost)),
            );
        }

        container(layout)
            .width(Fill)
            .padding([12, 14])
            .style(alert(notice.kind))
            .into()
    }
}

