#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Single,
    Multiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleButtonVariant {
    Default,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupPosition {
    Standalone,
    First,
    Middle,
    Last,
}

fn previous_navigation_index(current: usize, count: usize) -> usize {
    if count == 0 {
        0
    } else if current == 0 {
        count - 1
    } else {
        current - 1
    }
}

fn next_navigation_index(current: usize, count: usize) -> usize {
    if count == 0 { 0 } else { (current + 1) % count }
}

#[derive(Debug, Default)]
struct NavigationState {
    focused: bool,
}

impl iced::advanced::widget::operation::Focusable for NavigationState {
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

struct NavigationGroup<'a, Message> {
    content: Element<'a, Message>,
    id: iced::widget::Id,
    orientation: Orientation,
    item_count: usize,
    focused_index: usize,
    on_focus: Box<dyn Fn(usize) -> Message + 'a>,
    on_activate: Box<dyn Fn(usize) -> Message + 'a>,
    on_remove: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for NavigationGroup<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<NavigationState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(NavigationState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
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
        let state = tree.state.downcast_mut::<NavigationState>();
        operation.focusable(Some(&self.id), layout.bounds(), state);
        self.content
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
        let state = tree.state.downcast_mut::<NavigationState>();

        if matches!(
            event,
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. })
        ) {
            let focused = cursor.is_over(layout.bounds());
            if state.focused != focused {
                state.focused = focused;
                shell.request_redraw();
            }
        }

        if state.focused
            && self.item_count > 0
            && let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event
        {
            use keyboard::key::Named;

            let previous_key = match self.orientation {
                Orientation::Horizontal => Named::ArrowLeft,
                Orientation::Vertical => Named::ArrowUp,
            };
            let next_key = match self.orientation {
                Orientation::Horizontal => Named::ArrowRight,
                Orientation::Vertical => Named::ArrowDown,
            };
            let message = match key.as_ref() {
                keyboard::Key::Named(named) if named == previous_key => Some((self.on_focus)(
                    previous_navigation_index(self.focused_index, self.item_count),
                )),
                keyboard::Key::Named(named) if named == next_key => Some((self.on_focus)(
                    next_navigation_index(self.focused_index, self.item_count),
                )),
                keyboard::Key::Named(Named::Home) => Some((self.on_focus)(0)),
                keyboard::Key::Named(Named::End) => Some((self.on_focus)(self.item_count - 1)),
                keyboard::Key::Named(Named::Enter | Named::Space) => {
                    Some((self.on_activate)(self.focused_index))
                }
                keyboard::Key::Named(Named::Backspace | Named::Delete) => self
                    .on_remove
                    .as_ref()
                    .map(|on_remove| on_remove(self.focused_index)),
                _ => None,
            };

            if let Some(message) = message {
                shell.publish(message);
                shell.capture_event();
                return;
            }
        }

        self.content.as_widget_mut().update(
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
        self.content.as_widget().draw(
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
        self.content.as_widget().mouse_interaction(
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
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn navigation_group<'a, Message>(
    id: iced::widget::Id,
    content: impl Into<Element<'a, Message>>,
    orientation: Orientation,
    item_count: usize,
    focused_index: usize,
    _active: bool,
    on_focus: impl Fn(usize) -> Message + 'a,
    on_activate: impl Fn(usize) -> Message + 'a,
    on_remove: Option<impl Fn(usize) -> Message + 'a>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    Element::new(NavigationGroup {
        content: content.into(),
        id,
        orientation,
        item_count,
        focused_index: focused_index.min(item_count.saturating_sub(1)),
        on_focus: Box::new(on_focus),
        on_activate: Box::new(on_activate),
        on_remove: on_remove.map(|on_remove| Box::new(on_remove) as Box<_>),
    })
}

