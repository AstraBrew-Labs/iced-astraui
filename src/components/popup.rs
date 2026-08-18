#[derive(Debug, Clone, Copy, PartialEq)]
enum PopupPlacement {
    Below,
    Cursor(Point),
}

struct Popup<'a, Message> {
    base: Element<'a, Message>,
    menu: Element<'a, Message>,
    expanded: bool,
    placement: PopupPlacement,
    on_dismiss: Message,
    on_context: Option<Box<dyn Fn(Point) -> Message + 'a>>,
}

impl<'a, Message> Popup<'a, Message> {
    fn dropdown(
        base: Element<'a, Message>,
        menu: Element<'a, Message>,
        expanded: bool,
        on_dismiss: Message,
    ) -> Self {
        Self {
            base,
            menu,
            expanded,
            placement: PopupPlacement::Below,
            on_dismiss,
            on_context: None,
        }
    }

    fn context(
        base: Element<'a, Message>,
        menu: Element<'a, Message>,
        position: Option<Point>,
        on_open: impl Fn(Point) -> Message + 'a,
        on_dismiss: Message,
    ) -> Self {
        Self {
            base,
            menu,
            expanded: position.is_some(),
            placement: PopupPlacement::Cursor(position.unwrap_or(Point::ORIGIN)),
            on_dismiss,
            on_context: Some(Box::new(on_open)),
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Popup<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.base), Tree::new(&self.menu)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.base.as_widget(), self.menu.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.base.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.base.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.base
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
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
        if let (Some(on_context), Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right))) =
            (&self.on_context, event)
            && let Some(position) = cursor.position_over(layout.bounds())
        {
            shell.publish(on_context(position));
            shell.capture_event();
            return;
        }

        self.base.as_widget_mut().update(
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
        self.base.as_widget().draw(
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
        self.base.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        self.base
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
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
        let base_tree = children.next().unwrap();
        let menu_tree = children.next().unwrap();
        let base_overlay =
            self.base
                .as_widget_mut()
                .overlay(base_tree, layout, renderer, viewport, translation);
        let placement = translated_popup_placement(self.placement, translation);
        let popup = self.expanded.then(|| {
            overlay::Element::new(Box::new(PopupOverlay {
                menu: &mut self.menu,
                tree: menu_tree,
                placement,
                target_bounds: layout.bounds() + translation,
                coordinate_translation: translation,
                on_dismiss: self.on_dismiss.clone(),
                on_context: self.on_context.as_deref(),
            }))
        });

        if base_overlay.is_some() || popup.is_some() {
            Some(
                overlay::Group::with_children(base_overlay.into_iter().chain(popup).collect())
                    .overlay(),
            )
        } else {
            None
        }
    }
}

impl<'a, Message> From<Popup<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(popup: Popup<'a, Message>) -> Self {
        Element::new(popup)
    }
}

struct PopupOverlay<'a, 'b, Message> {
    menu: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    placement: PopupPlacement,
    target_bounds: Rectangle,
    coordinate_translation: Vector,
    on_dismiss: Message,
    on_context: Option<&'b dyn Fn(Point) -> Message>,
}

fn translated_popup_placement(placement: PopupPlacement, translation: Vector) -> PopupPlacement {
    match placement {
        PopupPlacement::Below => PopupPlacement::Below,
        PopupPlacement::Cursor(position) => PopupPlacement::Cursor(position + translation),
    }
}

fn popup_origin(
    placement: PopupPlacement,
    target_bounds: Rectangle,
    menu_size: Size,
    viewport_size: Size,
) -> Point {
    let margin = 8.0;
    let (preferred_x, preferred_y) = match placement {
        PopupPlacement::Below => {
            let below = target_bounds.y + target_bounds.height + 6.0;
            let y = if below + menu_size.height <= viewport_size.height - margin {
                below
            } else {
                target_bounds.y - menu_size.height - 6.0
            };
            (target_bounds.x, y)
        }
        PopupPlacement::Cursor(position) => (position.x + 2.0, position.y + 2.0),
    };

    Point::new(
        preferred_x.clamp(
            margin,
            (viewport_size.width - menu_size.width - margin).max(margin),
        ),
        preferred_y.clamp(
            margin,
            (viewport_size.height - menu_size.height - margin).max(margin),
        ),
    )
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for PopupOverlay<'_, '_, Message>
where
    Message: Clone,
{
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let margin = 8.0;
        let menu = self.menu.as_widget_mut().layout(
            self.tree,
            renderer,
            &layout::Limits::new(
                Size::ZERO,
                Size::new(
                    (bounds.width - margin * 2.0).max(0.0),
                    (bounds.height - margin * 2.0).max(0.0),
                ),
            ),
        );
        let menu_size = menu.size();
        let origin = popup_origin(self.placement, self.target_bounds, menu_size, bounds);

        layout::Node::with_children(menu_size, vec![menu]).move_to(origin)
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let menu_layout = layout.children().next().unwrap();
        self.menu.as_widget_mut().update(
            self.tree,
            event,
            menu_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );
        if shell.is_event_captured() {
            return;
        }

        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) = event
            && let Some(on_context) = self.on_context
            && let Some(position) = cursor.position_over(self.target_bounds)
        {
            shell.publish(on_context(position - self.coordinate_translation));
            shell.capture_event();
            return;
        }

        let pointer_pressed = matches!(
            event,
            Event::Mouse(mouse::Event::ButtonPressed(_))
                | Event::Touch(touch::Event::FingerPressed { .. })
        );
        let escape_pressed = matches!(
            event,
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            })
        );
        if escape_pressed || (pointer_pressed && !cursor.is_over(layout.bounds())) {
            shell.publish(self.on_dismiss.clone());
            shell.capture_event();
        }
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.menu.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            &Rectangle::with_size(Size::INFINITE),
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.menu.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().unwrap(),
            cursor,
            &layout.bounds(),
            renderer,
        )
    }
}

fn disclosure_surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: LINE,
            width: 1.0,
            radius: RADIUS_FIELD.into(),
        },
        ..container::Style::default()
    }
}

fn disclosure_panel_surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: LINE,
            width: 1.0,
            radius: iced::border::bottom(RADIUS_FIELD),
        },
        ..container::Style::default()
    }
}

fn disclosure_trigger_style(expanded: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        let background = if pressed {
            Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.12)
        } else if expanded || hovered {
            Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.07)
        } else {
            SURFACE
        };

        button::Style {
            background: Some(Background::Color(background)),
            text_color: INK,
            border: Border {
                radius: if expanded {
                    iced::border::top(RADIUS_FIELD)
                } else {
                    RADIUS_FIELD.into()
                },
                ..Border::default()
            },
            ..button::Style::default()
        }
    }
}

