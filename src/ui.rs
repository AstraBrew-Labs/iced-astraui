//! Astra UI: a small, local design system for Iced applications.
//!
//! The public API intentionally mirrors HeroUI's language: semantic variants,
//! consistent radii, and one blue/cyan palette shared by every primitive.

use iced::advanced::Renderer as _;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget, layout, mouse, overlay, renderer};
use iced::animation::{Animation, Easing};
use iced::time::{Duration, Instant};
use iced::widget::{
    button, checkbox, column, container, mouse_area, overlay::menu, pick_list, progress_bar,
    radio as iced_radio, row, rule, space, stack, text, text_input,
};
use iced::{
    Background, Border, Color, Element, Event, Fill, Font, Length, Pixels, Point, Rectangle,
    Shadow, Size, Theme, Vector, keyboard, touch,
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
pub enum ToastPlacement {
    TopStart,
    Top,
    TopEnd,
    BottomStart,
    #[default]
    Bottom,
    BottomEnd,
}

impl ToastPlacement {
    pub const ALL: [Self; 6] = [
        Self::TopStart,
        Self::Top,
        Self::TopEnd,
        Self::BottomStart,
        Self::Bottom,
        Self::BottomEnd,
    ];

    pub const fn alignment(self) -> (iced::Alignment, iced::Alignment) {
        match self {
            Self::TopStart => (iced::Alignment::Start, iced::Alignment::Start),
            Self::Top => (iced::Alignment::Center, iced::Alignment::Start),
            Self::TopEnd => (iced::Alignment::End, iced::Alignment::Start),
            Self::BottomStart => (iced::Alignment::Start, iced::Alignment::End),
            Self::Bottom => (iced::Alignment::Center, iced::Alignment::End),
            Self::BottomEnd => (iced::Alignment::End, iced::Alignment::End),
        }
    }
}

impl std::fmt::Display for ToastPlacement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::TopStart => "Top start",
            Self::Top => "Top center",
            Self::TopEnd => "Top end",
            Self::BottomStart => "Bottom start",
            Self::Bottom => "Bottom center",
            Self::BottomEnd => "Bottom end",
        })
    }
}

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

fn card_variant_style(variant: CardVariant) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let (background, border, shadow) = match variant {
            CardVariant::Transparent => (None, Border::default(), Shadow::default()),
            CardVariant::Default => (
                Some(Background::Color(SURFACE)),
                Border {
                    color: LINE,
                    width: 1.0,
                    radius: RADIUS_PANEL.into(),
                },
                Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.07),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 10.0,
                },
            ),
            CardVariant::Secondary => (
                Some(Background::Color(SURFACE_ALT)),
                Border {
                    color: Color::from_rgba(LINE.r, LINE.g, LINE.b, 0.72),
                    width: 1.0,
                    radius: RADIUS_PANEL.into(),
                },
                Shadow::default(),
            ),
            CardVariant::Tertiary => (
                Some(Background::Color(Color::from_rgb8(226, 240, 253))),
                Border {
                    color: Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.18),
                    width: 1.0,
                    radius: RADIUS_PANEL.into(),
                },
                Shadow::default(),
            ),
        };

        container::Style {
            background,
            border: if matches!(variant, CardVariant::Transparent) {
                border
            } else {
                Border {
                    radius: RADIUS_PANEL.into(),
                    ..border
                }
            },
            shadow,
            text_color: Some(INK),
            ..container::Style::default()
        }
    }
}

/// A composable HeroUI-style card with optional header and footer sections.
pub struct Card<'a, Message> {
    header: Option<Element<'a, Message>>,
    content: Element<'a, Message>,
    footer: Option<Element<'a, Message>>,
    variant: CardVariant,
    width: Length,
    padding: u16,
    gap: u32,
}

impl<'a, Message> Card<'a, Message>
where
    Message: 'a,
{
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            header: None,
            content: content.into(),
            footer: None,
            variant: CardVariant::Default,
            width: Length::Shrink,
            padding: 16,
            gap: 12,
        }
    }

    pub fn header(mut self, header: impl Into<Element<'a, Message>>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub fn gap(mut self, gap: u32) -> Self {
        self.gap = gap;
        self
    }
}

