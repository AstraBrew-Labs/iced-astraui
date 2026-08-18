/// Visual treatment for a [`TextArea`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAreaVariant {
    #[default]
    Primary,
    Secondary,
}

fn text_area_style(
    variant: TextAreaVariant,
) -> impl Fn(&Theme, text_editor::Status) -> text_editor::Style {
    move |_theme, status| {
        let focused = matches!(status, text_editor::Status::Focused { .. });
        let hovered = matches!(
            status,
            text_editor::Status::Hovered
                | text_editor::Status::Focused { is_hovered: true }
        );
        let disabled = matches!(status, text_editor::Status::Disabled);
        text_editor::Style {
            background: Background::Color(if disabled {
                Color::from_rgba(SURFACE_ALT.r, SURFACE_ALT.g, SURFACE_ALT.b, 0.5)
            } else if variant == TextAreaVariant::Secondary {
                SURFACE_ALT
            } else {
                SURFACE
            }),
            border: Border {
                color: if disabled {
                    Color::from_rgba(LINE.r, LINE.g, LINE.b, 0.55)
                } else if focused {
                    BLUE_600
                } else if hovered {
                    BLUE_500
                } else {
                    LINE
                },
                width: if focused { 2.0 } else { 1.0 },
                radius: RADIUS_FIELD.into(),
            },
            placeholder: INK_SUBTLE,
            value: if disabled { INK_SUBTLE } else { INK },
            selection: Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.20),
        }
    }
}

/// A controlled multi-line editor backed by iced's `text_editor` widget.
pub struct TextArea<'a, Message> {
    content: &'a text_editor::Content,
    placeholder: String,
    rows: u16,
    width: Length,
    variant: TextAreaVariant,
    disabled: bool,
    on_action: Option<Box<dyn Fn(text_editor::Action) -> Message + 'a>>,
}

impl<'a, Message> TextArea<'a, Message>
where
    Message: 'a,
{
    pub fn new(content: &'a text_editor::Content) -> Self {
        Self {
            content,
            placeholder: String::new(),
            rows: 3,
            width: Fill,
            variant: TextAreaVariant::Primary,
            disabled: false,
            on_action: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub const fn rows(mut self, rows: u16) -> Self {
        self.rows = if rows == 0 { 1 } else { rows };
        self
    }

    pub const fn variant(mut self, variant: TextAreaVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn on_action(
        mut self,
        on_action: impl Fn(text_editor::Action) -> Message + 'a,
    ) -> Self {
        self.on_action = Some(Box::new(on_action));
        self
    }
}

impl<'a, Message> From<TextArea<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(area: TextArea<'a, Message>) -> Self {
        let height = Pixels(24.0 * f32::from(area.rows) + 24.0);
        let mut editor = text_editor(area.content)
            .placeholder(area.placeholder)
            .height(height)
            .padding([10, 12])
            .font(crate::fonts::REGULAR)
            .size(13)
            .line_height(iced::widget::text::LineHeight::Absolute(Pixels(20.0)))
            .style(text_area_style(area.variant));
        if !area.disabled {
            if let Some(on_action) = area.on_action {
                editor = editor.on_action(on_action);
            }
        }
        container(editor).width(area.width).into()
    }
}

/// Convenience constructor for a controlled text area.
pub fn text_area<'a, Message>(
    content: &'a text_editor::Content,
    placeholder: impl Into<String>,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    TextArea::new(content)
        .placeholder(placeholder)
        .on_action(on_action)
        .into()
}
