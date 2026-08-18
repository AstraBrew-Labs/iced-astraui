/// A button-triggered action menu with optional leading icon support.
pub fn dropdown<'a, Message>(
    trigger_label: &'a str,
    trigger_icon: Option<LucideIcon>,
    expanded: bool,
    on_toggle: Message,
    on_dismiss: Message,
    items: Vec<MenuItem<'a, Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut content = row![].spacing(8).align_y(iced::Alignment::Center);
    if let Some(icon) = trigger_icon {
        content = content.push(crate::icons::icon(icon, 15, INK));
    }
    content = content
        .push(
            text(trigger_label)
                .size(12)
                .font(crate::fonts::MEDIUM)
                .width(Fill),
        )
        .push(crate::icons::icon(
            if expanded {
                LucideIcon::ChevronUp
            } else {
                LucideIcon::ChevronDown
            },
            15,
            if expanded { BLUE_600 } else { INK_MUTED },
        ));

    let trigger = button(content)
        .on_press(on_toggle)
        .width(176)
        .height(CONTROL_HEIGHT_MD)
        .padding([8, 13])
        .style(dropdown_button_style(expanded));

    Popup::dropdown(trigger.into(), menu_panel(items), expanded, on_dismiss).into()
}