impl<'a, Message> From<Card<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(card: Card<'a, Message>) -> Self {
        let mut sections = column![].spacing(card.gap);
        if let Some(header) = card.header {
            sections = sections.push(header);
        }
        sections = sections.push(card.content);
        if let Some(footer) = card.footer {
            sections = sections.push(footer);
        }

        container(sections)
            .width(card.width)
            .padding(card.padding)
            .style(card_variant_style(card.variant))
            .into()
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
    let hovered = matches!(status, pick_list::Status::Hovered);
    let opened = matches!(status, pick_list::Status::Opened { .. });
    pick_list::Style {
        text_color: INK,
        placeholder_color: INK_SUBTLE,
        handle_color: if opened { BLUE_600 } else { INK_MUTED },
        background: Background::Color(if hovered {
            Color::from_rgb8(249, 249, 249)
        } else {
            SURFACE
        }),
        border: Border {
            color: if opened {
                BLUE_600
            } else if hovered {
                BLUE_500
            } else {
                LINE
            },
            width: if opened { 2.0 } else { 1.0 },
            radius: RADIUS_FIELD.into(),
        },
    }
}

pub fn pick_list_menu_style(_theme: &Theme) -> menu::Style {
    menu::Style {
        background: Background::Color(SURFACE),
        border: Border {
            color: LINE,
            width: 1.0,
            radius: RADIUS_FIELD.into(),
        },
        text_color: INK,
        selected_text_color: BLUE_700,
        selected_background: Background::Color(Color::from_rgba(
            BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.10,
        )),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 20.0,
        },
    }
}

pub fn pick_list_handle() -> pick_list::Handle<Font> {
    let icon = |glyph: LucideIcon| pick_list::Icon {
        font: Font::with_name("lucide"),
        code_point: glyph.into(),
        size: Some(Pixels(16.0)),
        line_height: iced::widget::text::LineHeight::Absolute(Pixels(16.0)),
        shaping: iced::widget::text::Shaping::Basic,
    };

    pick_list::Handle::Dynamic {
        closed: icon(LucideIcon::ChevronDown),
        open: icon(LucideIcon::ChevronUp),
    }
}

pub struct MenuItem<'a, Message> {
    label: &'a str,
    icon: Option<LucideIcon>,
    on_press: Message,
    danger: bool,
}

impl<'a, Message> MenuItem<'a, Message> {
    pub fn new(label: &'a str, icon: Option<LucideIcon>, on_press: Message) -> Self {
        Self {
            label,
            icon,
            on_press,
            danger: false,
        }
    }

    pub fn danger(label: &'a str, icon: Option<LucideIcon>, on_press: Message) -> Self {
        Self {
            label,
            icon,
            on_press,
            danger: true,
        }
    }
}

fn popup_surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: LINE,
            width: 1.0,
            radius: RADIUS_FIELD.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.14),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        ..container::Style::default()
    }
}

fn menu_item_style(danger: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        let foreground = if danger { DANGER } else { INK };
        let background = if pressed {
            Some(Color::from_rgba(
                foreground.r,
                foreground.g,
                foreground.b,
                0.14,
            ))
        } else if hovered {
            Some(Color::from_rgba(
                foreground.r,
                foreground.g,
                foreground.b,
                0.08,
            ))
        } else {
            None
        };

        button::Style {
            background: background.map(Background::Color),
            text_color: foreground,
            border: Border {
                radius: 8.0.into(),
                ..Border::default()
            },
            ..button::Style::default()
        }
    }
}

