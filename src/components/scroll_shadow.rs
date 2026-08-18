#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollShadowOrientation {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollShadowVisibility {
    #[default]
    Auto,
    Both,
    Top,
    Bottom,
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ScrollShadowEdges {
    #[default]
    None,
    Before,
    After,
    Both,
}

impl ScrollShadowEdges {
    const fn before(self) -> bool {
        matches!(self, Self::Before | Self::Both)
    }

    const fn after(self) -> bool {
        matches!(self, Self::After | Self::Both)
    }

    const fn visibility(self, orientation: ScrollShadowOrientation) -> ScrollShadowVisibility {
        match (self, orientation) {
            (Self::None, _) => ScrollShadowVisibility::None,
            (Self::Both, _) => ScrollShadowVisibility::Both,
            (Self::Before, ScrollShadowOrientation::Vertical) => ScrollShadowVisibility::Top,
            (Self::After, ScrollShadowOrientation::Vertical) => ScrollShadowVisibility::Bottom,
            (Self::Before, ScrollShadowOrientation::Horizontal) => ScrollShadowVisibility::Left,
            (Self::After, ScrollShadowOrientation::Horizontal) => ScrollShadowVisibility::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ScrollMetrics {
    bounds: Rectangle,
    content_bounds: Rectangle,
    translation: Vector,
}

fn automatic_scroll_shadow_edges(
    metrics: ScrollMetrics,
    orientation: ScrollShadowOrientation,
    offset: f32,
) -> ScrollShadowEdges {
    let (scroll_start, viewport_size, content_size) = match orientation {
        ScrollShadowOrientation::Vertical => (
            metrics.translation.y,
            metrics.bounds.height,
            metrics.content_bounds.height,
        ),
        ScrollShadowOrientation::Horizontal => (
            metrics.translation.x,
            metrics.bounds.width,
            metrics.content_bounds.width,
        ),
    };
    let offset = offset.max(0.0);
    let has_before = content_size > viewport_size && scroll_start > offset;
    let has_after =
        content_size > viewport_size && scroll_start + viewport_size + offset < content_size - 1.0;

    match (has_before, has_after) {
        (true, true) => ScrollShadowEdges::Both,
        (true, false) => ScrollShadowEdges::Before,
        (false, true) => ScrollShadowEdges::After,
        (false, false) => ScrollShadowEdges::None,
    }
}

fn controlled_scroll_shadow_edges(
    visibility: ScrollShadowVisibility,
    orientation: ScrollShadowOrientation,
) -> Option<ScrollShadowEdges> {
    match visibility {
        ScrollShadowVisibility::Auto => None,
        ScrollShadowVisibility::Both => Some(ScrollShadowEdges::Both),
        ScrollShadowVisibility::Top | ScrollShadowVisibility::Left => {
            Some(ScrollShadowEdges::Before)
        }
        ScrollShadowVisibility::Bottom | ScrollShadowVisibility::Right => {
            Some(ScrollShadowEdges::After)
        }
        ScrollShadowVisibility::None => Some(ScrollShadowEdges::None),
    }
    .map(|edges| match (visibility, orientation) {
        (
            ScrollShadowVisibility::Top | ScrollShadowVisibility::Bottom,
            ScrollShadowOrientation::Horizontal,
        )
        | (
            ScrollShadowVisibility::Left | ScrollShadowVisibility::Right,
            ScrollShadowOrientation::Vertical,
        ) => ScrollShadowEdges::None,
        _ => edges,
    })
}

/// 可滚动内容容器；根据当前位置自动在起点或终点绘制 HeroUI 风格渐隐提示。
pub struct ScrollShadow<'a, Message> {
    content: Element<'a, Message>,
    orientation: ScrollShadowOrientation,
    visibility: ScrollShadowVisibility,
    size: f32,
    offset: f32,
    is_enabled: bool,
    hide_scrollbar: bool,
    width: Length,
    height: Length,
    fade_color: Color,
    on_visibility_change: Option<Box<dyn Fn(ScrollShadowVisibility) -> Message + 'a>>,
}

impl<'a, Message> ScrollShadow<'a, Message>
where
    Message: 'a,
{
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            orientation: ScrollShadowOrientation::Vertical,
            visibility: ScrollShadowVisibility::Auto,
            size: 40.0,
            offset: 0.0,
            is_enabled: true,
            hide_scrollbar: false,
            width: Length::Fill,
            height: Length::Fill,
            fade_color: SURFACE,
            on_visibility_change: None,
        }
    }

