// Astra UI: a small, local design system for Iced applications.
//
// The public API intentionally mirrors HeroUI's language: semantic variants,
// consistent radii, and one blue/cyan palette shared by every primitive.

use iced::advanced::Renderer as _;
use iced::advanced::text::{Paragraph as _, Renderer as _};
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget, layout, mouse, overlay, renderer};
use iced::animation::{Animation, Easing};
use iced::time::{Duration, Instant};
use iced::widget::{
    button, canvas as iced_canvas, checkbox, column, container, image, mouse_area, overlay::menu,
    pick_list, radio as iced_radio, row, rule, scrollable, space, stack, text, text_editor,
    text_input,
};
use iced::{
    Background, Border, Color, Element, Event, Fill, Font, Length, Pixels, Point, Radians,
    Rectangle, Shadow, Size, Theme, Vector, keyboard, touch,
};
use lucide_icons::Icon as LucideIcon;

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

const SWITCH_WIDTH: f32 = 40.0;
const SWITCH_HEIGHT: f32 = 20.0;
const SWITCH_THUMB_SIZE: f32 = 16.0;
const SWITCH_PADDING: f32 = 2.0;

const PAGINATION_ITEM_SIZE: f32 = 34.0;
const PAGINATION_NAV_WIDTH: f32 = 94.0;

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
    modal: Animation<bool>,
    drawer: Animation<bool>,
}

impl Default for MotionState {
    fn default() -> Self {
        Self {
            press: bool_animation(false),
            active_press: None,
            toggled: bool_animation(false),
            checked: bool_animation(false),
            progress: bool_animation(false),
            modal: overlay_animation(false, Duration::from_millis(220)),
            drawer: overlay_animation(false, Duration::from_millis(260)),
        }
    }
}

fn bool_animation(initial: bool) -> Animation<bool> {
    Animation::new(initial)
        .duration(MOTION_DURATION)
        .easing(Easing::EaseOutCubic)
}

fn overlay_animation(initial: bool, duration: Duration) -> Animation<bool> {
    Animation::new(initial)
        .duration(duration)
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

    pub fn set_modal(&mut self, visible: bool, now: Instant) {
        self.modal.go_mut(visible, now);
    }

    pub fn set_drawer(&mut self, visible: bool, now: Instant) {
        self.drawer.go_mut(visible, now);
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
            || self.modal.is_animating(now)
            || self.drawer.is_animating(now)
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

    pub fn modal_progress(&self, now: Instant) -> f32 {
        self.modal.interpolate(0.0, 1.0, now)
    }

    pub fn drawer_progress(&self, now: Instant) -> f32 {
        self.drawer.interpolate(0.0, 1.0, now)
    }

    pub fn modal_animating(&self, now: Instant) -> bool {
        self.modal.is_animating(now)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertKind {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Rounded,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarColor {
    Default,
    #[default]
    Accent,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionVariant {
    #[default]
    Default,
    Surface,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionSelectionMode {
    #[default]
    Single,
    Multiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    Info,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeContent<'a> {
    Dot,
    Count(u32),
    Label(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgePosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipVariant {
    Flat,
    Solid,
    Outline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardVariant {
    Transparent,
    #[default]
    Default,
    Secondary,
    Tertiary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastVariant {
    Default,
    Accent,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressBarColor {
    Default,
    #[default]
    Accent,
    Success,
    Warning,
    Danger,
}

impl ProgressBarColor {
    const fn fill(self) -> Color {
        match self {
            Self::Default => NAVY_800,
            Self::Accent => BLUE_600,
            Self::Success => SUCCESS,
            Self::Warning => WARNING,
            Self::Danger => DANGER,
        }
    }
}

pub type ProgressCircleColor = ProgressBarColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressBarSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ProgressBarSize {
    const fn girth(self) -> f32 {
        match self {
            Self::Small => 4.0,
            Self::Medium => 8.0,
            Self::Large => 12.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressCircleSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl AvatarSize {
    const fn diameter(self) -> f32 {
        match self {
            Self::Small => 32.0,
            Self::Medium => 40.0,
            Self::Large => 48.0,
        }
    }
}

impl AvatarShape {
    fn radius(self, diameter: f32) -> iced::border::Radius {
        match self {
            Self::Circle => (diameter / 2.0).into(),
            Self::Rounded => (diameter / 4.0).into(),
            Self::Square => 0.0.into(),
        }
    }
}

impl AvatarColor {
    const fn background(self) -> Color {
        match self {
            Self::Default => NAVY_800,
            Self::Accent => BLUE_600,
            Self::Success => SUCCESS,
            Self::Warning => WARNING,
            Self::Danger => DANGER,
        }
    }
}