fn menu_panel<'a, Message>(items: Vec<MenuItem<'a, Message>>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let items = items
        .into_iter()
        .fold(column![].spacing(2).width(Fill), |menu, item| {
            let foreground = if item.danger { DANGER } else { INK };
            let mut content = row![].spacing(9).align_y(iced::Alignment::Center);
            if let Some(icon) = item.icon {
                content = content.push(crate::icons::icon(icon, 15, foreground));
            }
            content = content.push(
                text(item.label)
                    .size(12)
                    .font(crate::fonts::REGULAR)
                    .color(foreground),
            );

            menu.push(
                button(content)
                    .on_press(item.on_press)
                    .width(Fill)
                    .height(36)
                    .padding([8, 10])
                    .style(menu_item_style(item.danger)),
            )
        });

    container(items)
        .width(208)
        .padding(6)
        .style(popup_surface)
        .into()
}

fn dropdown_button_style(expanded: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        button::Style {
            background: Some(Background::Color(if pressed {
                Color::from_rgb8(218, 218, 220)
            } else if hovered || expanded {
                Color::from_rgb8(228, 228, 230)
            } else {
                SURFACE_ALT
            })),
            text_color: INK,
            border: Border {
                color: if expanded {
                    BLUE_600
                } else {
                    Color::TRANSPARENT
                },
                width: if expanded { 1.0 } else { 0.0 },
                radius: RADIUS_CONTROL.into(),
            },
            ..button::Style::default()
        }
    }
}

/// A button-triggered action menu with optional leading icon support.
pub fn dropdown<'a, Message>(
    trigger_label: &'a str,
    trigger_icon: Option<LucideIcon>,
    expanded: bool,
    on_toggle: Message,
    on_dismiss: Message,
    items: Vec<MenuItem<'a, Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut content = row![].spacing(8).align_y(iced::Alignment::Center);
    if let Some(icon) = trigger_icon {
        content = content.push(crate::icons::icon(icon, 15, INK));
    }
    content = content
        .push(
            text(trigger_label)
                .size(12)
                .font(crate::fonts::MEDIUM)
                .width(Fill),
        )
        .push(crate::icons::icon(
            if expanded {
                LucideIcon::ChevronUp
            } else {
                LucideIcon::ChevronDown
            },
            15,
            if expanded { BLUE_600 } else { INK_MUTED },
        ));

    let trigger = button(content)
        .on_press(on_toggle)
        .width(176)
        .height(CONTROL_HEIGHT_MD)
        .padding([8, 13])
        .style(dropdown_button_style(expanded));

    Popup::dropdown(trigger.into(), menu_panel(items), expanded, on_dismiss).into()
}

/// Opens the shared dropdown menu at the pointer position after a right click.
/// Wrap a local element or a full-screen element to choose the interaction scope.
pub fn context_menu<'a, Message>(
    target: impl Into<Element<'a, Message>>,
    position: Option<Point>,
    on_open: impl Fn(Point) -> Message + 'a,
    on_dismiss: Message,
    items: Vec<MenuItem<'a, Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    Popup::context(
        target.into(),
        menu_panel(items),
        position,
        on_open,
        on_dismiss,
    )
    .into()
}

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
                radius: RADIUS_FIELD.into(),
                ..Border::default()
            },
            ..button::Style::default()
        }
    }
}

/// A titled, full-width section that reveals or hides its content panel.
pub fn disclosure<'a, Message>(
    title: &'a str,
    description: Option<&'a str>,
    expanded: bool,
    on_toggle: Message,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut labels = column![text(title).size(13).font(crate::fonts::MEDIUM).color(INK)].spacing(2);
    if let Some(description) = description {
        labels = labels.push(
            text(description)
                .size(10)
                .font(crate::fonts::REGULAR)
                .color(INK_MUTED),
        );
    }

    let trigger = button(
        row![
            labels.width(Fill),
            crate::icons::icon(
                if expanded {
                    LucideIcon::ChevronUp
                } else {
                    LucideIcon::ChevronDown
                },
                16,
                if expanded { BLUE_600 } else { INK_MUTED },
            )
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
    )
    .on_press(on_toggle)
    .width(Fill)
    .height(52)
    .padding([8, 14])
    .style(disclosure_trigger_style(expanded));

    let panel = if expanded {
        column![
            trigger,
            rule::horizontal(1),
            container(content.into()).width(Fill).padding([14, 16])
        ]
    } else {
        column![trigger]
    };

    container(panel)
        .width(Fill)
        .style(disclosure_surface)
        .into()
}

