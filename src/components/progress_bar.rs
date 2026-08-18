/// HeroUI 风格的确定或不确定进度条。
#[derive(Debug, Clone)]
pub struct ProgressBar {
    value: f32,
    min_value: f32,
    max_value: f32,
    label: Option<String>,
    value_label: Option<String>,
    show_value: bool,
    is_indeterminate: bool,
    animation_phase: f32,
    color: ProgressBarColor,
    size: ProgressBarSize,
    width: Length,
}

impl ProgressBar {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            min_value: 0.0,
            max_value: 100.0,
            label: None,
            value_label: None,
            show_value: true,
            is_indeterminate: false,
            animation_phase: 0.0,
            color: ProgressBarColor::default(),
            size: ProgressBarSize::default(),
            width: Length::Fill,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn value_label(mut self, label: impl Into<String>) -> Self {
        self.value_label = Some(label.into());
        self
    }

    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    pub fn range(mut self, range: std::ops::RangeInclusive<f32>) -> Self {
        (self.min_value, self.max_value) = range.into_inner();
        self
    }

    pub fn is_indeterminate(mut self, is_indeterminate: bool) -> Self {
        self.is_indeterminate = is_indeterminate;
        self
    }

    pub fn animation_phase(mut self, animation_phase: f32) -> Self {
        self.animation_phase = animation_phase;
        self
    }

    pub fn color(mut self, color: ProgressBarColor) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: ProgressBarSize) -> Self {
        self.size = size;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    fn fraction(&self) -> f32 {
        progress_fraction(self.value, self.min_value, self.max_value)
    }

    fn formatted_value(&self) -> String {
        self.value_label
            .clone()
            .unwrap_or_else(|| format!("{:.0}%", self.fraction() * 100.0))
    }
}

struct ProgressBarTrack {
    fraction: f32,
    is_indeterminate: bool,
    animation_phase: f32,
    fill: Color,
    girth: f32,
    width: Length,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for ProgressBarTrack {
    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Fixed(self.girth))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, Length::Fixed(self.girth))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let radius = (bounds.height / 2.0).into();
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius,
                    ..Border::default()
                },
                ..renderer::Quad::default()
            },
            Background::Color(LINE),
        );

        let fill_bounds = if self.is_indeterminate {
            let (offset, width) = indeterminate_segment(bounds.width, self.animation_phase);
            Rectangle {
                x: bounds.x + offset,
                width,
                ..bounds
            }
        } else {
            Rectangle {
                width: bounds.width * self.fraction.clamp(0.0, 1.0),
                ..bounds
            }
        };

        if fill_bounds.width > 0.0 {
            renderer.with_layer(bounds, |renderer| {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: fill_bounds,
                        border: Border {
                            radius,
                            ..Border::default()
                        },
                        ..renderer::Quad::default()
                    },
                    Background::Color(self.fill),
                );
            });
        }
    }
}

impl<'a, Message> From<ProgressBar> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(progress: ProgressBar) -> Self {
        let fraction = progress.fraction();
        let output = progress.formatted_value();
        let track: Element<'a, Message> = Element::new(ProgressBarTrack {
            fraction,
            is_indeterminate: progress.is_indeterminate,
            animation_phase: progress.animation_phase,
            fill: progress.color.fill(),
            girth: progress.size.girth(),
            width: progress.width,
        });
        let mut content = iced::widget::column![].spacing(4).width(progress.width);

        if let Some(label) = progress.label {
            let mut header = row![text(label).size(12).font(crate::fonts::MEDIUM).color(INK)]
                .width(Fill)
                .align_y(iced::Alignment::Center);
            if progress.show_value && !progress.is_indeterminate {
                header = header.push(space::horizontal()).push(
                    text(output)
                        .size(12)
                        .font(crate::fonts::MEDIUM)
                        .color(INK_MUTED),
                );
            }
            content = content.push(header);
        }

        content.push(track).into()
    }
}

fn progress_circle_arc(fraction: f32, is_indeterminate: bool, animation_phase: f32) -> (f32, f32) {
    let phase = if animation_phase.is_finite() {
        animation_phase.rem_euclid(1.0)
    } else {
        0.0
    };
    let start = -std::f32::consts::FRAC_PI_2
        + if is_indeterminate {
            phase * std::f32::consts::TAU
        } else {
            0.0
        };
    let sweep = if is_indeterminate {
        std::f32::consts::TAU * 0.25
    } else {
        std::f32::consts::TAU * fraction.clamp(0.0, 1.0)
    };
    (start, sweep)
}