    pub const fn orientation(mut self, orientation: ScrollShadowOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub const fn visibility(mut self, visibility: ScrollShadowVisibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(0.0);
        self
    }

    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset.max(0.0);
        self
    }

    pub const fn is_enabled(mut self, is_enabled: bool) -> Self {
        self.is_enabled = is_enabled;
        self
    }

    pub const fn hide_scrollbar(mut self, hide_scrollbar: bool) -> Self {
        self.hide_scrollbar = hide_scrollbar;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub const fn fade_color(mut self, fade_color: Color) -> Self {
        self.fade_color = fade_color;
        self
    }

    pub fn on_visibility_change(
        mut self,
        on_visibility_change: impl Fn(ScrollShadowVisibility) -> Message + 'a,
    ) -> Self {
        self.on_visibility_change = Some(Box::new(on_visibility_change));
        self
    }
}

#[derive(Debug, Default)]
struct ScrollShadowState {
    edges: ScrollShadowEdges,
}

#[derive(Debug, Default)]
struct ReadScrollMetrics {
    metrics: Option<ScrollMetrics>,
}

impl iced::advanced::widget::Operation for ReadScrollMetrics {
    fn traverse(&mut self, _operate: &mut dyn FnMut(&mut dyn iced::advanced::widget::Operation)) {}

    fn scrollable(
        &mut self,
        _id: Option<&iced::widget::Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        translation: Vector,
        _state: &mut dyn iced::advanced::widget::operation::Scrollable,
    ) {
        self.metrics.get_or_insert(ScrollMetrics {
            bounds,
            content_bounds,
            translation,
        });
    }
}

struct ScrollShadowWidget<'a, Message> {
    scrollable: Element<'a, Message>,
    orientation: ScrollShadowOrientation,
    visibility: ScrollShadowVisibility,
    size: f32,
    offset: f32,
    is_enabled: bool,
    hide_scrollbar: bool,
    fade_color: Color,
    on_visibility_change: Option<Box<dyn Fn(ScrollShadowVisibility) -> Message + 'a>>,
}