fn switch_thumb_offset(progress: f32) -> f32 {
    let travel = SWITCH_WIDTH - SWITCH_PADDING * 2.0 - SWITCH_THUMB_SIZE;
    SWITCH_PADDING + travel * progress.clamp(0.0, 1.0)
}

fn switch_track(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(color)),
        border: Border {
            radius: (SWITCH_HEIGHT / 2.0).into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

fn switch_thumb(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(WHITE)),
        border: Border {
            radius: (SWITCH_THUMB_SIZE / 2.0).into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.18),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 3.0,
        },
        ..container::Style::default()
    }
}

fn switch_button(_theme: &Theme, status: button::Status) -> button::Style {
    let interactive = matches!(status, button::Status::Hovered | button::Status::Pressed);
    button::Style {
        background: None,
        text_color: INK,
        border: Border {
            color: if interactive {
                Color::from_rgba(BLUE_600.r, BLUE_600.g, BLUE_600.b, 0.22)
            } else {
                Color::TRANSPARENT
            },
            width: 1.0,
            radius: RADIUS_CONTROL.into(),
        },
        shadow: Shadow::default(),
        ..button::Style::default()
    }
}

/// A 40x20 switch whose track color and thumb position share one transition.
pub fn switch<'a, Message>(
    label: &'a str,
    is_toggled: bool,
    transition_progress: f32,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let progress = transition_progress.clamp(0.0, 1.0);
    let offset = switch_thumb_offset(progress);
    let track_color = mix_color(SURFACE_ALT, BLUE_600, progress);
    let control = stack![
        container(space::Space::new())
            .width(SWITCH_WIDTH)
            .height(SWITCH_HEIGHT)
            .style(switch_track(track_color)),
        container(row![
            space::Space::new().width(offset),
            container(space::Space::new())
                .width(SWITCH_THUMB_SIZE)
                .height(SWITCH_THUMB_SIZE)
                .style(switch_thumb)
        ])
        .width(SWITCH_WIDTH)
        .height(SWITCH_HEIGHT)
        .align_y(iced::Alignment::Center),
    ]
    .width(SWITCH_WIDTH)
    .height(SWITCH_HEIGHT);

    button(
        row![
            control,
            text(label).size(12).font(crate::fonts::REGULAR).color(INK)
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
    )
    .on_press(on_toggle(!is_toggled))
    .padding(0)
    .height(SWITCH_HEIGHT)
    .style(switch_button)
    .into()
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

pub fn radio<'a, V, Message>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_select: impl FnOnce(V) -> Message,
) -> Element<'a, Message>
where
    V: Eq + Copy,
    Message: Clone + 'a,
{
    iced_radio(label, value, selected, on_select)
        .size(16)
        .spacing(8)
        .text_size(12)
        .text_line_height(iced::widget::text::LineHeight::Absolute(Pixels(20.0)))
        .font(crate::fonts::REGULAR)
        .style(radio_style)
        .into()
}

