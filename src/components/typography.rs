#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypographyType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    #[default]
    Body,
    BodySmall,
    BodyExtraSmall,
    Code,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypographyAlign {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypographyColor {
    #[default]
    Default,
    Muted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypographyWeight {
    Normal,
    Medium,
    Semibold,
    Bold,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TypographyMetrics {
    size: f32,
    line_height: f32,
    font: Font,
}

impl TypographyType {
    const fn metrics(self) -> TypographyMetrics {
        match self {
            Self::H1 => TypographyMetrics {
                size: 36.0,
                line_height: 40.0,
                font: crate::fonts::BOLD,
            },
            Self::H2 => TypographyMetrics {
                size: 30.0,
                line_height: 35.0,
                font: crate::fonts::BOLD,
            },
            Self::H3 => TypographyMetrics {
                size: 24.0,
                line_height: 30.0,
                font: crate::fonts::BOLD,
            },
            Self::H4 => TypographyMetrics {
                size: 20.0,
                line_height: 27.0,
                font: crate::fonts::BOLD,
            },
            Self::H5 => TypographyMetrics {
                size: 18.0,
                line_height: 25.0,
                font: crate::fonts::BOLD,
            },
            Self::H6 => TypographyMetrics {
                size: 16.0,
                line_height: 24.0,
                font: crate::fonts::BOLD,
            },
            Self::Body => TypographyMetrics {
                size: 16.0,
                line_height: 28.0,
                font: crate::fonts::REGULAR,
            },
            Self::BodySmall => TypographyMetrics {
                size: 14.0,
                line_height: 21.0,
                font: crate::fonts::REGULAR,
            },
            Self::BodyExtraSmall => TypographyMetrics {
                size: 12.0,
                line_height: 15.0,
                font: crate::fonts::REGULAR,
            },
            Self::Code => TypographyMetrics {
                size: 14.0,
                line_height: 20.0,
                font: crate::fonts::MEDIUM,
            },
        }
    }
}

/// 标题、正文和行内代码共用的语义化排版原语。
#[derive(Debug, Clone)]
pub struct Typography {
    content: String,
    kind: TypographyType,
    align: TypographyAlign,
    color: TypographyColor,
    weight: Option<TypographyWeight>,
    truncate: bool,
    width: Length,
}

#[derive(Debug, Clone)]
pub struct CopyableTypography<Message> {
    typography: Typography,
    on_copy: Message,
}

impl Typography {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            kind: TypographyType::Body,
            align: TypographyAlign::Start,
            color: TypographyColor::Default,
            weight: None,
            truncate: false,
            width: Length::Shrink,
        }
    }

    pub fn heading(level: u8, content: impl Into<String>) -> Self {
        Self::new(content).kind(match level.clamp(1, 6) {
            1 => TypographyType::H1,
            2 => TypographyType::H2,
            3 => TypographyType::H3,
            4 => TypographyType::H4,
            5 => TypographyType::H5,
            _ => TypographyType::H6,
        })
    }

    pub fn paragraph(content: impl Into<String>) -> Self {
        Self::new(content)
    }

    pub fn code(content: impl Into<String>) -> Self {
        Self::new(content).kind(TypographyType::Code)
    }

    pub fn on_copy<Message>(self, message: Message) -> CopyableTypography<Message> {
        CopyableTypography {
            typography: self,
            on_copy: message,
        }
    }

    pub const fn kind(mut self, kind: TypographyType) -> Self {
        self.kind = kind;
        self
    }

    pub const fn align(mut self, align: TypographyAlign) -> Self {
        self.align = align;
        self
    }

    pub const fn color(mut self, color: TypographyColor) -> Self {
        self.color = color;
        self
    }

    pub const fn weight(mut self, weight: TypographyWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    pub const fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

fn typography_font(default: Font, weight: Option<TypographyWeight>) -> Font {
    match weight {
        None => default,
        Some(TypographyWeight::Normal) => crate::fonts::REGULAR,
        Some(TypographyWeight::Medium) => crate::fonts::MEDIUM,
        Some(TypographyWeight::Semibold | TypographyWeight::Bold) => crate::fonts::BOLD,
    }
}

fn translated_typography_context_position(position: Point, translation: Vector) -> Point {
    position + translation
}

fn typography_context_menu_text_layout(bounds: Rectangle) -> (Size, Point) {
    (
        Size::new((bounds.width - 24.0).max(0.0), bounds.height),
        Point::new(bounds.x + 12.0, bounds.center_y()),
    )
}

type TypographyParagraph = <iced::Renderer as iced::advanced::text::Renderer>::Paragraph;

#[derive(Debug, Default)]
struct SelectableTypographyState {
    paragraph: TypographyParagraph,
    anchor: usize,
    focus: usize,
    dragging: bool,
    focused: bool,
    modifiers: keyboard::Modifiers,
    context_menu: Option<Point>,
}

impl SelectableTypographyState {
    fn selection(&self) -> Option<std::ops::Range<usize>> {
        let start = self.anchor.min(self.focus);
        let end = self.anchor.max(self.focus);
        (start < end).then_some(start..end)
    }

    fn begin_selection(&mut self, index: usize) {
        self.anchor = index;
        self.focus = index;
        self.dragging = true;
        self.focused = true;
        self.context_menu = None;
    }

    fn extend_selection(&mut self, index: usize) -> bool {
        if self.focus == index {
            false
        } else {
            self.focus = index;
            true
        }
    }
}

struct SelectableTypography<'a, Message> {
    content: String,
    metrics: TypographyMetrics,
    color: Color,
    align: iced::advanced::text::Alignment,
    wrapping: iced::advanced::text::Wrapping,
    width: Length,
    on_copy: Option<Box<dyn Fn() -> Message + 'a>>,
}

impl<Message> SelectableTypography<'_, Message> {
    fn char_boundary_at_or_before(&self, index: usize) -> usize {
        let mut index = index.min(self.content.len());
        while !self.content.is_char_boundary(index) {
            index = index.saturating_sub(1);
        }
        index
    }

    fn logical_line_start(&self, line: usize) -> usize {
        self.content
            .split_inclusive('\n')
            .take(line)
            .map(str::len)
            .sum()
    }

    fn clamp_selection(&self, state: &mut SelectableTypographyState) {
        state.anchor = self.char_boundary_at_or_before(state.anchor);
        state.focus = self.char_boundary_at_or_before(state.focus);
    }

    fn selected_text<'a>(&'a self, state: &SelectableTypographyState) -> Option<&'a str> {
        state
            .selection()
            .and_then(|selection| self.content.get(selection))
    }

    fn span<'a>(&self, content: &'a str) -> iced::advanced::text::Span<'a, (), Font> {
        iced::advanced::text::Span::new(content)
            .size(self.metrics.size)
            .line_height(iced::advanced::text::LineHeight::Absolute(Pixels(
                self.metrics.line_height,
            )))
            .font(self.metrics.font)
            .color(self.color)
    }

    fn paragraph(
        &self,
        bounds: Size,
        selection: Option<std::ops::Range<usize>>,
    ) -> TypographyParagraph {
        let selection = selection.map(|selection| {
            self.char_boundary_at_or_before(selection.start)
                ..self.char_boundary_at_or_before(selection.end)
        });
        let mut spans: Vec<iced::advanced::text::Span<'_, (), Font>> = Vec::with_capacity(3);
        if let Some(selection) = selection.filter(|selection| selection.start < selection.end) {
            if selection.start > 0 {
                spans.push(self.span(&self.content[..selection.start]));
            }
            spans.push(
                self.span(&self.content[selection.clone()])
                    .background(Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.24)),
            );
            if selection.end < self.content.len() {
                spans.push(self.span(&self.content[selection.end..]));
            }
        } else {
            spans.push(self.span(&self.content));
        }

        TypographyParagraph::with_spans(iced::advanced::text::Text {
            content: spans.as_slice(),
            bounds,
            size: Pixels(self.metrics.size),
            line_height: iced::advanced::text::LineHeight::Absolute(Pixels(
                self.metrics.line_height,
            )),
            font: self.metrics.font,
            align_x: self.align,
            align_y: iced::alignment::Vertical::Top,
            shaping: iced::advanced::text::Shaping::Auto,
            wrapping: self.wrapping,
        })
    }

    fn rebuild(&self, state: &mut SelectableTypographyState, bounds: Size) {
        self.clamp_selection(state);
        state.paragraph = self.paragraph(bounds, state.selection());
    }

    fn text_anchor(&self, paragraph: &TypographyParagraph, bounds: Rectangle) -> Point {
        bounds.anchor(
            paragraph.min_bounds(),
            paragraph.align_x(),
            paragraph.align_y(),
        )
    }

    fn hit_index(
        &self,
        paragraph: &TypographyParagraph,
        bounds: Rectangle,
        position: Point,
    ) -> usize {
        let anchor = self.text_anchor(paragraph, bounds);
        let local = Point::new(position.x - anchor.x, position.y - anchor.y);
        paragraph
            .buffer()
            .hit(local.x, local.y)
            .map(|cursor| self.logical_line_start(cursor.line) + cursor.index)
            .map(|index| self.char_boundary_at_or_before(index))
            .unwrap_or_else(|| {
                if local.x <= 0.0 && local.y <= 0.0 {
                    0
                } else {
                    self.content.len()
                }
            })
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for SelectableTypography<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<SelectableTypographyState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(SelectableTypographyState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree.state.downcast_mut::<SelectableTypographyState>();
        layout::sized(limits, self.width, Length::Shrink, |limits| {
            self.rebuild(state, limits.max());
            state.paragraph.min_bounds()
        })
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<SelectableTypographyState>();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                if let Some(position) = cursor.position_over(layout.bounds())
                    && self.selected_text(state).is_some()
                {
                    state.context_menu = Some(position);
                    state.focused = true;
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                state.context_menu = None;
                if let Some(position) = cursor.position_over(layout.bounds()) {
                    let index = self.hit_index(&state.paragraph, layout.bounds(), position);
                    state.begin_selection(index);
                    self.rebuild(state, layout.bounds().size());
                    shell.request_redraw();
                    shell.capture_event();
                } else if state.focused || state.selection().is_some() {
                    state.focused = false;
                    state.dragging = false;
                    state.anchor = 0;
                    state.focus = 0;
                    self.rebuild(state, layout.bounds().size());
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) if state.dragging => {
                let position = cursor.position().unwrap_or(*position);
                let index = self.hit_index(&state.paragraph, layout.bounds(), position);
                if state.extend_selection(index) {
                    self.rebuild(state, layout.bounds().size());
                    shell.request_redraw();
                }
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) if state.dragging => {
                state.dragging = false;
                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.modifiers = *modifiers;
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) if state.focused => {
                let keyboard::Key::Character(character) = key else {
                    return;
                };
                if state.modifiers.command() && character.eq_ignore_ascii_case("c") {
                    if let Some(selection) = self.selected_text(state) {
                        clipboard.write(
                            iced::advanced::clipboard::Kind::Standard,
                            selection.to_owned(),
                        );
                    }
                    shell.capture_event();
                } else if state.modifiers.command() && character.eq_ignore_ascii_case("a") {
                    state.anchor = 0;
                    state.focus = self.content.len();
                    self.rebuild(state, layout.bounds().size());
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<SelectableTypographyState>();
        let anchor = self.text_anchor(&state.paragraph, layout.bounds());
        if let Some(selection) = state.selection()
            && selection.start < selection.end
        {
            let selected_span = usize::from(selection.start > 0);
            let highlight = Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.24);
            if let Some(visible_bounds) = layout.bounds().intersection(viewport) {
                renderer.with_layer(visible_bounds, |renderer| {
                    for selection_bounds in state.paragraph.span_bounds(selected_span) {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: Rectangle {
                                    x: anchor.x + selection_bounds.x,
                                    y: anchor.y + selection_bounds.y,
                                    ..selection_bounds
                                },
                                border: Border {
                                    radius: 3.0.into(),
                                    ..Border::default()
                                },
                                ..renderer::Quad::default()
                            },
                            Background::Color(highlight),
                        );
                    }
                });
            }
        }
        renderer.fill_paragraph(&state.paragraph, anchor, self.color, *viewport);
    }

    fn operate(
        &mut self,
        _tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        operation.text(None, layout.bounds(), &self.content);
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Text
        } else {
            mouse::Interaction::None
        }
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut Tree,
        _layout: Layout<'a>,
        _renderer: &iced::Renderer,
        _viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, iced::Renderer>> {
        let state = tree.state.downcast_mut::<SelectableTypographyState>();
        let origin = translated_typography_context_position(state.context_menu?, translation);
        let selected = self.selected_text(state)?.to_owned();

        Some(overlay::Element::new(Box::new(TypographyContextMenu {
            origin,
            selected,
            on_copy: self.on_copy.as_deref(),
            open: &mut state.context_menu,
        })))
    }
}

