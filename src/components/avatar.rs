fn avatar_initial(label: &str) -> String {
    label
        .trim()
        .chars()
        .next()
        .map(|character| character.to_uppercase().collect())
        .filter(|initial: &String| !initial.is_empty())
        .unwrap_or_else(|| "?".to_owned())
}

fn avatar_surface(
    background: Color,
    radius: iced::border::Radius,
) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(background)),
        border: Border {
            radius,
            ..Border::default()
        },
        text_color: Some(readable_on(background)),
        ..container::Style::default()
    }
}

/// A profile image with a single-character fallback and three shape variants.
pub struct Avatar<'a, Message> {
    label: String,
    image: Option<image::Handle>,
    fallback: Option<Element<'a, Message>>,
    shape: AvatarShape,
    size: AvatarSize,
    color: AvatarColor,
}

impl<'a, Message> Avatar<'a, Message>
where
    Message: 'a,
{
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            image: None,
            fallback: None,
            shape: AvatarShape::Circle,
            size: AvatarSize::Medium,
            color: AvatarColor::Accent,
        }
    }

    pub fn image(mut self, handle: impl Into<image::Handle>) -> Self {
        self.image = Some(handle.into());
        self
    }

    pub fn fallback(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.fallback = Some(content.into());
        self
    }

    pub const fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    pub const fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub const fn color(mut self, color: AvatarColor) -> Self {
        self.color = color;
        self
    }
}

impl<'a, Message> From<Avatar<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(avatar: Avatar<'a, Message>) -> Self {
        let diameter = avatar.size.diameter();
        let radius = avatar.shape.radius(diameter);
        let background = avatar.color.background();
        let foreground = readable_on(background);
        let fallback: Element<'a, Message> = avatar.fallback.unwrap_or_else(|| {
            container(
                text(avatar_initial(&avatar.label))
                    .size(if avatar.size == AvatarSize::Large {
                        16
                    } else {
                        14
                    })
                    .font(crate::fonts::MEDIUM)
                    .color(foreground),
            )
            .width(diameter)
            .height(diameter)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .into()
        });
        let fallback = container(fallback)
            .width(diameter)
            .height(diameter)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .style(avatar_surface(background, radius));

        if let Some(handle) = avatar.image {
            container(
                image(handle)
                    .width(diameter)
                    .height(diameter)
                    .content_fit(iced::ContentFit::Cover)
                    .border_radius(radius),
            )
            .width(diameter)
            .height(diameter)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .style(avatar_surface(background, radius))
            .clip(true)
            .into()
        } else {
            fallback.into()
        }
    }
}
