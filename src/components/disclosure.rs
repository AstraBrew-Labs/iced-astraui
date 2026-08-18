/// A titled, full-width section that reveals or hides its content panel.
pub fn disclosure<'a, Message>(
    title: &'a str,
    description: Option<&'a str>,
    expanded: bool,
    on_toggle: Message,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut labels = iced::widget::column![text(title).size(13).font(crate::fonts::MEDIUM).color(INK)].spacing(2);
    if let Some(description) = description {
        labels = labels.push(
            text(description)
                .size(10)
                .font(crate::fonts::REGULAR)
                .color(INK_MUTED),
        );
    }

    let trigger = button(
        row![
            labels.width(Fill),
            crate::icons::icon(
                if expanded {
                    LucideIcon::ChevronUp
                } else {
                    LucideIcon::ChevronDown
                },
                16,
                if expanded { BLUE_600 } else { INK_MUTED },
            )
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
    )
    .on_press(on_toggle)
    .width(Fill)
    .height(52)
    .padding([8, 14])
    .style(disclosure_trigger_style(expanded));

    let panel = if expanded {
        iced::widget::column![
            trigger,
            container(content.into())
                .width(Fill)
                .padding([14, 16])
                .style(disclosure_panel_surface)
        ]
    } else {
        iced::widget::column![trigger]
    };

    container(panel)
        .width(Fill)
        .style(disclosure_surface)
        .into()
}

