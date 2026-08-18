/// A 40x20 switch whose track color and thumb position share one transition.
pub fn switch<'a, Message>(
    label: &'a str,
    is_toggled: bool,
    transition_progress: f32,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let progress = transition_progress.clamp(0.0, 1.0);
    let offset = switch_thumb_offset(progress);
    let track_color = mix_color(SURFACE_ALT, BLUE_600, progress);
    let control = stack![
        container(space::Space::new())
            .width(SWITCH_WIDTH)
            .height(SWITCH_HEIGHT)
            .style(switch_track(track_color)),
        container(row![
            space::Space::new().width(offset),
            container(space::Space::new())
                .width(SWITCH_THUMB_SIZE)
                .height(SWITCH_THUMB_SIZE)
                .style(switch_thumb)
        ])
        .width(SWITCH_WIDTH)
        .height(SWITCH_HEIGHT)
        .align_y(iced::Alignment::Center),
    ]
    .width(SWITCH_WIDTH)
    .height(SWITCH_HEIGHT);

    button(
        row![
            control,
            text(label).size(12).font(crate::fonts::REGULAR).color(INK)
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
    )
    .on_press(on_toggle(!is_toggled))
    .padding(0)
    .height(SWITCH_HEIGHT)
    .style(switch_button)
    .into()
}

