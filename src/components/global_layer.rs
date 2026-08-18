#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalLayer {
    Modal,
    Toast,
    Message,
}

impl GlobalLayer {
    fn index(self) -> f32 {
        // Iced reserves `f32::MAX` for overlays such as scrollbars. Global
        // surfaces must sit above that plane so the modal backdrop cannot be
        // pierced by framework-owned overlays. Modal/message ordering is
        // handled by their order inside the shared global portal.
        f32::INFINITY
    }
}
/// Promotes content into the application's reserved topmost overlay plane.
/// Global messages render above global modals inside the shared portal.
pub fn global_layer<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    layer: GlobalLayer,
) -> Element<'a, Message>
where
    Message: 'a,
{
    Element::new(GlobalPortal {
        content: content.into(),
        index: layer.index(),
    })
}

struct GlobalPortal<'a, Message> {
    content: Element<'a, Message>,
    index: f32,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for GlobalPortal<'_, Message> {
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

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut iced::Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut Tree,
        layout: Layout<'a>,
        _renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, iced::Renderer>> {
        Some(overlay::Element::new(Box::new(GlobalPortalOverlay {
            content: &mut self.content,
            tree: &mut tree.children[0],
            layout,
            viewport: *viewport,
            translation,
            index: self.index,
        })))
    }
}

struct GlobalPortalOverlay<'a, 'b, Message> {
    content: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    layout: Layout<'b>,
    viewport: Rectangle,
    translation: Vector,
    index: f32,
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer>
    for GlobalPortalOverlay<'_, '_, Message>
{
    fn layout(&mut self, _renderer: &iced::Renderer, _bounds: Size) -> layout::Node {
        let bounds = self.layout.bounds() + self.translation;
        layout::Node::new(bounds.size()).move_to(bounds.position())
    }

    fn update(
        &mut self,
        event: &Event,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        self.content.as_widget_mut().update(
            self.tree,
            event,
            self.layout,
            cursor - self.translation,
            renderer,
            clipboard,
            shell,
            &(self.viewport - self.translation),
        );
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        renderer.with_translation(self.translation, |renderer| {
            self.content.as_widget().draw(
                self.tree,
                renderer,
                theme,
                style,
                self.layout,
                cursor - self.translation,
                &(self.viewport - self.translation),
            );
        });
    }

    fn mouse_interaction(
        &self,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            self.layout,
            cursor - self.translation,
            &(self.viewport - self.translation),
            renderer,
        )
    }

    fn operate(
        &mut self,
        _layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(self.tree, self.layout, renderer, operation);
    }

    fn overlay<'a>(
        &'a mut self,
        _layout: Layout<'a>,
        renderer: &iced::Renderer,
    ) -> Option<overlay::Element<'a, Message, Theme, iced::Renderer>> {
        self.content.as_widget_mut().overlay(
            self.tree,
            self.layout,
            renderer,
            &(self.viewport - self.translation),
            self.translation,
        )
    }

    fn index(&self) -> f32 {
        self.index
    }
}

