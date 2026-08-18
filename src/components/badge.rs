struct Badge<'a, Message> {
    anchor: Element<'a, Message>,
    indicator: Element<'a, Message>,
    position: BadgePosition,
    extent: f32,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Badge<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.anchor), Tree::new(&self.indicator)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.anchor.as_widget(), self.indicator.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.anchor.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.anchor.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let anchor = self
            .anchor
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        let size = anchor.size();
        let indicator =
            self.indicator
                .as_widget_mut()
                .layout(&mut tree.children[1], renderer, &limits.loose());
        let indicator_size = indicator.size();
        let offset = badge_offset(self.position, self.extent);
        let origin = match self.position {
            BadgePosition::TopRight => {
                Point::new(size.width - indicator_size.width + offset.x, offset.y)
            }
            BadgePosition::TopLeft => Point::new(offset.x, offset.y),
            BadgePosition::BottomRight => Point::new(
                size.width - indicator_size.width + offset.x,
                size.height - indicator_size.height + offset.y,
            ),
            BadgePosition::BottomLeft => {
                Point::new(offset.x, size.height - indicator_size.height + offset.y)
            }
        };

        layout::Node::with_children(size, vec![anchor, indicator.move_to(origin)])
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
        if let Some(anchor_layout) = layout.children().next() {
            self.anchor.as_widget_mut().update(
                &mut tree.children[0],
                event,
                anchor_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }
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
        let mut children = layout.children();
        let Some(anchor_layout) = children.next() else {
            return;
        };
        let Some(indicator_layout) = children.next() else {
            return;
        };

        self.anchor.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            anchor_layout,
            cursor,
            viewport,
        );
        renderer.with_layer(*viewport, |renderer| {
            self.indicator.as_widget().draw(
                &tree.children[1],
                renderer,
                theme,
                style,
                indicator_layout,
                mouse::Cursor::Unavailable,
                viewport,
            );
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        layout
            .children()
            .next()
            .map(|anchor_layout| {
                self.anchor.as_widget().mouse_interaction(
                    &tree.children[0],
                    anchor_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            })
            .unwrap_or_default()
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            for ((child, tree), layout) in [&mut self.anchor, &mut self.indicator]
                .into_iter()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                child
                    .as_widget_mut()
                    .operate(tree, layout, renderer, operation);
            }
        });
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut Tree,
        layout: Layout<'a>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, iced::Renderer>> {
        self.anchor.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next()?,
            renderer,
            viewport,
            translation,
        )
    }
}

/// Places a status dot, count, or short label over one corner of an anchor.
/// The badge extends outside the anchor without creating a global overlay.
pub fn badge<'a, Message>(
    anchor: impl Into<Element<'a, Message>>,
    content: BadgeContent<'a>,
    color: Color,
    position: BadgePosition,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let (indicator, extent): (Element<'a, Message>, f32) = match content {
        BadgeContent::Dot => (
            container(space::Space::new())
                .width(10)
                .height(10)
                .style(badge_surface(color))
                .into(),
            10.0,
        ),
        BadgeContent::Count(count) => {
            let label = if count > 99 {
                "99+".to_owned()
            } else {
                count.to_string()
            };
            let width = if label.len() == 1 {
                Length::Fixed(20.0)
            } else {
                Length::Shrink
            };

            (
                container(
                    text(label)
                        .size(10)
                        .font(crate::fonts::BOLD)
                        .line_height(iced::widget::text::LineHeight::Absolute(Pixels(16.0))),
                )
                .width(width)
                .height(20)
                .padding([0, 5])
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center)
                .style(badge_surface(color))
                .into(),
                20.0,
            )
        }
        BadgeContent::Label(label) => (
            container(
                text(label)
                    .size(9)
                    .font(crate::fonts::BOLD)
                    .line_height(iced::widget::text::LineHeight::Absolute(Pixels(16.0))),
            )
            .height(20)
            .padding([0, 7])
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .style(badge_surface(color))
            .into(),
            20.0,
        ),
    };
    Element::new(Badge {
        anchor: anchor.into(),
        indicator,
        position,
        extent,
    })
}

