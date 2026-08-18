#[allow(clippy::too_many_arguments)]
pub fn toolbar<'a, Message>(
    id: iced::widget::Id,
    items: Vec<Element<'a, Message>>,
    focused_index: usize,
    active: bool,
    orientation: Orientation,
    attached: bool,
    on_focus: impl Fn(usize) -> Message + 'a,
    on_activate: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let count = items.len();
    let content: Element<'a, Message> = match orientation {
        Orientation::Horizontal => row(items)
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into(),
        Orientation::Vertical => column(items).spacing(4).into(),
    };
    let content = if attached {
        container(content).padding(4).style(toolbar_surface).into()
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
        on_activate,
        None::<fn(usize) -> Message>,
    )
}

