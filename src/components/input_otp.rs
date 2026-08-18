use std::rc::Rc;

/// Visual treatment for [`InputOtp`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputOtpVariant {
    #[default]
    Primary,
    Secondary,
}

/// Acronym alias matching the component name used in HeroUI documentation.
pub type InputOTPVariant = InputOtpVariant;

/// The kind of edit that produced an [`InputOtpChange`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputOtpAction {
    Input,
    Backspace,
}

/// Computes the slot that should receive focus after an edit.
///
/// The next slot is based on the configured OTP length rather than the
/// currently populated value, so entering the first character advances to
/// the second slot even while the remaining slots are empty.
pub(crate) const fn input_otp_focus_index(
    index: usize,
    input_length: usize,
    max_length: usize,
    action: InputOtpAction,
) -> usize {
    let last_index = max_length.saturating_sub(1);
    match action {
        InputOtpAction::Input => {
            let next_index = index.saturating_add(input_length);
            if next_index < last_index {
                next_index
            } else {
                last_index
            }
        }
        InputOtpAction::Backspace => index.saturating_sub(1),
    }
}

/// Controlled OTP update information, including the slot to focus next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputOtpChange {
    pub value: String,
    pub index: usize,
    pub action: InputOtpAction,
    pub focus_index: usize,
    pub focus_id: String,
}

fn input_otp_slot_style(
    variant: InputOtpVariant,
    filled: bool,
    disabled: bool,
) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_theme, status| {
        let focused = matches!(status, text_input::Status::Focused { .. });
        let hovered = matches!(
            status,
            text_input::Status::Hovered | text_input::Status::Focused { is_hovered: true }
        );
        text_input::Style {
            background: Background::Color(if disabled {
                Color::from_rgba(SURFACE_ALT.r, SURFACE_ALT.g, SURFACE_ALT.b, 0.5)
            } else if variant == InputOtpVariant::Secondary {
                SURFACE_ALT
            } else {
                SURFACE
            }),
            border: Border {
                color: if disabled {
                    Color::from_rgba(LINE.r, LINE.g, LINE.b, 0.55)
                } else if focused {
                    BLUE_600
                } else if hovered || filled {
                    BLUE_500
                } else {
                    LINE
                },
                width: if focused { 2.0 } else { 1.0 },
                radius: RADIUS_FIELD.into(),
            },
            icon: BLUE_600,
            placeholder: INK_SUBTLE,
            value: if disabled { INK_SUBTLE } else { INK },
            selection: Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.20),
        }
    }
}

/// A controlled one-time-password input composed of one-character slots.
pub struct InputOtp<'a, Message> {
    value: String,
    max_length: usize,
    placeholder: String,
    variant: InputOtpVariant,
    disabled: bool,
    separator_after: Option<usize>,
    id_prefix: String,
    on_change: Rc<dyn Fn(InputOtpChange) -> Message + 'a>,
}

/// HeroUI-compatible acronym alias.
pub type InputOTP<'a, Message> = InputOtp<'a, Message>;

impl<'a, Message> InputOtp<'a, Message>
where
    Message: 'a,
{
    pub fn new(
        value: impl Into<String>,
        max_length: usize,
        on_change: impl Fn(InputOtpChange) -> Message + 'a,
    ) -> Self {
        Self {
            value: value.into(),
            max_length: max_length.max(1),
            placeholder: String::new(),
            variant: InputOtpVariant::Primary,
            disabled: false,
            separator_after: None,
            id_prefix: "input-otp".to_owned(),
            on_change: Rc::new(on_change),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub const fn variant(mut self, variant: InputOtpVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Adds a separator after the zero-based slot index.
    pub const fn separator_after(mut self, index: usize) -> Self {
        self.separator_after = Some(index);
        self
    }

    pub fn id_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub const fn max_length(&self) -> usize {
        self.max_length
    }
}

impl<'a, Message> From<InputOtp<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(otp: InputOtp<'a, Message>) -> Self {
        let mut characters = vec!['\0'; otp.max_length];
        for (index, character) in otp.value.chars().take(otp.max_length).enumerate() {
            characters[index] = character;
        }
        let mut slots = Vec::with_capacity(otp.max_length * 2);

        for index in 0..otp.max_length {
            let slot_character = characters[index];
            let slot_value = if slot_character == '\0' {
                String::new()
            } else {
                slot_character.to_string()
            };
            let filled = !slot_value.is_empty();
            let placeholder = otp.placeholder.clone();
            let focus_id = format!("{}-{}", otp.id_prefix, index);
            let mut slot = text_input(&placeholder, &slot_value)
                .id(focus_id.clone())
                .width(40)
                .padding([8, 0])
                .size(16)
                .align_x(iced::alignment::Horizontal::Center)
                .style(input_otp_slot_style(otp.variant, filled, otp.disabled));

            if !otp.disabled {
                let callback = Rc::clone(&otp.on_change);
                let initial = characters.clone();
                let id_prefix = otp.id_prefix.clone();
                slot = slot.on_input(move |input| {
                    let mut current = initial.clone();
                    let action = if input.is_empty() {
                        InputOtpAction::Backspace
                    } else {
                        InputOtpAction::Input
                    };
                    let input_length = input.chars().count();
                    for (offset, character) in input
                        .chars()
                        .take(current.len().saturating_sub(index))
                        .enumerate()
                    {
                        current[index + offset] = character;
                    }
                    if input.is_empty() {
                        current[index] = '\0';
                    }
                    while current.last().copied() == Some('\0') {
                        current.pop();
                    }
                    let value = current
                            .iter()
                            .filter(|character| **character != '\0')
                            .collect();
                    let focus_index = input_otp_focus_index(
                        index,
                        input_length,
                        initial.len(),
                        action,
                    );
                    callback(InputOtpChange {
                        value,
                        index,
                        action,
                        focus_index,
                        focus_id: format!("{}-{}", id_prefix, focus_index),
                    })
                });
            }
            slots.push(
                container(slot)
                    .width(40)
                    .height(40)
                    .align_y(iced::Alignment::Center)
                    .into(),
            );

            if otp.separator_after == Some(index) && index + 1 < otp.max_length {
                slots.push(
                    container(space::horizontal().width(8))
                        .height(40)
                        .align_y(iced::Alignment::Center)
                        .into(),
                );
            }
        }

        container(row(slots).spacing(6).align_y(iced::Alignment::Center)).into()
    }
}

/// Convenience constructor for a controlled OTP input.
pub fn input_otp<'a, Message>(
    value: impl Into<String>,
    max_length: usize,
    on_change: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    InputOtp::new(value, max_length, move |change| on_change(change.value)).into()
}