impl<Message> ScrollShadowWidget<'_, Message> {
    fn metrics(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
    ) -> Option<ScrollMetrics> {
        let mut operation = ReadScrollMetrics::default();
        self.scrollable.as_widget_mut().operate(
            &mut tree.children[0],
            layout,
            renderer,
            &mut operation,
        );
        operation.metrics
    }

    fn edges(&self, metrics: ScrollMetrics) -> ScrollShadowEdges {
        if !self.is_enabled {
            ScrollShadowEdges::None
        } else if let Some(edges) =
            controlled_scroll_shadow_edges(self.visibility, self.orientation)
        {
            edges
        } else {
            automatic_scroll_shadow_edges(metrics, self.orientation, self.offset)
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for ScrollShadowWidget<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ScrollShadowState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ScrollShadowState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.scrollable)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.scrollable));
    }

    fn size(&self) -> Size<Length> {
        self.scrollable.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.scrollable.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let node = self
            .scrollable
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        let metrics = self.metrics(tree, Layout::new(&node), renderer);
        if let Some(metrics) = metrics {
            tree.state.downcast_mut::<ScrollShadowState>().edges = self.edges(metrics);
        }
        node
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        self.scrollable
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
        self.scrollable.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        if let Some(metrics) = self.metrics(tree, layout, renderer) {
            let edges = self.edges(metrics);
            let state = tree.state.downcast_mut::<ScrollShadowState>();
            if state.edges != edges {
                state.edges = edges;
                if let Some(on_visibility_change) = self.on_visibility_change.as_ref() {
                    shell.publish(on_visibility_change(edges.visibility(self.orientation)));
                }
                shell.request_redraw();
            }
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
        self.scrollable.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );

        let bounds = layout.bounds();
        let edges = tree.state.downcast_ref::<ScrollShadowState>().edges;
        let extent = match self.orientation {
            ScrollShadowOrientation::Vertical => self.size.min(bounds.height / 2.0),
            ScrollShadowOrientation::Horizontal => self.size.min(bounds.width / 2.0),
        };
        if extent <= 0.0 || edges == ScrollShadowEdges::None {
            return;
        }

        let opaque = Color {
            a: self.fade_color.a * 0.98,
            ..self.fade_color
        };
        let transparent = Color {
            a: 0.0,
            ..self.fade_color
        };
        renderer.with_layer(bounds, |renderer| {
            let scrollbar_clearance = if self.hide_scrollbar { 0.0 } else { 10.0 };
            if edges.before() {
                let (fade_bounds, angle) = match self.orientation {
                    ScrollShadowOrientation::Vertical => (
                        Rectangle {
                            width: (bounds.width - scrollbar_clearance).max(0.0),
                            height: extent,
                            ..bounds
                        },
                        Radians::PI,
                    ),
                    ScrollShadowOrientation::Horizontal => (
                        Rectangle {
                            width: extent,
                            height: (bounds.height - scrollbar_clearance).max(0.0),
                            ..bounds
                        },
                        Radians::PI / 2.0,
                    ),
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: fade_bounds,
                        ..renderer::Quad::default()
                    },
                    iced::gradient::Linear::new(angle)
                        .add_stop(0.0, opaque)
                        .add_stop(1.0, transparent),
                );
            }
            if edges.after() {
                let (fade_bounds, angle) = match self.orientation {
                    ScrollShadowOrientation::Vertical => (
                        Rectangle {
                            y: bounds.y + bounds.height - extent,
                            width: (bounds.width - scrollbar_clearance).max(0.0),
                            height: extent,
                            ..bounds
                        },
                        Radians(0.0),
                    ),
                    ScrollShadowOrientation::Horizontal => (
                        Rectangle {
                            x: bounds.x + bounds.width - extent,
                            width: extent,
                            height: (bounds.height - scrollbar_clearance).max(0.0),
                            ..bounds
                        },
                        Radians(std::f32::consts::PI * 1.5),
                    ),
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: fade_bounds,
                        ..renderer::Quad::default()
                    },
                    iced::gradient::Linear::new(angle)
                        .add_stop(0.0, opaque)
                        .add_stop(1.0, transparent),
                );
            }
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
        self.scrollable.as_widget().mouse_interaction(
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
        self.scrollable.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message> From<ScrollShadow<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(scroll_shadow: ScrollShadow<'a, Message>) -> Self {
        let scrollbar = if scroll_shadow.hide_scrollbar {
            scrollable::Scrollbar::hidden()
        } else {
            scrollable::Scrollbar::default()
        };
        let direction = match scroll_shadow.orientation {
            ScrollShadowOrientation::Vertical => scrollable::Direction::Vertical(scrollbar),
            ScrollShadowOrientation::Horizontal => scrollable::Direction::Horizontal(scrollbar),
        };
        let scrollable: Element<'a, Message> = scrollable(scroll_shadow.content)
            .direction(direction)
            .width(scroll_shadow.width)
            .height(scroll_shadow.height)
            .into();

        Element::new(ScrollShadowWidget {
            scrollable,
            orientation: scroll_shadow.orientation,
            visibility: scroll_shadow.visibility,
            size: scroll_shadow.size,
            offset: scroll_shadow.offset,
            is_enabled: scroll_shadow.is_enabled,
            hide_scrollbar: scroll_shadow.hide_scrollbar,
            fade_color: scroll_shadow.fade_color,
            on_visibility_change: scroll_shadow.on_visibility_change,
        })
    }
}