pub fn radio_style(_theme: &Theme, status: iced_radio::Status) -> iced_radio::Style {
    let (selected, hovered) = match status {
        iced_radio::Status::Active { is_selected } => (is_selected, false),
        iced_radio::Status::Hovered { is_selected } => (is_selected, true),
    };
    iced_radio::Style {
        background: Background::Color(SURFACE),
        dot_color: BLUE_600,
        border_width: 1.0,
        border_color: if selected {
            BLUE_600
        } else if hovered {
            BLUE_500
        } else {
            LINE
        },
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

pub fn tag_style(color: Color) -> impl Fn(&Theme) -> container::Style {
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

fn readable_on(color: Color) -> Color {
    fn linear(channel: f32) -> f32 {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }

    fn luminance(color: Color) -> f32 {
        0.2126 * linear(color.r) + 0.7152 * linear(color.g) + 0.0722 * linear(color.b)
    }

    fn contrast(first: Color, second: Color) -> f32 {
        let (lighter, darker) = {
            let first = luminance(first);
            let second = luminance(second);
            if first > second {
                (first, second)
            } else {
                (second, first)
            }
        };
        (lighter + 0.05) / (darker + 0.05)
    }

    if contrast(color, INK) >= contrast(color, WHITE) {
        INK
    } else {
        WHITE
    }
}

fn chip_style(color: Color, variant: ChipVariant) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let (background, border_color, border_width, text_color) = match variant {
            ChipVariant::Flat => (
                Some(Color::from_rgba(color.r, color.g, color.b, 0.14)),
                Color::TRANSPARENT,
                0.0,
                color,
            ),
            ChipVariant::Solid => (Some(color), Color::TRANSPARENT, 0.0, readable_on(color)),
            ChipVariant::Outline => (None, color, 1.0, color),
        };

        container::Style {
            background: background.map(Background::Color),
            border: Border {
                color: border_color,
                width: border_width,
                radius: 999.0.into(),
            },
            text_color: Some(text_color),
            ..container::Style::default()
        }
    }
}

/// A compact label for status, category, or lightweight metadata.
pub fn chip<'a, Message>(
    label: &'a str,
    icon: Option<LucideIcon>,
    color: Color,
    variant: ChipVariant,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let foreground = if matches!(variant, ChipVariant::Solid) {
        readable_on(color)
    } else {
        color
    };
    let label = text(label)
        .size(11)
        .font(crate::fonts::MEDIUM)
        .line_height(iced::widget::text::LineHeight::Absolute(Pixels(16.0)))
        .color(foreground);
    let content = if let Some(icon) = icon {
        row![crate::icons::icon(icon, 12, foreground), label]
            .spacing(5)
            .align_y(iced::Alignment::Center)
    } else {
        row![label].align_y(iced::Alignment::Center)
    };

    container(content)
        .height(24)
        .padding([2, 8])
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .style(chip_style(color, variant))
        .into()
}

fn badge_surface(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(color)),
        border: Border {
            color: SURFACE,
            width: 2.0,
            radius: 999.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 3.0,
        },
        text_color: Some(WHITE),
        ..container::Style::default()
    }
}

fn badge_offset(position: BadgePosition, extent: f32) -> Vector {
    let horizontal = match position {
        BadgePosition::TopRight | BadgePosition::BottomRight => extent / 2.0,
        BadgePosition::TopLeft | BadgePosition::BottomLeft => -extent / 2.0,
    };
    let vertical = match position {
        BadgePosition::TopRight | BadgePosition::TopLeft => -extent / 2.0,
        BadgePosition::BottomRight | BadgePosition::BottomLeft => extent / 2.0,
    };

    Vector::new(horizontal, vertical)
}

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

fn message_kind_style(kind: MessageKind) -> (Color, LucideIcon) {
    match kind {
        MessageKind::Info => (BLUE_600, LucideIcon::Info),
        MessageKind::Success => (SUCCESS, LucideIcon::CircleCheck),
        MessageKind::Warning => (WARNING, LucideIcon::TriangleAlert),
        MessageKind::Danger => (DANGER, LucideIcon::CircleX),
    }
}

fn global_message_surface(accent: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: Color::from_rgba(accent.r, accent.g, accent.b, 0.26),
            width: 1.0,
            radius: RADIUS_FIELD.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.14),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        ..container::Style::default()
    }
}

fn centered_button_icon<'a, Message: 'a>(
    glyph: LucideIcon,
    size: u32,
    color: Color,
) -> Element<'a, Message> {
    container(crate::icons::icon(glyph, size, color))
        .width(Fill)
        .height(Fill)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .into()
}

