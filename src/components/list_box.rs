/// Selection behavior for [`ListBox`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListBoxSelectionMode {
    None,
    #[default]
    Single,
    Multiple,
}

/// Visual treatment for a list item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListBoxItemVariant {
    #[default]
    Default,
    Danger,
}

/// Alias for the list-level `variant` prop used by HeroUI.
pub type ListBoxVariant = ListBoxItemVariant;

fn list_box_surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: LINE,
            width: 1.0,
            radius: RADIUS_FIELD.into(),
        },
        ..container::Style::default()
    }
}

fn list_box_item_style(
    selected: bool,
    disabled: bool,
    variant: ListBoxItemVariant,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        let danger = variant == ListBoxItemVariant::Danger;
        button::Style {
            background: if selected {
                Some(Background::Color(if danger {
                    Color::from_rgba(DANGER.r, DANGER.g, DANGER.b, 0.10)
                } else {
                    Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.10)
                }))
            } else if (hovered || pressed) && !disabled {
                Some(Background::Color(if danger {
                    Color::from_rgba(DANGER.r, DANGER.g, DANGER.b, 0.07)
                } else {
                    Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.06)
                }))
            } else {
                None
            },
            text_color: if disabled {
                INK_SUBTLE
            } else if danger {
                DANGER
            } else {
                INK
            },
            border: Border {
                radius: RADIUS_CONTROL.into(),
                ..Border::default()
            },
            ..button::Style::default()
        }
    }
}

/// A selectable row in a [`ListBox`].
pub struct ListBoxItem<'a, Message> {
    id: String,
    label: Element<'a, Message>,
    description: Option<Element<'a, Message>>,
    leading: Option<Element<'a, Message>>,
    disabled: bool,
    selected: bool,
    indicator: bool,
    variant: ListBoxItemVariant,
    on_select: Option<Message>,
}

impl<'a, Message> ListBoxItem<'a, Message>
where
    Message: 'a,
{
    pub fn new(id: impl Into<String>, label: impl Into<Element<'a, Message>>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            leading: None,
            disabled: false,
            selected: false,
            indicator: true,
            variant: ListBoxItemVariant::Default,
            on_select: None,
        }
    }

    pub fn description(mut self, description: impl Into<Element<'a, Message>>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn leading(mut self, leading: impl Into<Element<'a, Message>>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub const fn indicator(mut self, visible: bool) -> Self {
        self.indicator = visible;
        self
    }

    pub const fn variant(mut self, variant: ListBoxItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn on_select(mut self, message: Message) -> Self {
        self.on_select = Some(message);
        self
    }

    pub fn item_id(&self) -> &str {
        &self.id
    }

    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }
}

/// A titled group of list-box items.
pub struct ListBoxSection<'a, Message> {
    heading: Option<String>,
    items: Vec<ListBoxItem<'a, Message>>,
}

impl<'a, Message> ListBoxSection<'a, Message>
where
    Message: 'a,
{
    pub fn new(items: Vec<ListBoxItem<'a, Message>>) -> Self {
        Self {
            heading: None,
            items,
        }
    }

    pub fn heading(mut self, heading: impl Into<String>) -> Self {
        self.heading = Some(heading.into());
        self
    }
}

/// A controlled single- or multi-selection list with optional sections.
pub struct ListBox<'a, Message> {
    sections: Vec<ListBoxSection<'a, Message>>,
    selection_mode: ListBoxSelectionMode,
    selected: Vec<usize>,
    variant: ListBoxItemVariant,
    width: Length,
    on_selection_change: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_action: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> ListBox<'a, Message>
