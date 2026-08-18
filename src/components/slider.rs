/// A HeroUI-style slider with a full background track and an overlaid fill.
///
/// The visual handle and the pointer mapping share the same effective range,
/// keeping endpoint dragging continuous in both directions.
pub fn slider<'a, Message>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    Element::new(HeroSlider {
        range,
        value,
        step: 0.1,
        on_change: Box::new(on_change),
    })
}

struct HeroSlider<'a, Message> {
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    step: f32,
    on_change: Box<dyn Fn(f32) -> Message + 'a>,
}

#[derive(Debug, Default)]
struct HeroSliderState {
    is_dragging: bool,
    grab_offset: f32,
    active_finger: Option<touch::Finger>,
}

impl<Message> HeroSlider<'_, Message> {
    fn progress(&self) -> f32 {
        let start = *self.range.start();
        let end = *self.range.end();

        if end > start {
            ((self.value - start) / (end - start)).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    fn handle_center(&self, bounds: Rectangle) -> f32 {
        let usable_width = (bounds.width - SLIDER_HANDLE_RADIUS * 2.0).max(0.0);
        bounds.x + SLIDER_HANDLE_RADIUS + usable_width * self.progress()
    }

    fn value_at(&self, bounds: Rectangle, pointer_x: f32, grab_offset: f32) -> f32 {
        let start = *self.range.start();
        let end = *self.range.end();
        let left = bounds.x + SLIDER_HANDLE_RADIUS;
        let usable_width = (bounds.width - SLIDER_HANDLE_RADIUS * 2.0).max(1.0);
        let progress = ((pointer_x - grab_offset - left) / usable_width).clamp(0.0, 1.0);
        let raw = start + (end - start) * progress;
        let stepped = ((raw - start) / self.step).round() * self.step + start;

        stepped.clamp(start, end)
    }

    fn publish_value(&mut self, value: f32, shell: &mut Shell<'_, Message>) {
        if (self.value - value).abs() > f32::EPSILON {
            self.value = value;
            shell.publish((self.on_change)(value));
        }
    }

    fn begin_drag(
        &mut self,
        state: &mut HeroSliderState,
        bounds: Rectangle,
        position: Point,
        shell: &mut Shell<'_, Message>,
    ) {
        let handle_center = self.handle_center(bounds);
        state.grab_offset = if (position.x - handle_center).abs() <= SLIDER_HANDLE_RADIUS {
            position.x - handle_center
        } else {
            0.0
        };
        state.is_dragging = true;

        let value = self.value_at(bounds, position.x, state.grab_offset);
        self.publish_value(value, shell);
        shell.capture_event();
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for HeroSlider<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<HeroSliderState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(HeroSliderState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(SLIDER_WIDTH), Length::Fixed(SLIDER_HEIGHT))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(
            limits,
            Length::Fixed(SLIDER_WIDTH),
            Length::Fixed(SLIDER_HEIGHT),
        )
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<HeroSliderState>();
        let bounds = layout.bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(position) = cursor.position_over(bounds) {
                    state.active_finger = None;
                    self.begin_drag(state, bounds, position, shell);
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) if state.is_dragging => {
                let value = self.value_at(bounds, position.x, state.grab_offset);
                self.publish_value(value, shell);
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if state.is_dragging && state.active_finger.is_none() =>
            {
                state.is_dragging = false;
                state.grab_offset = 0.0;
                shell.capture_event();
            }
            Event::Touch(touch::Event::FingerPressed { id, position })
                if bounds.contains(*position) =>
            {
                state.active_finger = Some(*id);
                self.begin_drag(state, bounds, *position, shell);
            }
            Event::Touch(touch::Event::FingerMoved { id, position })
                if state.active_finger == Some(*id) =>
            {
                let value = self.value_at(bounds, position.x, state.grab_offset);
                self.publish_value(value, shell);
                shell.capture_event();
            }
            Event::Touch(touch::Event::FingerLifted { id, .. })
            | Event::Touch(touch::Event::FingerLost { id, .. })
                if state.active_finger == Some(*id) =>
            {
                state.is_dragging = false;
                state.grab_offset = 0.0;
                state.active_finger = None;
                shell.capture_event();
            }
            _ => {}
        }
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
        let progress = self.progress();
        let track_border = Border {
            radius: (SLIDER_HEIGHT / 2.0).into(),
            ..Border::default()
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: track_border,
                ..renderer::Quad::default()
            },
            Background::Color(SURFACE_ALT),
        );

        if progress > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        width: bounds.width * progress,
                        ..bounds
                    },
                    border: track_border,
                    ..renderer::Quad::default()
                },
                Background::Color(BLUE_600),
            );
        }

        let handle_center = self.handle_center(bounds);
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: handle_center - SLIDER_HANDLE_RADIUS,
                    y: bounds.center_y() - SLIDER_HANDLE_RADIUS,
                    width: SLIDER_HANDLE_RADIUS * 2.0,
                    height: SLIDER_HANDLE_RADIUS * 2.0,
                },
                border: Border {
                    radius: SLIDER_HANDLE_RADIUS.into(),
                    ..Border::default()
                },
                ..renderer::Quad::default()
            },
            Background::Color(WHITE),
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<HeroSliderState>();

        if state.is_dragging {
            mouse::Interaction::Grabbing
        } else if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Grab
        } else {
            mouse::Interaction::None
        }
    }
}

fn progress_fraction(value: f32, min_value: f32, max_value: f32) -> f32 {
    if !value.is_finite()
        || !min_value.is_finite()
        || !max_value.is_finite()
        || min_value >= max_value
    {
        0.0
    } else {
        ((value - min_value) / (max_value - min_value)).clamp(0.0, 1.0)
    }
}

fn indeterminate_segment(track_width: f32, phase: f32) -> (f32, f32) {
    let segment_width = track_width.max(0.0) * 0.4;
    let phase = if phase.is_finite() {
        phase.rem_euclid(1.0)
    } else {
        0.0
    };
    let eased = phase * phase * (3.0 - 2.0 * phase);
    (segment_width * (-1.0 + 4.5 * eased), segment_width)
}

