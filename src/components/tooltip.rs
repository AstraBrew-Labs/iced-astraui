#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Default)]
struct TooltipState {
    focused: bool,
    hovered: bool,
}

impl iced::advanced::widget::operation::Focusable for TooltipState {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

struct FocusableTooltip<'a, Message> {
    trigger: Element<'a, Message>,
    content: Element<'a, Message>,
    id: iced::widget::Id,
    placement: TooltipPlacement,
    enabled: bool,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for FocusableTooltip<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<TooltipState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(TooltipState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.trigger), Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.trigger.as_widget(), self.content.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.trigger.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.trigger.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.trigger
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        let state = tree.state.downcast_mut::<TooltipState>();
        operation.focusable(Some(&self.id), layout.bounds(), state);
        self.trigger
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<TooltipState>();
        let hovered = self.enabled && cursor.is_over(layout.bounds());
        if state.hovered != hovered {
            state.hovered = hovered;
            shell.invalidate_layout();
            shell.request_redraw();
        }

        if matches!(
            event,
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. })
        ) {
            let focused = self.enabled && cursor.is_over(layout.bounds());
            if state.focused != focused {
                state.focused = focused;
                shell.invalidate_layout();
                shell.request_redraw();
            }
        }

        self.trigger.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.trigger.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.trigger.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut Tree,
        layout: Layout<'a>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, iced::Renderer>> {
        let mut children = tree.children.iter_mut();
        let trigger_overlay = self.trigger.as_widget_mut().overlay(
            children.next().unwrap(),
            layout,
            renderer,
            viewport,
            translation,
        );
        let state = tree.state.downcast_ref::<TooltipState>();
        let tooltip_overlay = (self.enabled && (state.hovered || state.focused)).then(|| {
            overlay::Element::new(Box::new(TooltipOverlay {
                content: &mut self.content,
                tree: children.next().unwrap(),
                target: layout.bounds() + translation,
                placement: self.placement,
            }))
        });

        if trigger_overlay.is_some() || tooltip_overlay.is_some() {
            Some(
                overlay::Group::with_children(
                    trigger_overlay.into_iter().chain(tooltip_overlay).collect(),
                )
                .overlay(),
            )
        } else {
            None
        }
    }
}

struct TooltipOverlay<'a, 'b, Message> {
    content: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    target: Rectangle,
    placement: TooltipPlacement,
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for TooltipOverlay<'_, '_, Message> {
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let node = self.content.as_widget_mut().layout(
            self.tree,
            renderer,
            &layout::Limits::new(Size::ZERO, bounds),
        );
        let size = node.size();
        let gap = 8.0;
        let origin = match self.placement {
            TooltipPlacement::Top => Point::new(
                self.target.center_x() - size.width / 2.0,
                self.target.y - size.height - gap,
            ),
            TooltipPlacement::Bottom => Point::new(
                self.target.center_x() - size.width / 2.0,
                self.target.y + self.target.height + gap,
            ),
            TooltipPlacement::Left => Point::new(
                self.target.x - size.width - gap,
                self.target.center_y() - size.height / 2.0,
            ),
            TooltipPlacement::Right => Point::new(
                self.target.x + self.target.width + gap,
                self.target.center_y() - size.height / 2.0,
            ),
        };
        let margin = 8.0;
        node.move_to(Point::new(
            origin
                .x
                .clamp(margin, (bounds.width - size.width - margin).max(margin)),
            origin
                .y
                .clamp(margin, (bounds.height - size.height - margin).max(margin)),
        ))
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            &layout.bounds(),
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout,
            cursor,
            &layout.bounds(),
            renderer,
        )
    }
}

fn tooltip_surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(INK)),
        text_color: Some(WHITE),
        border: Border {
            radius: 9.0.into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.20),
            offset: Vector::new(0.0, 5.0),
            blur_radius: 16.0,
        },
        ..container::Style::default()
    }
}

pub fn tooltip<'a, Message>(
    id: iced::widget::Id,
    trigger: impl Into<Element<'a, Message>>,
    label: &'a str,
    placement: TooltipPlacement,
    enabled: bool,
) -> Element<'a, Message>
where
    Message: 'a,
{
    Element::new(FocusableTooltip {
        trigger: trigger.into(),
        content: container(
            text(label)
                .size(11)
                .font(crate::fonts::MEDIUM)
                .line_height(iced::widget::text::LineHeight::Absolute(Pixels(16.0))),
        )
        .padding([6, 9])
        .style(tooltip_surface)
        .into(),
        id,
        placement,
        enabled,
    })
}