/// A compact global message suitable for action feedback and status updates.
pub fn global_message<'a, Message>(
    title: &'a str,
    description: &'a str,
    kind: MessageKind,
    on_close: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let (accent, glyph) = message_kind_style(kind);
    container(
        row![
            container(crate::icons::icon(glyph, 18, accent))
                .width(36)
                .height(36)
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center)
                .style(tag_style(accent)),
            column![
                text(title).size(13).font(crate::fonts::BOLD).color(INK),
                text(description)
                    .size(11)
                    .font(crate::fonts::REGULAR)
                    .color(INK_MUTED)
            ]
            .spacing(3)
            .width(Fill),
            button(centered_button_icon(LucideIcon::X, 15, INK_MUTED))
                .on_press(on_close)
                .width(32)
                .height(32)
                .padding(0)
                .style(button_style(ButtonVariant::Ghost)),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
    )
    .width(360)
    .padding([12, 14])
    .style(global_message_surface(accent))
    .into()
}

fn toast_variant_style(variant: ToastVariant) -> (Color, LucideIcon) {
    match variant {
        ToastVariant::Default => (INK_MUTED, LucideIcon::Info),
        ToastVariant::Accent => (BLUE_600, LucideIcon::Sparkles),
        ToastVariant::Success => (SUCCESS, LucideIcon::CircleCheck),
        ToastVariant::Warning => (WARNING, LucideIcon::TriangleAlert),
        ToastVariant::Danger => (DANGER, LucideIcon::CircleX),
    }
}

fn toast_surface(accent: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: Color::from_rgba(accent.r, accent.g, accent.b, 0.18),
            width: 1.0,
            radius: RADIUS_INNER.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.16),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 26.0,
        },
        ..container::Style::default()
    }
}

/// An interactive, temporary notification displayed by a toast region.
pub fn toast<'a, Message>(
    title: &'a str,
    description: &'a str,
    variant: ToastVariant,
    action: Option<(&'a str, Message)>,
    on_close: Message,
    on_interact: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let (accent, glyph) = toast_variant_style(variant);
    let mut trailing = row![].spacing(4).align_y(iced::Alignment::Center);

    if let Some((label, on_action)) = action {
        trailing = trailing.push(
            button(
                container(
                    text(label)
                        .size(11)
                        .font(crate::fonts::MEDIUM)
                        .line_height(iced::widget::text::LineHeight::Absolute(Pixels(16.0))),
                )
                .height(Fill)
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center),
            )
            .on_press(on_action)
            .height(30)
            .padding([0, 10])
            .style(button_style(ButtonVariant::Secondary)),
        );
    }

    trailing = trailing.push(
        button(centered_button_icon(LucideIcon::X, 14, INK_MUTED))
            .on_press(on_close)
            .width(30)
            .height(30)
            .padding(0)
            .style(button_style(ButtonVariant::Ghost)),
    );

    mouse_area(
        container(
            row![
                container(crate::icons::icon(glyph, 17, accent))
                    .width(28)
                    .height(28)
                    .align_x(iced::Alignment::Center)
                    .align_y(iced::Alignment::Center),
                column![
                    text(title).size(13).font(crate::fonts::MEDIUM).color(
                        if matches!(variant, ToastVariant::Default) {
                            INK
                        } else {
                            accent
                        }
                    ),
                    text(description)
                        .size(11)
                        .font(crate::fonts::REGULAR)
                        .color(INK_MUTED),
                ]
                .spacing(2)
                .width(Fill),
                trailing,
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center),
        )
        .width(420)
        .padding([11, 13])
        .style(toast_surface(accent)),
    )
    .on_press(on_interact)
    .into()
}

/// Positions a stack of toasts at one of six viewport edges.
pub fn toast_region<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    placement: ToastPlacement,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let (horizontal, vertical) = placement.alignment();
    container(content)
        .width(Fill)
        .height(Fill)
        .align_x(horizontal)
        .align_y(vertical)
        .padding(24)
        .into()
}

fn modal_backdrop(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgba(
            INK.r, INK.g, INK.b, 0.38,
        ))),
        border: Border::default(),
        shadow: Shadow::default(),
        ..button::Style::default()
    }
}

fn modal_surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: LINE,
            width: 1.0,
            radius: RADIUS_PANEL.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.20),
            offset: Vector::new(0.0, 16.0),
            blur_radius: 40.0,
        },
        ..container::Style::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalModalOptions {
    close_on_backdrop: bool,
    show_close_button: bool,
}

