/// A composable HeroUI-style card with optional header and footer sections.
pub struct Card<'a, Message> {
    header: Option<Element<'a, Message>>,
    content: Element<'a, Message>,
    footer: Option<Element<'a, Message>>,
    variant: CardVariant,
    width: Length,
    padding: u16,
    gap: u32,
}

impl<'a, Message> Card<'a, Message>
where
    Message: 'a,
{
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            header: None,
            content: content.into(),
            footer: None,
            variant: CardVariant::Default,
            width: Length::Shrink,
            padding: 16,
            gap: 12,
        }
    }

    pub fn header(mut self, header: impl Into<Element<'a, Message>>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub fn gap(mut self, gap: u32) -> Self {
        self.gap = gap;
        self
    }
}

impl<'a, Message> From<Card<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(card: Card<'a, Message>) -> Self {
        let mut sections = iced::widget::column![].spacing(card.gap);
        if let Some(header) = card.header {
            sections = sections.push(header);
        }
        sections = sections.push(card.content);
        if let Some(footer) = card.footer {
            sections = sections.push(footer);
        }

        container(sections)
            .width(card.width)
            .padding(card.padding)
            .style(card_variant_style(card.variant))
            .into()
    }
}