where
    Message: 'a,
{
    pub fn new(items: Vec<ListBoxItem<'a, Message>>) -> Self {
        Self {
            sections: vec![ListBoxSection::new(items)],
            selection_mode: ListBoxSelectionMode::Single,
            selected: Vec::new(),
            variant: ListBoxItemVariant::Default,
            width: Length::Shrink,
            on_selection_change: None,
            on_action: None,
        }
    }

    pub fn from_sections(sections: Vec<ListBoxSection<'a, Message>>) -> Self {
        Self {
            sections,
            selection_mode: ListBoxSelectionMode::Single,
            selected: Vec::new(),
            variant: ListBoxItemVariant::Default,
            width: Length::Shrink,
            on_selection_change: None,
            on_action: None,
        }
    }

    pub fn section(
        mut self,
        heading: impl Into<String>,
        items: Vec<ListBoxItem<'a, Message>>,
    ) -> Self {
        self.sections.push(ListBoxSection::new(items).heading(heading));
        self
    }

    pub const fn selection_mode(mut self, selection_mode: ListBoxSelectionMode) -> Self {
        self.selection_mode = selection_mode;
        self
    }

    pub const fn variant(mut self, variant: ListBoxItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = vec![index];
        self
    }

    pub fn selected_many(mut self, indices: impl Into<Vec<usize>>) -> Self {
        self.selected = indices.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn on_selection_change(
        mut self,
        on_selection_change: impl Fn(usize) -> Message + 'a,
    ) -> Self {
        self.on_selection_change = Some(Rc::new(on_selection_change));
        self
    }

    pub fn on_action(mut self, on_action: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_action = Some(Rc::new(on_action));
        self
    }
}

impl<'a, Message> From<ListBox<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(list_box: ListBox<'a, Message>) -> Self {
        let ListBox {
            sections,
            selection_mode,
            selected,
            variant: list_variant,
            width,
            on_selection_change,
            on_action,
        } = list_box;
        let section_count = sections.len();
        let mut rows: Vec<Element<'a, Message>> = Vec::new();
        let mut flat_index = 0;

        for (section_index, section) in sections.into_iter().enumerate() {
            if let Some(heading) = section.heading {
                rows.push(
                    text(heading)
                        .size(11)
                        .font(crate::fonts::MEDIUM)
                        .color(INK_MUTED)
                        .into(),
                );
            }
            for item in section.items {
                let ListBoxItem {
                    id: _id,
                    label,
                    description,
                    leading,
                    disabled,
                    selected: item_selected,
                    indicator,
                    variant: item_variant,
                    on_select,
                } = item;
                let selected = item_selected || selected.contains(&flat_index);
                let variant = if item_variant == ListBoxItemVariant::Default {
                    list_variant
                } else {
                    item_variant
                };
                let mut labels = iced::widget::column![label]
                    .spacing(2)
                    .width(Fill)
                    .height(Length::Shrink);
                if let Some(description) = description {
                    labels = labels.push(description);
                }
                let mut content = row![labels]
                    .spacing(10)
                    .align_y(iced::Alignment::Center);
                if let Some(leading) = leading {
                    content = row![leading, content].spacing(10).align_y(iced::Alignment::Center);
                }
                if indicator && selection_mode != ListBoxSelectionMode::None {
                    let icon = if selected {
                        crate::icons::icon(LucideIcon::Check, 16, BLUE_600)
                    } else {
                        space::horizontal().width(16).into()
                    };
                    content = content.push(icon);
                }

                let mut button = button(content)
                    .width(Fill)
                    .height(Length::Shrink)
                    .padding([8, 10])
                    .style(list_box_item_style(selected, disabled, variant));
                if !disabled {
                    let message = match selection_mode {
                        ListBoxSelectionMode::None => {
                            on_action.as_ref().map(|callback| callback(flat_index)).or(on_select)
                        }
                        _ => on_selection_change
                            .as_ref()
                            .map(|callback| callback(flat_index))
                            .or_else(|| on_action.as_ref().map(|callback| callback(flat_index)))
                            .or(on_select),
                    };
                    button = button.on_press_maybe(message);
                }
                rows.push(button.into());
                flat_index += 1;
            }
            if section_index + 1 < section_count {
                rows.push(container(rule::horizontal(1)).padding([4, 10]).into());
            }
        }

        container(iced::widget::column(rows).spacing(2))
            .padding(4)
            .width(width)
            .style(list_box_surface)
            .into()
    }
}

/// Convenience constructor for a list box.
pub fn list_box<'a, Message>(
    items: Vec<ListBoxItem<'a, Message>>,
    selection_mode: ListBoxSelectionMode,
    on_selection_change: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    ListBox::new(items)
        .selection_mode(selection_mode)
        .on_selection_change(on_selection_change)
        .into()
}