impl GlobalModalOptions {
    pub const fn close_on_backdrop(mut self, close_on_backdrop: bool) -> Self {
        self.close_on_backdrop = close_on_backdrop;
        self
    }

    pub const fn confirmation() -> Self {
        Self {
            close_on_backdrop: false,
            show_close_button: false,
        }
    }
}

impl Default for GlobalModalOptions {
    fn default() -> Self {
        Self {
            close_on_backdrop: true,
            show_close_button: true,
        }
    }
}

/// A global modal shell. Clicking the backdrop closes it by default.
#[allow(clippy::too_many_arguments)]
pub fn global_modal<'a, Message>(
    title: &'a str,
    description: &'a str,
    body: impl Into<Element<'a, Message>>,
    cancel_label: &'a str,
    confirm_label: &'a str,
    destructive: bool,
    on_cancel: Message,
    on_confirm: Message,
    on_interact: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    global_modal_with_options(
        title,
        description,
        body,
        cancel_label,
        confirm_label,
        destructive,
        on_cancel,
        on_confirm,
        on_interact,
        GlobalModalOptions::default(),
    )
}

/// A global modal shell with configurable dismissal behavior.
#[allow(clippy::too_many_arguments)]
pub fn global_modal_with_options<'a, Message>(
    title: &'a str,
    description: &'a str,
    body: impl Into<Element<'a, Message>>,
    cancel_label: &'a str,
    confirm_label: &'a str,
    destructive: bool,
    on_cancel: Message,
    on_confirm: Message,
    on_interact: Message,
    options: GlobalModalOptions,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut header = row![
        column![
            text(title).size(18).font(crate::fonts::BOLD).color(INK),
            text(description)
                .size(12)
                .font(crate::fonts::REGULAR)
                .color(INK_MUTED)
        ]
        .spacing(4)
        .width(Fill),
    ]
    .spacing(16)
    .align_y(iced::Alignment::Center);

    if options.show_close_button {
        header = header.push(
            button(centered_button_icon(LucideIcon::X, 17, INK_MUTED))
                .on_press(on_cancel.clone())
                .width(32)
                .height(32)
                .padding(0)
                .style(button_style(ButtonVariant::Ghost)),
        );
    }

    let dialog = mouse_area(
        container(
            column![
                header,
                rule::horizontal(1),
                body.into(),
                rule::horizontal(1),
                row![
                    space::horizontal(),
                    button(text(cancel_label).size(13).font(crate::fonts::MEDIUM))
                        .on_press(on_cancel.clone())
                        .height(CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(button_style(ButtonVariant::Secondary)),
                    button(text(confirm_label).size(13).font(crate::fonts::MEDIUM))
                        .on_press(on_confirm)
                        .height(CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(button_style(if destructive {
                            ButtonVariant::Destructive
                        } else {
                            ButtonVariant::Primary
                        })),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
            ]
            .spacing(16),
        )
        .width(440)
        .padding(20)
        .style(modal_surface),
    )
    .on_press(on_interact.clone());

    let backdrop_action = if options.close_on_backdrop {
        on_cancel
    } else {
        on_interact
    };

    stack![
        button(space::Space::new())
            .on_press(backdrop_action)
            .width(Fill)
            .height(Fill)
            .padding(0)
            .style(modal_backdrop),
        container(dialog)
            .width(Fill)
            .height(Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .padding(24),
    ]
    .width(Fill)
    .height(Fill)
    .into()
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
    use super::{
        BadgePosition, CardVariant, GlobalLayer, GlobalModalOptions, HeroSlider, INK, NAVY_950,
        PopupPlacement, SLIDER_HANDLE_RADIUS, SLIDER_HEIGHT, SLIDER_WIDTH, SUCCESS, SWITCH_PADDING,
        SWITCH_THUMB_SIZE, SWITCH_WIDTH, ToastPlacement, WHITE, badge_offset, popup_origin,
        readable_on, switch_thumb_offset, translated_popup_placement,
    };
    use iced::{Point, Rectangle, Size, Vector};

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

    #[test]
    fn switch_thumb_animates_between_inner_edges() {
        let start = switch_thumb_offset(0.0);
        let middle = switch_thumb_offset(0.5);
        let end = switch_thumb_offset(1.0);

        assert_eq!(start, SWITCH_PADDING);
        assert_eq!(end, SWITCH_WIDTH - SWITCH_PADDING - SWITCH_THUMB_SIZE);
        assert!(start < middle && middle < end);
    }

    #[test]
    fn badge_positions_extend_from_each_anchor_corner() {
        assert_eq!(
            badge_offset(BadgePosition::TopRight, 20.0),
            Vector::new(10.0, -10.0)
        );
        assert_eq!(
            badge_offset(BadgePosition::TopLeft, 20.0),
            Vector::new(-10.0, -10.0)
        );
        assert_eq!(
            badge_offset(BadgePosition::BottomRight, 20.0),
            Vector::new(10.0, 10.0)
        );
        assert_eq!(
            badge_offset(BadgePosition::BottomLeft, 20.0),
            Vector::new(-10.0, 10.0)
        );
    }

    #[test]
    fn solid_chip_uses_the_higher_contrast_foreground() {
        assert_eq!(readable_on(SUCCESS), INK);
        assert_eq!(readable_on(NAVY_950), WHITE);
    }

    #[test]
    fn popup_flips_and_clamps_inside_the_viewport() {
        let viewport = Size::new(800.0, 600.0);
        let menu = Size::new(208.0, 120.0);
        let upper_target = Rectangle::new(Point::new(40.0, 40.0), Size::new(176.0, 36.0));
        let lower_target = Rectangle::new(Point::new(40.0, 540.0), Size::new(176.0, 36.0));

        assert_eq!(
            popup_origin(PopupPlacement::Below, upper_target, menu, viewport),
            Point::new(40.0, 82.0)
        );
        assert_eq!(
            popup_origin(PopupPlacement::Below, lower_target, menu, viewport),
            Point::new(40.0, 414.0)
        );
        assert_eq!(
            popup_origin(
                PopupPlacement::Cursor(Point::new(790.0, 590.0)),
                upper_target,
                menu,
                viewport,
            ),
            Point::new(584.0, 472.0)
        );
    }

    #[test]
    fn context_menu_cursor_is_translated_out_of_scroll_coordinates() {
        assert_eq!(
            translated_popup_placement(
                PopupPlacement::Cursor(Point::new(320.0, 740.0)),
                Vector::new(0.0, -500.0),
            ),
            PopupPlacement::Cursor(Point::new(320.0, 240.0))
        );
        assert_eq!(
            translated_popup_placement(PopupPlacement::Below, Vector::new(0.0, -500.0)),
            PopupPlacement::Below
        );
    }

    #[test]
    fn global_layers_are_above_framework_owned_overlays() {
        assert!(GlobalLayer::Modal.index() > f32::MAX);
        assert!(GlobalLayer::Toast.index() > f32::MAX);
        assert!(GlobalLayer::Message.index() > f32::MAX);
    }

    #[test]
    fn card_and_toast_defaults_match_standard_usage() {
        assert_eq!(CardVariant::default(), CardVariant::Default);
        assert_eq!(ToastPlacement::default(), ToastPlacement::Bottom);
        assert_eq!(
            ToastPlacement::TopEnd.alignment(),
            (iced::Alignment::End, iced::Alignment::Start)
        );
        assert_eq!(
            ToastPlacement::BottomStart.alignment(),
            (iced::Alignment::Start, iced::Alignment::End)
        );
    }

    #[test]
    fn modal_options_default_to_backdrop_dismissal() {
        let defaults = GlobalModalOptions::default();
        assert!(defaults.close_on_backdrop);
        assert!(defaults.show_close_button);

        let persistent = defaults.close_on_backdrop(false);
        assert!(!persistent.close_on_backdrop);
        assert!(persistent.show_close_button);

        let confirmation = GlobalModalOptions::confirmation();
        assert!(!confirmation.close_on_backdrop);
        assert!(!confirmation.show_close_button);
    }
}
