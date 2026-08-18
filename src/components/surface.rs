/// Semantic prominence levels for a [`Surface`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SurfaceVariant {
    /// The standard white application surface.
    #[default]
    Default,
    /// A quiet neutral surface for nested content.
    Secondary,
    /// A lightly tinted surface for emphasis.
    Tertiary,
    /// No background, useful when only spacing and composition are needed.
    Transparent,
}

fn surface_style(variant: SurfaceVariant) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let background = match variant {
            SurfaceVariant::Default => Some(Background::Color(SURFACE)),
            SurfaceVariant::Secondary => Some(Background::Color(SURFACE_ALT)),
            SurfaceVariant::Tertiary => Some(Background::Color(Color::from_rgb8(226, 240, 253))),
            SurfaceVariant::Transparent => None,
        };
        container::Style {
            background,
            border: Border {
                radius: RADIUS_PANEL.into(),
                ..Border::default()
            },
            text_color: Some(INK),
            ..container::Style::default()
        }
    }
}

/// A composable semantic surface that can wrap any iced content.
pub struct Surface<'a, Message> {
    content: Element<'a, Message>,
    variant: SurfaceVariant,
    width: Length,
    padding: u16,
}

impl<'a, Message> Surface<'a, Message>
where
    Message: 'a,
{
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            variant: SurfaceVariant::Default,
            width: Length::Shrink,
            padding: 16,
        }
    }

    pub const fn variant(mut self, variant: SurfaceVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub const fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }
}

impl<'a, Message> From<Surface<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(surface: Surface<'a, Message>) -> Self {
        container(surface.content)
            .width(surface.width)
            .padding(surface.padding)
            .style(surface_style(surface.variant))
            .into()
    }
}

/// Creates a default semantic surface around content.
pub fn surface<'a, Message>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message>
where
    Message: 'a,
{
    Surface::new(content).into()
}
