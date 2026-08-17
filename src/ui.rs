//! Astra UI: a small, local design system for Iced applications.
//!
//! The public API intentionally mirrors HeroUI's language: semantic variants,
//! consistent radii, and one blue/cyan palette shared by every primitive.

use iced::advanced::Renderer as _;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget, layout, mouse, renderer};
use iced::animation::{Animation, Easing};
use iced::time::{Duration, Instant};
use iced::widget::{
    button, checkbox, container, pick_list, progress_bar, radio, text_input, toggler,
};
use iced::{
    Background, Border, Color, Element, Event, Length, Point, Rectangle, Shadow, Size, Theme,
    Vector, touch,
};

pub const NAVY_950: Color = Color::from_rgb8(6, 6, 7);
pub const NAVY_900: Color = Color::from_rgb8(24, 24, 27);
pub const NAVY_800: Color = Color::from_rgb8(40, 40, 44);
pub const BLUE_700: Color = Color::from_rgb8(29, 99, 174);
pub const BLUE_600: Color = Color::from_rgb8(4, 133, 247);
pub const BLUE_500: Color = Color::from_rgb8(53, 146, 249);
pub const CYAN_500: Color = Color::from_rgb8(6, 182, 212);
pub const CYAN_300: Color = Color::from_rgb8(103, 232, 249);
pub const INK: Color = Color::from_rgb8(24, 24, 27);
pub const INK_MUTED: Color = Color::from_rgb8(113, 113, 122);
pub const INK_SUBTLE: Color = Color::from_rgb8(161, 161, 168);
pub const CANVAS: Color = Color::from_rgb8(245, 245, 245);
pub const SURFACE: Color = Color::WHITE;
pub const SURFACE_ALT: Color = Color::from_rgb8(235, 235, 236);
pub const LINE: Color = Color::from_rgb8(222, 222, 224);
pub const SUCCESS: Color = Color::from_rgb8(23, 201, 100);
pub const WARNING: Color = Color::from_rgb8(245, 165, 36);
pub const DANGER: Color = Color::from_rgb8(255, 56, 60);
pub const WHITE: Color = Color::from_rgb8(252, 252, 252);
pub const WHITE_MUTED: Color = Color::from_rgb8(161, 161, 168);

pub const RADIUS_FIELD: f32 = 12.0;
pub const RADIUS_CONTROL: f32 = 24.0;
pub const RADIUS_INNER: f32 = 20.0;
pub const RADIUS_PANEL: f32 = 24.0;
pub const CONTROL_HEIGHT_SM: f32 = 32.0;
pub const CONTROL_HEIGHT_MD: f32 = 36.0;
pub const CONTROL_HEIGHT_LG: f32 = 40.0;

const SLIDER_HANDLE_RADIUS: f32 = 8.0;
pub const SLIDER_HEIGHT: f32 = 20.0;
pub const SLIDER_WIDTH: f32 = 200.0;

const MOTION_DURATION: Duration = Duration::from_millis(140);

/// Shared short transitions used by the component showcase and reusable styles.
/// The state is kept by the application while style functions remain stateless.
#[derive(Debug)]
pub struct MotionState {
    press: Animation<bool>,
    active_press: Option<&'static str>,
    toggled: Animation<bool>,
    checked: Animation<bool>,
    progress: Animation<bool>,
}

impl Default for MotionState {
    fn default() -> Self {
        Self {
            press: bool_animation(false),
            active_press: None,
            toggled: bool_animation(false),
            checked: bool_animation(false),
            progress: bool_animation(false),
        }
    }
}

fn bool_animation(initial: bool) -> Animation<bool> {
    Animation::new(initial)
        .duration(MOTION_DURATION)
        .easing(Easing::EaseOutCubic)
}

impl MotionState {
    pub fn press(&mut self, id: &'static str, now: Instant) {
        self.active_press = Some(id);
        self.press.go_mut(true, now);
    }

    pub fn set_toggled(&mut self, value: bool, now: Instant) {
        self.toggled.go_mut(value, now);
    }

    pub fn set_checked(&mut self, value: bool, now: Instant) {
        self.checked.go_mut(value, now);
    }

    pub fn start_progress(&mut self, now: Instant) {
        self.progress.go_mut(true, now);
    }

    pub fn tick(&mut self, now: Instant) {
        if self.press.value() && !self.press.is_animating(now) {
            self.press.go_mut(false, now);
        } else if !self.press.value() && !self.press.is_animating(now) {
            self.active_press = None;
        }
    }

    pub fn needs_ticks(&self, now: Instant) -> bool {
        self.active_press.is_some()
            || self.press.is_animating(now)
            || self.toggled.is_animating(now)
            || self.checked.is_animating(now)
            || self.progress.is_animating(now)
    }

