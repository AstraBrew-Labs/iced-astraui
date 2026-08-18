pub fn toggle_button<'a, Message>(
    id: iced::widget::Id,
    content: impl Into<Element<'a, Message>>,
    selected: bool,
    active: bool,
    on_focus: Message,
    on_toggle: Message,
    variant: ToggleButtonVariant,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let control = toggle_button_item(
        content,
        selected,
        on_toggle.clone(),
        variant,
        GroupPosition::Standalone,
        Orientation::Horizontal,
    );
    navigation_group(
        id,
        control,
        Orientation::Horizontal,
        1,
        0,
        active,
        move |_| on_focus.clone(),
        move |_| on_toggle.clone(),
        None::<fn(usize) -> Message>,
    )
}

#[derive(Debug, Clone)]
pub struct ToggleButtonGroupItem<'a> {
    pub label: Option<&'a str>,
    pub icon: Option<LucideIcon>,
    pub selected: bool,
}

impl<'a> ToggleButtonGroupItem<'a> {
    pub const fn new(label: Option<&'a str>, icon: Option<LucideIcon>, selected: bool) -> Self {
        Self {
            label,
            icon,
            selected,
        }
    }
}

fn toggle_button_content<'a, Message: 'a>(
    item: &ToggleButtonGroupItem<'a>,
) -> Element<'a, Message> {
    let foreground = if item.selected { WHITE } else { INK_MUTED };
    let mut content = row![].spacing(6).align_y(iced::Alignment::Center);
    if let Some(icon) = item.icon {
        content = content.push(crate::icons::icon(icon, 15, foreground));
    }
    if let Some(label) = item.label {
        content = content.push(
            text(label)
                .size(11)
                .font(crate::fonts::MEDIUM)
                .line_height(iced::widget::text::LineHeight::Absolute(Pixels(16.0))),
        );
    }
    container(content)
        .height(Fill)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .into()
}

#[allow(clippy::too_many_arguments)]
pub fn toggle_button_group<'a, Message>(
    id: iced::widget::Id,
    items: Vec<ToggleButtonGroupItem<'a>>,
    focused_index: usize,
    active: bool,
    _selection_mode: SelectionMode,
    orientation: Orientation,
    detached: bool,
    on_focus: impl Fn(usize) -> Message + 'a,
    on_toggle: impl Fn(usize) -> Message + Clone + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let count = items.len();
    let spacing = if detached { 4 } else { 0 };
    let controls = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let position = if detached || count == 1 {
                GroupPosition::Standalone
            } else if index == 0 {
                GroupPosition::First
            } else if index + 1 == count {
                GroupPosition::Last
            } else {
                GroupPosition::Middle
            };
            toggle_button_item(
                toggle_button_content(item),
                item.selected,
                on_toggle.clone()(index),
                ToggleButtonVariant::Default,
                position,
                orientation,
            )
        })
        .collect::<Vec<_>>();
    let content: Element<'a, Message> = match orientation {
        Orientation::Horizontal => row(controls).spacing(spacing).into(),
        Orientation::Vertical => column(controls).spacing(spacing).into(),
    };
    let content = if !detached && count > 1 {
        container(content).style(toggle_button_group_surface).into()
    } else {
        content
    };

    navigation_group(
        id,
        content,
        orientation,
        count,
        focused_index,
        active,
        on_focus,
        on_toggle,
        None::<fn(usize) -> Message>,
    )
}

