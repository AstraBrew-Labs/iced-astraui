pub fn toggle_accordion_item(expanded: &mut [bool], index: usize, mode: AccordionSelectionMode) {
    let Some(was_expanded) = expanded.get(index).copied() else {
        return;
    };

    if mode == AccordionSelectionMode::Single {
        expanded.fill(false);
    }
    if let Some(item) = expanded.get_mut(index) {
        *item = !was_expanded;
    }
}

fn accordion_surface(variant: AccordionVariant) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: (variant == AccordionVariant::Surface).then_some(Background::Color(SURFACE)),
        border: if variant == AccordionVariant::Surface {
            Border {
                color: LINE,
                width: 1.0,
                radius: RADIUS_FIELD.into(),
            }
        } else {
            Border::default()
        },
        ..container::Style::default()
    }
}

fn accordion_trigger_style(
    expanded: bool,
    first: bool,
    last: bool,
    variant: AccordionVariant,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        let background = if pressed {
            Some(Background::Color(Color::from_rgba(
                BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.09,
            )))
        } else if hovered && !expanded {
            Some(Background::Color(Color::from_rgba(
                INK.r, INK.g, INK.b, 0.035,
            )))
        } else if variant == AccordionVariant::Surface {
            Some(Background::Color(SURFACE))
        } else {
            None
        };
        let radius = if first && last && !expanded {
            RADIUS_FIELD.into()
        } else if first {
            iced::border::top(RADIUS_FIELD)
        } else if last && !expanded {
            iced::border::bottom(RADIUS_FIELD)
        } else {
            iced::border::Radius::default()
        };

        button::Style {
            background,
            text_color: INK,
            border: Border {
                radius,
                ..Border::default()
            },
            ..button::Style::default()
        }
    }
}

/// One externally controlled item inside an [`Accordion`].
pub struct AccordionItem<'a, Message> {
    title: String,
    description: Option<String>,
    expanded: bool,
    disabled: bool,
    on_toggle: Message,
    content: Element<'a, Message>,
}

impl<'a, Message> AccordionItem<'a, Message>
where
    Message: 'a,
{
    pub fn new(
        title: impl Into<String>,
        expanded: bool,
        on_toggle: Message,
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            title: title.into(),
            description: None,
            expanded,
            disabled: false,
            on_toggle,
            content: content.into(),
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// A connected group of collapsible items with optional surface treatment.
pub struct Accordion<'a, Message> {
    items: Vec<AccordionItem<'a, Message>>,
    variant: AccordionVariant,
}

impl<'a, Message> Accordion<'a, Message>
where
    Message: 'a,
{
    pub fn new(items: Vec<AccordionItem<'a, Message>>) -> Self {
        Self {
            items,
            variant: AccordionVariant::Default,
        }
    }

    pub const fn variant(mut self, variant: AccordionVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl<'a, Message> From<Accordion<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(accordion: Accordion<'a, Message>) -> Self {
        let item_count = accordion.items.len();
        let mut items: Vec<Element<'a, Message>> = Vec::with_capacity(item_count * 2);

        for (index, item) in accordion.items.into_iter().enumerate() {
            let first = index == 0;
            let last = index + 1 == item_count;
            let mut labels = iced::widget::column![
                text(item.title)
                    .size(13)
                    .font(crate::fonts::MEDIUM)
                    .color(INK)
            ]
            .spacing(2)
            .width(Fill);
            if let Some(description) = item.description {
                labels = labels.push(
                    text(description)
                        .size(11)
                        .font(crate::fonts::REGULAR)
                        .color(INK_MUTED),
                );
            }

            let trigger = button(
                row![
                    labels,
                    crate::icons::icon(
                        if item.expanded {
                            LucideIcon::ChevronUp
                        } else {
                            LucideIcon::ChevronDown
                        },
                        16,
                        if item.expanded { BLUE_600 } else { INK_MUTED },
                    )
                ]
                .spacing(12)
                .align_y(iced::Alignment::Center),
            )
            .on_press_maybe((!item.disabled).then_some(item.on_toggle))
            .width(Fill)
            .height(52)
            .padding([10, 14])
            .style(accordion_trigger_style(
                item.expanded,
                first,
                last,
                accordion.variant,
            ));
            let mut section = iced::widget::column![trigger].width(Fill);
            if item.expanded {
                section =
                    section.push(container(item.content).width(Fill).padding(iced::Padding {
                        top: 0.0,
                        right: 14.0,
                        bottom: 14.0,
                        left: 14.0,
                    }));
            }
            items.push(section.into());
            if !last {
                items.push(
                    container(rule::horizontal(1))
                        .padding([0, 14])
                        .width(Fill)
                        .into(),
                );
            }
        }

        container(column(items).width(Fill))
            .width(Fill)
            .style(accordion_surface(accordion.variant))
            .into()
    }
}

fn switch_thumb_offset(progress: f32) -> f32 {
    let travel = SWITCH_WIDTH - SWITCH_PADDING * 2.0 - SWITCH_THUMB_SIZE;
    SWITCH_PADDING + travel * progress.clamp(0.0, 1.0)
}

fn switch_track(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(color)),
        border: Border {
            radius: (SWITCH_HEIGHT / 2.0).into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

fn switch_thumb(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(WHITE)),
        border: Border {
            radius: (SWITCH_THUMB_SIZE / 2.0).into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.18),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 3.0,
        },
        ..container::Style::default()
    }
}

fn switch_button(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: INK,
        border: Border {
            radius: RADIUS_CONTROL.into(),
            ..Border::default()
        },
        shadow: Shadow::default(),
        ..button::Style::default()
    }
}