    pub fn press_progress(&self, id: &'static str, now: Instant) -> f32 {
        if self.active_press == Some(id) {
            self.press.interpolate(0.0, 1.0, now)
        } else {
            0.0
        }
    }

    pub fn toggled_progress(&self, now: Instant) -> f32 {
        self.toggled.interpolate(0.0, 1.0, now)
    }

    pub fn checked_progress(&self, now: Instant) -> f32 {
        self.checked.interpolate(0.0, 1.0, now)
    }

    pub fn progress_progress(&self, now: Instant) -> f32 {
        self.progress.interpolate(0.0, 1.0, now)
    }
}

fn mix_color(from: Color, to: Color, amount: f32) -> Color {
    let amount = amount.clamp(0.0, 1.0);
    Color::from_rgba(
        from.r + (to.r - from.r) * amount,
        from.g + (to.g - from.g) * amount,
        from.b + (to.b - from.b) * amount,
        from.a + (to.a - from.a) * amount,
    )
}

pub fn app_theme() -> Theme {
    Theme::custom(
        "Astra UI".to_owned(),
        iced::theme::Palette {
            background: CANVAS,
            text: INK,
            primary: BLUE_600,
            success: SUCCESS,
            warning: WARNING,
            danger: DANGER,
        },
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Tertiary,
    Ghost,
    Destructive,
    DangerSoft,
    Outline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertKind {
    Info,
    Success,
    Warning,
    Danger,
}

pub fn canvas(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(CANVAS)),
        text_color: Some(INK),
        ..container::Style::default()
    }
}

pub fn sidebar(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        text_color: Some(INK),
        border: Border {
            color: LINE,
            width: 1.0,
            ..Border::default()
        },
        ..container::Style::default()
    }
}

pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            radius: RADIUS_PANEL.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

pub fn flat_card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.937, 0.937, 0.941))),
        border: Border {
            radius: RADIUS_INNER.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

pub fn code_block(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(INK)),
        border: Border {
            radius: RADIUS_FIELD.into(),
            ..Border::default()
        },
        text_color: Some(CYAN_300),
        ..container::Style::default()
    }
}

pub fn tint(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE_ALT)),
        border: Border {
            radius: RADIUS_FIELD.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

/// Outer frame for Figma fields. `text_input::Style` has no shadow field, so
/// the elevation belongs to a zero-padding container around the control.
pub fn field_frame(_theme: &Theme) -> container::Style {
    container::Style {
        border: Border {
            radius: RADIUS_FIELD.into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
        ..container::Style::default()
    }
}

pub fn button_style(variant: ButtonVariant) -> impl Fn(&Theme, button::Status) -> button::Style {
    button_style_animated(variant, 0.0)
}

pub fn button_style_animated(
    variant: ButtonVariant,
    press_progress: f32,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        let disabled = matches!(status, button::Status::Disabled);
        let interactive = hovered || pressed;
        let press_mix = press_progress.max(if pressed { 0.75 } else { 0.0 });
        let default_hover = Color::from_rgb8(225, 225, 226);
        let on_surface_hover = Color::from_rgb8(239, 239, 240);
        let (background, text_color, border_color) = match variant {
            ButtonVariant::Primary => (
                Some(if interactive { BLUE_500 } else { BLUE_600 }),
                WHITE,
                Color::TRANSPARENT,
            ),
            ButtonVariant::Secondary => (
                Some(if interactive {
                    default_hover
                } else {
                    SURFACE_ALT
                }),
                BLUE_700,
                Color::TRANSPARENT,
            ),
            ButtonVariant::Tertiary => (
                Some(if interactive {
                    default_hover
                } else {
                    SURFACE_ALT
                }),
                INK,
                Color::TRANSPARENT,
            ),
            ButtonVariant::Ghost => (
                if interactive {
                    Some(on_surface_hover)
                } else {
                    None
                },
                INK,
                Color::TRANSPARENT,
            ),
            ButtonVariant::Destructive => (
                Some(if interactive {
                    Color::from_rgb8(255, 85, 81)
                } else {
                    DANGER
                }),
                WHITE,
                Color::TRANSPARENT,
            ),
            ButtonVariant::DangerSoft => (
                Some(if interactive {
                    Color::from_rgba(DANGER.r, DANGER.g, DANGER.b, 0.20)
                } else {
                    Color::from_rgba(DANGER.r, DANGER.g, DANGER.b, 0.15)
                }),
                Color::from_rgb8(164, 53, 50),
                Color::TRANSPARENT,
            ),
            ButtonVariant::Outline => (
                if interactive {
                    Some(on_surface_hover)
                } else {
                    None
                },
                INK,
                LINE,
            ),
        };

        button::Style {
            background: background.map(|color| {
                let color = mix_color(
                    color,
                    Color::from_rgba(0.0, 0.0, 0.0, color.a),
                    0.08 * press_mix,
                );
                Background::Color(if disabled {
                    Color::from_rgba(color.r, color.g, color.b, color.a * 0.5)
                } else {
                    color
                })
            }),
            text_color: if disabled {
                Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.5)
            } else {
                text_color
            },
            border: Border {
                color: border_color,
                width: if matches!(variant, ButtonVariant::Outline) {
                    1.0
                } else {
                    0.0
                },
                radius: RADIUS_CONTROL.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.08 * press_mix),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0 + 3.0 * press_mix,
            },
            ..button::Style::default()
        }
    }
}

