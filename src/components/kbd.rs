/// Platform-specific keyboard glyphs used by [`Kbd`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KbdPlatform {
    /// macOS modifier labels such as Command and Option.
    #[default]
    Mac,
    /// Windows modifier labels such as Ctrl, Alt and Win.
    Win,
}

/// Visual treatment for a keyboard shortcut.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KbdVariant {
    /// Raised keycap with a visible border.
    #[default]
    Default,
    /// Subtle keycap for use inline with supporting text.
    Light,
}

/// A modifier, special key, navigation key, or custom key label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KbdKey {
    Command,
    Control,
    Option,
    Alt,
    Shift,
    Win,
    Fn,
    Enter,
    Backspace,
    Delete,
    Escape,
    Tab,
    Space,
    CapsLock,
    Help,
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    /// A literal key label, such as `K`, `F1`, or `?`.
    Character(String),
}

impl KbdKey {
    /// Creates a literal key label.
    pub fn character(value: impl Into<String>) -> Self {
        Self::Character(value.into())
    }

    /// Alias for [`KbdKey::character`] that reads naturally in shortcut declarations.
    pub fn content(value: impl Into<String>) -> Self {
        Self::character(value)
    }

    fn display(&self, platform: KbdPlatform) -> String {
        let value = match (self, platform) {
            (Self::Command, KbdPlatform::Mac) => "⌘",
            (Self::Command | Self::Control, KbdPlatform::Win) => "Ctrl",
            (Self::Control, KbdPlatform::Mac) => "⌃",
            (Self::Option, KbdPlatform::Mac) | (Self::Alt, KbdPlatform::Mac) => "⌥",
            (Self::Option | Self::Alt, KbdPlatform::Win) => "Alt",
            (Self::Shift, KbdPlatform::Mac) => "⇧",
            (Self::Shift, KbdPlatform::Win) => "Shift",
            (Self::Win, _) => "Win",
            (Self::Fn, _) => "fn",
            (Self::Enter, KbdPlatform::Mac) => "↵",
            (Self::Enter, KbdPlatform::Win) => "Enter",
            (Self::Backspace, KbdPlatform::Mac) => "⌫",
            (Self::Backspace, KbdPlatform::Win) => "Backspace",
            (Self::Delete, _) => "Delete",
            (Self::Escape, _) => "Esc",
            (Self::Tab, KbdPlatform::Mac) => "⇥",
            (Self::Tab, KbdPlatform::Win) => "Tab",
            (Self::Space, _) => "Space",
            (Self::CapsLock, KbdPlatform::Mac) => "⇪",
            (Self::CapsLock, KbdPlatform::Win) => "Caps Lock",
            (Self::Help, _) => "Help",
            (Self::Up, _) => "↑",
            (Self::Down, _) => "↓",
            (Self::Left, _) => "←",
            (Self::Right, _) => "→",
            (Self::PageUp, _) => "PgUp",
            (Self::PageDown, _) => "PgDn",
            (Self::Home, _) => "Home",
            (Self::End, _) => "End",
            (Self::Character(value), _) => return value.clone(),
        };
        value.to_owned()
    }
}

fn kbd_style(variant: KbdVariant) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let (background, border_color, border_width, shadow, text_color) = match variant {
            KbdVariant::Default => (
                Some(Background::Color(SURFACE_ALT)),
                LINE,
                1.0,
                Shadow::default(),
                INK_MUTED,
            ),
            KbdVariant::Light => (None, Color::TRANSPARENT, 0.0, Shadow::default(), INK_MUTED),
        };
        container::Style {
            background,
            border: Border {
                color: border_color,
                width: border_width,
                radius: 6.0.into(),
            },
            shadow,
            text_color: Some(text_color),
            ..container::Style::default()
        }
    }
}

/// A compact keyboard shortcut composed of one or more keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kbd {
    keys: Vec<KbdKey>,
    platform: KbdPlatform,
    variant: KbdVariant,
}

impl Kbd {
    /// Creates a shortcut using macOS labels by default.
    pub fn new(keys: impl IntoIterator<Item = KbdKey>) -> Self {
        Self {
            keys: keys.into_iter().collect(),
            platform: KbdPlatform::Mac,
            variant: KbdVariant::Default,
        }
    }

    /// Creates a shortcut with macOS modifier labels.
    pub fn mac(keys: impl IntoIterator<Item = KbdKey>) -> Self {
        Self::new(keys).platform(KbdPlatform::Mac)
    }

    /// Creates a shortcut with Windows modifier labels.
    pub fn win(keys: impl IntoIterator<Item = KbdKey>) -> Self {
        Self::new(keys).platform(KbdPlatform::Win)
    }

    pub const fn platform(mut self, platform: KbdPlatform) -> Self {
        self.platform = platform;
        self
    }

    pub const fn variant(mut self, variant: KbdVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn push(mut self, key: KbdKey) -> Self {
        self.keys.push(key);
        self
    }

    pub fn keys(&self) -> &[KbdKey] {
        &self.keys
    }

    pub const fn selected_platform(&self) -> KbdPlatform {
        self.platform
    }

    pub const fn selected_variant(&self) -> KbdVariant {
        self.variant
    }
}

impl<'a, Message> From<Kbd> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(kbd: Kbd) -> Self {
        let labels = kbd
            .keys
            .iter()
            .map(|key| {
                text(key.display(kbd.platform))
                    .size(11)
                    .font(crate::fonts::MEDIUM)
                    .line_height(iced::widget::text::LineHeight::Absolute(Pixels(16.0)))
                    .color(INK)
                    .into()
            })
            .collect::<Vec<Element<'a, Message>>>();

        container(row(labels).spacing(4).align_y(iced::Alignment::Center))
            .height(24)
            .padding([3, 7])
            .style(kbd_style(kbd.variant))
            .into()
    }
}

/// Creates a default macOS keyboard shortcut.
pub fn kbd<'a, Message>(keys: impl IntoIterator<Item = KbdKey>) -> Element<'a, Message>
where
    Message: 'a,
{
    Kbd::new(keys).into()
}
