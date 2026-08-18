/// A semantic form label with required, disabled and invalid states.
pub struct Label {
    text: String,
    for_id: Option<String>,
    required: bool,
    disabled: bool,
    invalid: bool,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            for_id: None,
            required: false,
            disabled: false,
            invalid: false,
        }
    }

    /// Associates this label with an application-level control identifier.
    pub fn for_id(mut self, id: impl Into<String>) -> Self {
        self.for_id = Some(id.into());
        self
    }

    pub const fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub const fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn label_id(&self) -> Option<&str> {
        self.for_id.as_deref()
    }

    pub const fn is_required(&self) -> bool {
        self.required
    }

    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub const fn is_invalid(&self) -> bool {
        self.invalid
    }
}

impl<'a, Message> From<Label> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(label: Label) -> Self {
        let mut content = label.text;
        if label.required {
            content.push_str(" *");
        }
        text(content)
            .size(12)
            .font(crate::fonts::MEDIUM)
            .color(if label.invalid {
                DANGER
            } else if label.disabled {
                INK_SUBTLE
            } else {
                INK
            })
            .into()
    }
}

/// Creates a plain form label.
pub fn label<'a, Message>(content: impl Into<String>) -> Element<'a, Message>
where
    Message: 'a,
{
    Label::new(content).into()
}
