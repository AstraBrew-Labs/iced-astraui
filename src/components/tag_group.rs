#[derive(Debug, Clone)]
pub struct TagGroupItem<'a> {
    pub label: &'a str,
    pub icon: Option<LucideIcon>,
    pub selected: bool,
    pub removable: bool,
}

impl<'a> TagGroupItem<'a> {
    pub const fn new(label: &'a str, icon: Option<LucideIcon>, selected: bool) -> Self {
        Self {
            label,
            icon,
            selected,
            removable: false,
        }
    }

    pub const fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }
}

fn tag_group_surface(selected: bool, _focused: bool) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(if selected {
            Color::from_rgb8(220, 238, 255)
        } else {
            SURFACE
        })),
        border: Border {
            radius: RADIUS_CONTROL.into(),
            ..Border::default()
        },
        text_color: Some(if selected { BLUE_700 } else { INK }),
        ..container::Style::default()
    }
}

fn tag_group_button(selected: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, _status| button::Style {
        background: None,
        text_color: if selected { BLUE_700 } else { INK },
        border: Border {
            radius: RADIUS_CONTROL.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn tag_group<'a, Message>(
    id: iced::widget::Id,
    items: Vec<TagGroupItem<'a>>,
    focused_index: usize,
    active: bool,
    on_focus: impl Fn(usize) -> Message + 'a,
    on_select: impl Fn(usize) -> Message + Clone + 'a,
    on_remove: impl Fn(usize) -> Message + Clone + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let count = items.len();
    let tags = items
        .into_iter()
        .enumerate()
        .fold(row![].spacing(6), |tags, (index, item)| {
            let foreground = if item.selected { BLUE_700 } else { INK };
            let mut label = row![]
                .height(Fill)
                .spacing(5)
                .align_y(iced::Alignment::Center);
            if let Some(icon) = item.icon {
                label = label.push(crate::icons::icon(icon, 13, foreground));
            }
            label = label.push(
                text(item.label)
                    .size(11)
                    .font(crate::fonts::MEDIUM)
                    .line_height(iced::widget::text::LineHeight::Absolute(Pixels(16.0))),
            );

            let mut content = row![
                button(label)
                    .on_press(on_select.clone()(index))
                    .height(30)
                    .padding([0, 10])
                    .style(tag_group_button(item.selected)),
            ]
            .spacing(0)
            .align_y(iced::Alignment::Center);

            if item.removable {
                content = content.push(
                    button(centered_button_icon(LucideIcon::X, 12, foreground))
                        .on_press(on_remove.clone()(index))
                        .width(26)
                        .height(30)
                        .padding(0)
                        .style(tag_group_button(item.selected)),
                );
            }

            tags.push(container(content).style(tag_group_surface(
                item.selected,
                active && index == focused_index,
            )))
        })
        .wrap();

    navigation_group(
        id,
        tags,
        Orientation::Horizontal,
        count,
        focused_index,
        active,
        on_focus,
        on_select,
        Some(on_remove),
    )
}

fn toggle_button_style(
    selected: bool,
    variant: ToggleButtonVariant,
    position: GroupPosition,
    orientation: Orientation,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let pressed = matches!(status, button::Status::Pressed);
        let standalone = matches!(position, GroupPosition::Standalone);
        let radius = match (position, orientation) {
            (GroupPosition::Standalone, _) => RADIUS_CONTROL.into(),
            (GroupPosition::First, Orientation::Horizontal) => iced::border::left(RADIUS_FIELD),
            (GroupPosition::First, Orientation::Vertical) => iced::border::top(RADIUS_FIELD),
            (GroupPosition::Middle, _) => iced::border::Radius::default(),
            (GroupPosition::Last, Orientation::Horizontal) => iced::border::right(RADIUS_FIELD),
            (GroupPosition::Last, Orientation::Vertical) => iced::border::bottom(RADIUS_FIELD),
        };
        let background = if selected {
            Some(Background::Color(BLUE_600))
        } else if matches!(variant, ToggleButtonVariant::Default) {
            Some(Background::Color(SURFACE))
        } else {
            None
        };

        button::Style {
            background,
            text_color: if selected { WHITE } else { INK_MUTED },
            border: Border {
                color: if standalone { LINE } else { Color::TRANSPARENT },
                width: if standalone { 1.0 } else { 0.0 },
                radius,
            },
            shadow: if pressed && matches!(position, GroupPosition::Standalone) {
                Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                }
            } else {
                Shadow::default()
            },
            ..button::Style::default()
        }
    }
}

fn toggle_button_group_surface(_theme: &Theme) -> container::Style {
    container::Style {
        border: Border {
            color: LINE,
            width: 1.0,
            radius: RADIUS_FIELD.into(),
        },
        ..container::Style::default()
    }
}

fn toggle_button_item<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    selected: bool,
    on_toggle: Message,
    variant: ToggleButtonVariant,
    position: GroupPosition,
    orientation: Orientation,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    button(
        container(content)
            .height(Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center),
    )
    .on_press(on_toggle)
    .height(CONTROL_HEIGHT_MD)
    .padding([0, 11])
    .style(toggle_button_style(
        selected,
        variant,
        position,
        orientation,
    ))
    .into()
}

