#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalModalOptions {
    close_on_backdrop: bool,
    show_close_button: bool,
}

impl GlobalModalOptions {
    pub const fn close_on_backdrop(mut self, close_on_backdrop: bool) -> Self {
        self.close_on_backdrop = close_on_backdrop;
        self
    }

    pub const fn confirmation() -> Self {
        Self {
            close_on_backdrop: false,
            show_close_button: false,
        }
    }
}

impl Default for GlobalModalOptions {
    fn default() -> Self {
        Self {
            close_on_backdrop: true,
            show_close_button: true,
        }
    }
}

/// A required-action global dialog for consequential confirmations.
pub struct AlertDialog<'a, Message> {
    title: String,
    description: String,
    body: Option<Element<'a, Message>>,
    status: AlertKind,
    cancel_label: String,
    confirm_label: String,
    destructive: bool,
    animation_progress: f32,
    on_cancel: Message,
    on_confirm: Message,
    on_interact: Message,
}

impl<'a, Message> AlertDialog<'a, Message>
where
    Message: 'a,
{
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        on_cancel: Message,
        on_confirm: Message,
        on_interact: Message,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            body: None,
            status: AlertKind::Danger,
            cancel_label: "Cancel".to_owned(),
            confirm_label: "Confirm".to_owned(),
            destructive: false,
            animation_progress: 1.0,
            on_cancel,
            on_confirm,
            on_interact,
        }
    }

    pub fn body(mut self, body: impl Into<Element<'a, Message>>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub const fn status(mut self, status: AlertKind) -> Self {
        self.status = status;
        self
    }

    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn confirm_label(mut self, label: impl Into<String>) -> Self {
        self.confirm_label = label.into();
        self
    }

    pub const fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    /// Sets the 0..=1 transition progress for the dialog and its backdrop.
    pub const fn animation_progress(mut self, progress: f32) -> Self {
        self.animation_progress = progress;
        self
    }
}

impl<'a, Message> From<AlertDialog<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(alert_dialog: AlertDialog<'a, Message>) -> Self {
        let accent = alert_dialog.status.accent();
        let progress = alert_dialog.animation_progress.clamp(0.0, 1.0);
        let icon = container(crate::icons::icon(alert_dialog.status.icon(), 20, accent))
            .width(40)
            .height(40)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .style(tag_style(accent));
        let header = iced::widget::column![
            icon,
            text(alert_dialog.title)
                .size(17)
                .font(crate::fonts::MEDIUM)
                .color(INK),
        ]
        .spacing(12)
        .width(Fill);
        let mut body = iced::widget::column![
            text(alert_dialog.description)
                .size(13)
                .font(crate::fonts::REGULAR)
                .color(INK_MUTED)
        ]
        .spacing(10)
        .width(Fill);
        if let Some(custom_body) = alert_dialog.body {
            body = body.push(custom_body);
        }

        let cancel = button(
            container(
                text(alert_dialog.cancel_label)
                    .size(13)
                    .font(crate::fonts::MEDIUM)
                    .line_height(iced::widget::text::LineHeight::Absolute(Pixels(20.0))),
            )
            .height(Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center),
        )
        .on_press(alert_dialog.on_cancel)
        .height(CONTROL_HEIGHT_MD)
        .padding([0, 16])
        .style(button_style(ButtonVariant::Secondary));
        let confirm = button(
            container(
                text(alert_dialog.confirm_label)
                    .size(13)
                    .font(crate::fonts::MEDIUM)
                    .line_height(iced::widget::text::LineHeight::Absolute(Pixels(20.0))),
            )
            .height(Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center),
        )
        .on_press(alert_dialog.on_confirm)
        .height(CONTROL_HEIGHT_MD)
        .padding([0, 16])
        .style(button_style(if alert_dialog.destructive {
            ButtonVariant::Destructive
        } else {
            ButtonVariant::Primary
        }));
        let dialog = mouse_area(
            container(
                iced::widget::column![
                    header,
                    body,
                    row![space::horizontal(), cancel, confirm]
                        .spacing(8)
                        .align_y(iced::Alignment::Center),
                ]
                .spacing(18),
            )
            .width(400)
            .padding(24)
            .style(modal_surface),
        )
        .on_press(alert_dialog.on_interact.clone());

        stack![
            button(space::Space::new())
                .on_press(alert_dialog.on_interact)
                .width(Fill)
                .height(Fill)
                .padding(0)
                .style(modal_backdrop(progress)),
            container(translated(
                dialog,
                Vector::new(0.0, (1.0 - progress) * -16.0),
            ))
                .width(Fill)
                .height(Fill)
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center)
                .padding(24),
        ]
        .width(Fill)
        .height(Fill)
        .into()
    }
}