pub fn nav_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    nav_button_animated(active, 0.0)
}

pub fn nav_button_animated(
    active: bool,
    press_progress: f32,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let press_mix = press_progress.clamp(0.0, 1.0);
        button::Style {
            background: if active || hovered {
                Some(Background::Color(if active {
                    SURFACE_ALT
                } else {
                    Color::from_rgb8(245, 245, 245)
                }))
            } else {
                None
            },
            text_color: if active { INK } else { INK_MUTED },
            border: Border {
                radius: RADIUS_INNER.into(),
                ..Border::default()
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.04 * press_mix),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            ..button::Style::default()
        }
    }
}

pub fn text_input_style(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let focused = matches!(status, text_input::Status::Focused { .. });
    let hovered = matches!(
        status,
        text_input::Status::Hovered | text_input::Status::Focused { is_hovered: true }
    );
    let disabled = matches!(status, text_input::Status::Disabled);
    text_input::Style {
        background: Background::Color(if disabled {
            Color::from_rgba(SURFACE_ALT.r, SURFACE_ALT.g, SURFACE_ALT.b, 0.5)
        } else if hovered && !focused {
            Color::from_rgb8(249, 249, 249)
        } else {
            SURFACE
        }),
        border: Border {
            color: if focused { BLUE_600 } else { LINE },
            width: if focused { 2.0 } else { 1.0 },
            radius: RADIUS_FIELD.into(),
        },
        icon: BLUE_600,
        placeholder: INK_SUBTLE,
        value: INK,
        selection: Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.20),
    }
}

pub fn pick_list_style(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let active = matches!(
        status,
        pick_list::Status::Opened { .. } | pick_list::Status::Hovered
    );
    pick_list::Style {
        text_color: INK,
        placeholder_color: INK_SUBTLE,
        handle_color: BLUE_600,
        background: Background::Color(if active {
            Color::from_rgb8(249, 249, 249)
        } else {
            SURFACE
        }),
        border: Border {
            color: if active { BLUE_600 } else { LINE },
            width: if active { 2.0 } else { 1.0 },
            radius: RADIUS_FIELD.into(),
        },
    }
}

pub fn toggler_style(_theme: &Theme, status: toggler::Status) -> toggler::Style {
    let target = match status {
        toggler::Status::Active { is_toggled }
        | toggler::Status::Hovered { is_toggled }
        | toggler::Status::Disabled { is_toggled } => is_toggled,
    };
    toggler_style_with_progress(status, if target { 1.0 } else { 0.0 })
}

pub fn toggler_style_animated(
    transition_progress: f32,
) -> impl Fn(&Theme, toggler::Status) -> toggler::Style {
    move |_theme, status| toggler_style_with_progress(status, transition_progress)
}

fn toggler_style_with_progress(
    status: toggler::Status,
    transition_progress: f32,
) -> toggler::Style {
    let (_is_toggled, disabled) = match status {
        toggler::Status::Active { is_toggled } | toggler::Status::Hovered { is_toggled } => {
            (is_toggled, false)
        }
        toggler::Status::Disabled { is_toggled } => (is_toggled, true),
    };
    toggler::Style {
        background: Background::Color({
            let color = mix_color(SURFACE_ALT, BLUE_600, transition_progress);
            if disabled {
                Color::from_rgba(color.r, color.g, color.b, 0.5)
            } else {
                color
            }
        }),
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
        foreground: Background::Color(WHITE),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(INK),
        border_radius: Some(RADIUS_CONTROL.into()),
        padding_ratio: 0.10,
    }
}

pub fn checkbox_style(_theme: &Theme, status: checkbox::Status) -> checkbox::Style {
    let target = match status {
        checkbox::Status::Active { is_checked }
        | checkbox::Status::Hovered { is_checked }
        | checkbox::Status::Disabled { is_checked } => is_checked,
    };
    checkbox_style_with_progress(status, if target { 1.0 } else { 0.0 })
}

pub fn checkbox_style_animated(
    transition_progress: f32,
) -> impl Fn(&Theme, checkbox::Status) -> checkbox::Style {
    move |_theme, status| checkbox_style_with_progress(status, transition_progress)
}

