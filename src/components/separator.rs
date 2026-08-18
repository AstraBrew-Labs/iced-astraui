#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorVariant {
    #[default]
    Default,
    Secondary,
    Tertiary,
}

impl SeparatorVariant {
    const fn color(self) -> Color {
        match self {
            Self::Default => LINE,
            Self::Secondary => Color::from_rgb8(230, 230, 232),
            Self::Tertiary => Color::from_rgb8(239, 239, 240),
        }
    }
}

/// HeroUI 风格的内容分隔符，默认横向、1px、占满可用长度。
#[derive(Debug, Clone, Copy)]
pub struct Separator {
    orientation: SeparatorOrientation,
    variant: SeparatorVariant,
    thickness: f32,
    fill_mode: rule::FillMode,
}

impl Separator {
    pub const fn new() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            variant: SeparatorVariant::Default,
            thickness: 1.0,
            fill_mode: rule::FillMode::Full,
        }
    }

    pub const fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub const fn variant(mut self, variant: SeparatorVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness.max(1.0);
        self
    }

    pub const fn fill_mode(mut self, fill_mode: rule::FillMode) -> Self {
        self.fill_mode = fill_mode;
        self
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Separator> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(separator: Separator) -> Self {
        let color = separator.variant.color();
        let fill_mode = separator.fill_mode;
        let style = move |_theme: &Theme| rule::Style {
            color,
            radius: 1.0.into(),
            fill_mode,
            snap: true,
        };

        match separator.orientation {
            SeparatorOrientation::Horizontal => {
                rule::horizontal(separator.thickness).style(style).into()
            }
            SeparatorOrientation::Vertical => {
                rule::vertical(separator.thickness).style(style).into()
            }
        }
    }
}