struct TypographyContextMenu<'a, Message> {
    origin: Point,
    selected: String,
    on_copy: Option<&'a dyn Fn() -> Message>,
    open: &'a mut Option<Point>,
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer>
    for TypographyContextMenu<'_, Message>
{
    fn layout(&mut self, _renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let size = Size::new(112.0, 36.0);
        let margin = 8.0;
        let origin = Point::new(
            (self.origin.x + 2.0).clamp(margin, (bounds.width - size.width - margin).max(margin)),
            (self.origin.y + 2.0).clamp(margin, (bounds.height - size.height - margin).max(margin)),
        );
        layout::Node::new(size).move_to(origin)
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let inside = cursor.is_over(layout.bounds());
        let copy = matches!(
            event,
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if inside
        );
        let dismiss = matches!(
            event,
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) | Event::Mouse(mouse::Event::ButtonPressed(_)) if !inside
        );

        if copy {
            clipboard.write(
                iced::advanced::clipboard::Kind::Standard,
                self.selected.clone(),
            );
            if let Some(on_copy) = self.on_copy {
                shell.publish(on_copy());
            }
            *self.open = None;
            shell.capture_event();
        } else if dismiss {
            *self.open = None;
            shell.capture_event();
        }
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();
        let (text_bounds, text_origin) = typography_context_menu_text_layout(bounds);
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: LINE,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..renderer::Quad::default()
            },
            Background::Color(SURFACE),
        );
        renderer.fill_text(
            iced::advanced::text::Text {
                content: "复制".to_owned(),
                bounds: text_bounds,
                size: Pixels(13.0),
                line_height: iced::advanced::text::LineHeight::Absolute(Pixels(20.0)),
                font: crate::fonts::MEDIUM,
                align_x: iced::advanced::text::Alignment::Left,
                align_y: iced::alignment::Vertical::Center,
                shaping: iced::advanced::text::Shaping::Auto,
                wrapping: iced::advanced::text::Wrapping::None,
            },
            text_origin,
            INK,
            bounds,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

fn typography_element<'a, Message: 'a>(
    typography: Typography,
    on_copy: Option<Box<dyn Fn() -> Message + 'a>>,
) -> Element<'a, Message> {
    let kind = typography.kind;
    let metrics = kind.metrics();
    let color = match typography.color {
        TypographyColor::Default => INK,
        TypographyColor::Muted => INK_MUTED,
    };
    let align = match typography.align {
        TypographyAlign::Start => iced::advanced::text::Alignment::Left,
        TypographyAlign::Center => iced::advanced::text::Alignment::Center,
        TypographyAlign::End => iced::advanced::text::Alignment::Right,
    };
    let wrapping = if typography.truncate {
        iced::advanced::text::Wrapping::None
    } else {
        iced::advanced::text::Wrapping::Word
    };
    let content: Element<'a, Message> = Element::new(SelectableTypography {
        content: typography.content,
        metrics: TypographyMetrics {
            font: typography_font(metrics.font, typography.weight),
            ..metrics
        },
        color,
        align,
        wrapping,
        width: typography.width,
        on_copy,
    });

    if kind == TypographyType::Code {
        container(content)
            .padding([2, 6])
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(244, 244, 245))),
                border: Border {
                    radius: 6.0.into(),
                    ..Border::default()
                },
                ..container::Style::default()
            })
            .into()
    } else {
        content
    }
}

impl<'a, Message: 'a> From<Typography> for Element<'a, Message> {
    fn from(typography: Typography) -> Self {
        typography_element(typography, None)
    }
}

impl<'a, Message> From<CopyableTypography<Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(copyable: CopyableTypography<Message>) -> Self {
        let on_copy = copyable.on_copy;
        typography_element(copyable.typography, Some(Box::new(move || on_copy.clone())))
    }
}