/// A global modal shell. Clicking the backdrop closes it by default.
#[allow(clippy::too_many_arguments)]
pub fn global_modal<'a, Message>(
    title: &'a str,
    description: &'a str,
    body: impl Into<Element<'a, Message>>,
    cancel_label: &'a str,
    confirm_label: &'a str,
    destructive: bool,
    on_cancel: Message,
    on_confirm: Message,
    on_interact: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    global_modal_with_options(
        title,
        description,
        body,
        cancel_label,
        confirm_label,
        destructive,
        on_cancel,
        on_confirm,
        on_interact,
        GlobalModalOptions::default(),
    )
}

/// An animated global modal shell using the default modal options.
#[allow(clippy::too_many_arguments)]
pub fn global_modal_animated<'a, Message>(
    title: &'a str,
    description: &'a str,
    body: impl Into<Element<'a, Message>>,
    cancel_label: &'a str,
    confirm_label: &'a str,
    destructive: bool,
    on_cancel: Message,
    on_confirm: Message,
    on_interact: Message,
    animation_progress: f32,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    global_modal_with_options_animated(
        title,
        description,
        body,
        cancel_label,
        confirm_label,
        destructive,
        on_cancel,
        on_confirm,
        on_interact,
        GlobalModalOptions::default(),
        animation_progress,
    )
}

/// An animated global modal shell with configurable dismissal behavior.
#[allow(clippy::too_many_arguments)]
pub fn global_modal_with_options_animated<'a, Message>(
    title: &'a str,
    description: &'a str,
    body: impl Into<Element<'a, Message>>,
    cancel_label: &'a str,
    confirm_label: &'a str,
    destructive: bool,
    on_cancel: Message,
    on_confirm: Message,
    on_interact: Message,
    options: GlobalModalOptions,
    animation_progress: f32,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let progress = animation_progress.clamp(0.0, 1.0);
    let mut header = row![
        iced::widget::column![
            text(title).size(18).font(crate::fonts::BOLD).color(INK),
            text(description)
                .size(12)
                .font(crate::fonts::REGULAR)
                .color(INK_MUTED)
        ]
        .spacing(4)
        .width(Fill),
    ]
    .spacing(16)
    .align_y(iced::Alignment::Center);

    if options.show_close_button {
        header = header.push(
            button(centered_button_icon(LucideIcon::X, 17, INK_MUTED))
                .on_press(on_cancel.clone())
                .width(32)
                .height(32)
                .padding(0)
                .style(button_style(ButtonVariant::Ghost)),
        );
    }

    let dialog = mouse_area(
        container(
            iced::widget::column![
                header,
                rule::horizontal(1),
                body.into(),
                rule::horizontal(1),
                row![
                    space::horizontal(),
                    button(text(cancel_label).size(13).font(crate::fonts::MEDIUM))
                        .on_press(on_cancel.clone())
                        .height(CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(button_style(ButtonVariant::Secondary)),
                    button(text(confirm_label).size(13).font(crate::fonts::MEDIUM))
                        .on_press(on_confirm)
                        .height(CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(button_style(if destructive {
                            ButtonVariant::Destructive
                        } else {
                            ButtonVariant::Primary
                        })),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
            ]
            .spacing(16),
        )
        .width(440)
        .padding(20)
        .style(modal_surface),
    )
    .on_press(on_interact.clone());

    let backdrop_action = if options.close_on_backdrop {
        on_cancel
    } else {
        on_interact
    };

    stack![
        button(space::Space::new())
            .on_press(backdrop_action)
            .width(Fill)
            .height(Fill)
            .padding(0)
            .style(modal_backdrop(progress)),
        container(translated(
            dialog,
            Vector::new(0.0, (1.0 - progress) * -16.0),
        ))
        .width(Fill)
        .height(Fill)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .padding(24),
    ]
    .width(Fill)
    .height(Fill)
    .into()
}

/// A global modal shell with configurable dismissal behavior.
#[allow(clippy::too_many_arguments)]
pub fn global_modal_with_options<'a, Message>(
    title: &'a str,
    description: &'a str,
    body: impl Into<Element<'a, Message>>,
    cancel_label: &'a str,
    confirm_label: &'a str,
    destructive: bool,
    on_cancel: Message,
    on_confirm: Message,
    on_interact: Message,
    options: GlobalModalOptions,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    global_modal_with_options_animated(
        title,
        description,
        body,
        cancel_label,
        confirm_label,
        destructive,
        on_cancel,
        on_confirm,
        on_interact,
        options,
        1.0,
    )
}