fn checkbox_style_with_progress(
    status: checkbox::Status,
    transition_progress: f32,
) -> checkbox::Style {
    let (_checked, disabled) = match status {
        checkbox::Status::Active { is_checked } | checkbox::Status::Hovered { is_checked } => {
            (is_checked, false)
        }
        checkbox::Status::Disabled { is_checked } => (is_checked, true),
    };
    checkbox::Style {
        background: Background::Color(mix_color(SURFACE, BLUE_600, transition_progress)),
        icon_color: mix_color(INK, WHITE, transition_progress),
        border: Border {
            color: mix_color(LINE, BLUE_600, transition_progress),
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: Some(if disabled { INK_SUBTLE } else { INK }),
    }
}

pub fn radio_style(_theme: &Theme, status: radio::Status) -> radio::Style {
    let selected = match status {
        radio::Status::Active { is_selected } | radio::Status::Hovered { is_selected } => {
            is_selected
        }
    };
    radio::Style {
        background: Background::Color(SURFACE),
        dot_color: BLUE_600,
        border_width: 1.0,
        border_color: if selected { BLUE_600 } else { LINE },
        text_color: Some(INK),
    }
}

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
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<HeroSliderState>();
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
        let active = state.is_dragging || cursor.is_over(bounds);
        let endpoint = progress <= f32::EPSILON || progress >= 1.0 - f32::EPSILON;
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: handle_center - SLIDER_HANDLE_RADIUS,
                    y: bounds.center_y() - SLIDER_HANDLE_RADIUS,
                    width: SLIDER_HANDLE_RADIUS * 2.0,
                    height: SLIDER_HANDLE_RADIUS * 2.0,
                },
                border: Border {
                    color: if active {
                        BLUE_500
                    } else if endpoint {
                        BLUE_600
                    } else {
                        LINE
                    },
                    width: 1.0,
                    radius: SLIDER_HANDLE_RADIUS.into(),
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

pub fn progress_style(_theme: &Theme) -> progress_bar::Style {
    progress_bar::Style {
        background: Background::Color(SURFACE_ALT),
        bar: Background::Color(BLUE_600),
        border: Border {
            radius: RADIUS_FIELD.into(),
            ..Border::default()
        },
    }
}

pub fn badge(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(Color::from_rgba(
            color.r, color.g, color.b, 0.15,
        ))),
        border: Border {
            radius: RADIUS_FIELD.into(),
            ..Border::default()
        },
        text_color: Some(color),
        ..container::Style::default()
    }
}

pub fn alert(_kind: AlertKind) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            radius: RADIUS_PANEL.into(),
            ..Border::default()
        },
        text_color: Some(INK),
        ..container::Style::default()
    }
}

pub fn tab(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    tab_animated(active, 0.0)
}

pub fn tab_animated(
    active: bool,
    transition_progress: f32,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        button::Style {
            background: if active || hovered {
                Some(Background::Color(if active {
                    SURFACE
                } else {
                    Color::from_rgb(0.937, 0.937, 0.941)
                }))
            } else {
                None
            },
            text_color: INK,
            border: Border {
                radius: RADIUS_CONTROL.into(),
                ..Border::default()
            },
            shadow: if active {
                Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.06 + 0.04 * transition_progress),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                }
            } else {
                Shadow::default()
            },
            ..button::Style::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HeroSlider, SLIDER_HANDLE_RADIUS, SLIDER_HEIGHT, SLIDER_WIDTH};
    use iced::Rectangle;

    fn test_slider(value: f32) -> HeroSlider<'static, ()> {
        HeroSlider {
            range: 0.0..=100.0,
            value,
            step: 0.1,
            on_change: Box::new(|_| ()),
        }
    }

    #[test]
    fn slider_handle_centers_map_to_exact_endpoints() {
        let bounds = Rectangle::new([10.0, 20.0].into(), [SLIDER_WIDTH, SLIDER_HEIGHT].into());
        let slider = test_slider(0.0);
        let left = bounds.x + SLIDER_HANDLE_RADIUS;
        let right = bounds.x + bounds.width - SLIDER_HANDLE_RADIUS;

        assert_eq!(slider.value_at(bounds, left, 0.0), 0.0);
        assert_eq!(slider.value_at(bounds, right, 0.0), 100.0);
    }

    #[test]
    fn grabbing_thumb_edge_does_not_jump_from_zero() {
        let bounds = Rectangle::new([10.0, 20.0].into(), [SLIDER_WIDTH, SLIDER_HEIGHT].into());
        let slider = test_slider(0.0);
        let handle_center = slider.handle_center(bounds);
        let pointer = handle_center - 6.0;
        let grab_offset = pointer - handle_center;

        assert_eq!(slider.value_at(bounds, pointer, grab_offset), 0.0);
        assert!(slider.value_at(bounds, pointer + 2.0, grab_offset) > 0.0);
    }
}
