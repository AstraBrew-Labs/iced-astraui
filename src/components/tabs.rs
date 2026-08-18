pub fn tab(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    tab_animated(active, 0.0)
}

/// The visual treatment used by a [`Tabs`] control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsVariant {
    /// A connected surface with a raised selected tab.
    #[default]
    Primary,
    /// A quieter list with an underline indicator.
    Secondary,
}

/// The direction in which a tab list and its panels are laid out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// One tab label and its associated panel.
///
/// `TabItem` is intentionally controlled by the application: the selected
/// index lives in the application state and is passed to [`Tabs::selected`].
pub struct TabItem<'a, Message> {
    label: Element<'a, Message>,
    panel: Element<'a, Message>,
    id: Option<String>,
    disabled: bool,
    separator: bool,
    on_select: Option<Message>,
}

/// Short alias matching HeroUI's `Tabs.Tab` terminology.
pub type Tab<'a, Message> = TabItem<'a, Message>;

impl<'a, Message> TabItem<'a, Message>
where
    Message: 'a,
{
    /// Creates a tab from its label and panel content.
    pub fn new(
        label: impl Into<Element<'a, Message>>,
        panel: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            label: label.into(),
            panel: panel.into(),
            id: None,
            disabled: false,
            separator: false,
            on_select: None,
        }
    }

    /// Gives the tab a stable identifier for application-level routing.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Prevents the tab from being selected while keeping it visible.
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Adds a separator before this tab. The first tab ignores this setting.
    pub const fn separator(mut self, separator: bool) -> Self {
        self.separator = separator;
        self
    }

    /// Supplies a message for this item when the parent has no callback.
    pub fn on_select(mut self, message: Message) -> Self {
        self.on_select = Some(message);
        self
    }

    /// Returns the optional stable identifier.
    pub fn tab_id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Returns whether this item is disabled.
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }
}

/// A controlled, accessible tab list with primary and secondary variants.
///
/// The component renders only the selected panel. Callers own the selected
/// index and can either provide one callback with [`Tabs::on_selection_change`]
/// or attach a message to each [`TabItem`] with [`TabItem::on_select`].
pub struct Tabs<'a, Message> {
    items: Vec<TabItem<'a, Message>>,
    selected: usize,
    variant: TabsVariant,
    orientation: TabsOrientation,
    width: Length,
    on_selection_change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> Tabs<'a, Message>
where
    Message: 'a,
{
    /// Creates a tabs control with the first item selected.
    pub fn new(items: Vec<TabItem<'a, Message>>) -> Self {
        Self {
            items,
            selected: 0,
            variant: TabsVariant::Primary,
            orientation: TabsOrientation::Horizontal,
            width: Fill,
            on_selection_change: None,
        }
    }

    /// Sets the selected item. Out-of-range values are clamped at render time.
    pub const fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    /// Alias for [`Tabs::selected`] that reads naturally at call sites.
    pub const fn selected_index(self, selected: usize) -> Self {
        self.selected(selected)
    }

    pub const fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Emits the item index whenever a non-disabled tab is pressed.
    pub fn on_selection_change(
        mut self,
        on_selection_change: impl Fn(usize) -> Message + 'a,
    ) -> Self {
        self.on_selection_change = Some(Box::new(on_selection_change));
        self
    }
}

/// Creates a controlled tabs element without keeping the builder value.
pub fn tabs<'a, Message>(
    items: Vec<TabItem<'a, Message>>,
    selected: usize,
    on_selection_change: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    Tabs::new(items)
        .selected(selected)
        .on_selection_change(on_selection_change)
        .into()
}

fn tabs_list_style(variant: TabsVariant) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: (variant == TabsVariant::Primary)
            .then_some(Background::Color(SURFACE_ALT)),
        border: Border {
            color: if variant == TabsVariant::Secondary {
                LINE
            } else {
                Color::TRANSPARENT
            },
            width: f32::from(variant == TabsVariant::Secondary),
            radius: RADIUS_FIELD.into(),
        },
        ..container::Style::default()
    }
}

fn tabs_tab_style(
    active: bool,
    variant: TabsVariant,
    disabled: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        let background = if disabled {
            None
        } else if active && variant == TabsVariant::Primary {
            Some(Background::Color(SURFACE))
        } else if hovered || pressed {
            Some(Background::Color(Color::from_rgba(
                BLUE_600.r,
                BLUE_600.g,
                BLUE_600.b,
                if pressed { 0.14 } else { 0.08 },
            )))
        } else {
            None
        };

        button::Style {
            background,
            text_color: if disabled {
                INK_SUBTLE
            } else if active && variant == TabsVariant::Secondary {
                BLUE_700
            } else {
                INK_MUTED
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: RADIUS_CONTROL.into(),
            },
            shadow: if active && variant == TabsVariant::Primary {
                Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 5.0,
                }
            } else {
                Shadow::default()
            },
            ..button::Style::default()
        }
    }
}

fn tabs_separator_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(LINE)),
        ..container::Style::default()
    }
}

fn tabs_indicator_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BLUE_600)),
        border: Border {
            radius: 2.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

impl<'a, Message> From<Tabs<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(tabs: Tabs<'a, Message>) -> Self {
        let Tabs {
            items,
            selected,
            variant,
            orientation,
            width,
            on_selection_change,
        } = tabs;
        let selected = selected.min(items.len().saturating_sub(1));
        let item_count = items.len();
        let callback = on_selection_change.as_ref();
        let mut labels = Vec::with_capacity(item_count);
        let mut panel = None;

        for (index, item) in items.into_iter().enumerate() {
            let TabItem {
                label: item_label,
                panel: item_panel,
                disabled,
                separator,
                on_select,
                ..
            } = item;
            if index == selected {
                panel = Some(item_panel);
            }
            let centered_label = container(item_label)
                .height(Fill)
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center);
            let mut label = button(centered_label)
                .height(CONTROL_HEIGHT_MD)
                .padding([0, 14])
                .style(tabs_tab_style(index == selected, variant, disabled));
            if !disabled {
                let message = callback.map(|callback| callback(index)).or(on_select);
                label = label.on_press_maybe(message);
            }

            let label: Element<'a, Message> = if variant == TabsVariant::Secondary
                && index == selected
            {
                let indicator = match orientation {
                    TabsOrientation::Horizontal => container(space::vertical().height(2)).width(Fill),
                    TabsOrientation::Vertical => container(space::horizontal().width(2)).height(Fill),
                }
                .style(tabs_indicator_style);
                match orientation {
                    TabsOrientation::Horizontal => {
                        iced::widget::column![label, indicator].into()
                    }
                    TabsOrientation::Vertical => row![label, indicator].into(),
                }
            } else {
                label.into()
            };

            let label: Element<'a, Message> = if separator && index > 0 {
                let separator = match orientation {
                    TabsOrientation::Horizontal => container(space::vertical().height(18)),
                    TabsOrientation::Vertical => container(space::horizontal().width(18)),
                }
                .style(tabs_separator_style);
                match orientation {
                    TabsOrientation::Horizontal => row![separator, label].spacing(4).into(),
                    TabsOrientation::Vertical => iced::widget::column![separator, label]
                        .spacing(4)
                        .into(),
                }
            } else {
                label
            };
            labels.push(label);
        }

        let list: Element<'a, Message> = match orientation {
            TabsOrientation::Horizontal => row(labels).spacing(2).into(),
            TabsOrientation::Vertical => iced::widget::column(labels).spacing(2).into(),
        };
        let list = container(list)
            .padding(if variant == TabsVariant::Primary { 4 } else { 0 })
            .width(if orientation == TabsOrientation::Horizontal {
                Fill
            } else {
                Length::Shrink
            })
            .style(tabs_list_style(variant));

        let panel = panel.map(|panel| container(panel).width(Fill).padding([14, 0]));
        let content: Element<'a, Message> = match (orientation, panel) {
            (TabsOrientation::Horizontal, Some(panel)) => {
                iced::widget::column![list, panel].spacing(2).into()
            }
            (TabsOrientation::Vertical, Some(panel)) => row![list, panel].spacing(16).into(),
            (_, None) => list.into(),
        };

        container(content).width(width).into()
    }
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
        AccordionSelectionMode, AlertDialog, AlertKind, Avatar, AvatarColor, AvatarShape,
        AvatarSize, BadgePosition, CardVariant, Drawer, DrawerBackdrop, DrawerOptions,
        DrawerPlacement, GlobalLayer, GlobalModalOptions, HeroSlider, INK,
        NAVY_950, PaginationItem, PopupPlacement, ProgressBar, ProgressBarColor, ProgressBarSize,
        ProgressCircle, ProgressCircleColor, ProgressCircleSize, SLIDER_HANDLE_RADIUS,
        SLIDER_HEIGHT, SLIDER_WIDTH, SUCCESS, SWITCH_PADDING, SWITCH_THUMB_SIZE, SWITCH_WIDTH,
        ScrollMetrics, ScrollShadowEdges, ScrollShadowOrientation, SelectableTypography,
        SelectableTypographyState, Separator, SeparatorOrientation, SeparatorVariant,
        InputOtp, InputOtpVariant, Label, ListBox, ListBoxItem, ListBoxItemVariant,
        ListBoxSelectionMode, Surface, SurfaceVariant, TabItem, Tabs, TabsOrientation, TabsVariant,
        Kbd, KbdKey, KbdPlatform, KbdVariant, MessageKind,
        TextArea, TextAreaVariant, ToastPlacement, Typography, TypographyType, TypographyWeight,
        ToastVariant, WHITE, INK_SUBTLE, SURFACE, SURFACE_ALT,
        automatic_scroll_shadow_edges, avatar_initial, badge_offset, indeterminate_segment,
        next_navigation_index, pagination_items, popup_origin, previous_navigation_index,
        progress_circle_arc, progress_fraction, readable_on, switch_thumb_offset,
        toggle_accordion_item, translated_popup_placement, translated_typography_context_position,
        typography_context_menu_text_layout, typography_font, tabs_tab_style, surface_style,
        drawer_transition_offset, kbd_style,
    };
    use iced::{Color, Point, Rectangle, Size, Vector};
    use iced::widget::text;
    use iced::widget::text_editor;

    fn test_slider(value: f32) -> HeroSlider<'static, ()> {
        HeroSlider {
            range: 0.0..=100.0,
            value,
            step: 0.1,
            on_change: Box::new(|_| ()),
        }
    }

    #[test]
    fn avatar_shapes_sizes_and_default_fallback_are_stable() {
        let avatar: Avatar<'_, ()> = Avatar::new("Astra");
        assert_eq!(avatar.shape, AvatarShape::Circle);
        assert_eq!(avatar.size, AvatarSize::Medium);
        assert_eq!(avatar.color, AvatarColor::Accent);
        assert!(avatar.image.is_none());
        assert!(avatar.fallback.is_none());
        assert_eq!(avatar_initial("Astra"), "A");
        assert_eq!(avatar_initial("  中文"), "中");
        assert_eq!(avatar_initial("  "), "?");

        let circle = AvatarShape::Circle.radius(40.0);
        let rounded = AvatarShape::Rounded.radius(40.0);
        let square = AvatarShape::Square.radius(40.0);
        assert_eq!(
            (
                circle.top_left,
                circle.top_right,
                circle.bottom_right,
                circle.bottom_left
            ),
            (20.0, 20.0, 20.0, 20.0)
        );
        assert_eq!(
            (
                rounded.top_left,
                rounded.top_right,
                rounded.bottom_right,
                rounded.bottom_left,
            ),
            (10.0, 10.0, 10.0, 10.0)
        );
        assert_eq!(square, iced::border::Radius::default());
        assert_eq!(AvatarSize::Small.diameter(), 32.0);
        assert_eq!(AvatarSize::Large.diameter(), 48.0);
    }

    #[test]
    fn alert_statuses_map_to_distinct_indicators() {
        let theme = super::app_theme();
        let info = super::alert(AlertKind::Info)(&theme);
        let warning = super::alert(AlertKind::Warning)(&theme);

        assert_eq!(info.border.width, 1.0);
        assert_eq!(warning.border.width, 1.0);
        assert_ne!(AlertKind::Info.accent(), AlertKind::Success.accent());
        assert_ne!(AlertKind::Warning.accent(), AlertKind::Danger.accent());
        assert_ne!(info.border.color, warning.border.color);
    }

    #[test]
    fn accordion_toggle_supports_single_and_multiple_selection() {
        let mut single = [true, false, false];
        toggle_accordion_item(&mut single, 1, AccordionSelectionMode::Single);
        assert_eq!(single, [false, true, false]);
        toggle_accordion_item(&mut single, 1, AccordionSelectionMode::Single);
        assert_eq!(single, [false, false, false]);

        let mut multiple = [true, false, false];
        toggle_accordion_item(&mut multiple, 1, AccordionSelectionMode::Multiple);
        assert_eq!(multiple, [true, true, false]);
        toggle_accordion_item(&mut multiple, usize::MAX, AccordionSelectionMode::Multiple);
        assert_eq!(multiple, [true, true, false]);
    }

    #[test]
    fn alert_dialog_defaults_to_required_danger_confirmation() {
        let dialog = AlertDialog::new("Delete?", "This cannot be undone.", (), (), ())
            .confirm_label("Delete")
            .destructive(true);

        assert_eq!(dialog.status, AlertKind::Danger);
        assert_eq!(dialog.cancel_label, "Cancel");
        assert_eq!(dialog.confirm_label, "Delete");
        assert!(dialog.destructive);
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
    fn progress_bar_normalizes_custom_ranges_and_clamps_values() {
        let progress = ProgressBar::new(750.0).range(0.0..=1000.0);

        assert_eq!(progress.fraction(), 0.75);
        assert_eq!(progress.formatted_value(), "75%");
        assert_eq!(progress_fraction(-20.0, 0.0, 100.0), 0.0);
        assert_eq!(progress_fraction(120.0, 0.0, 100.0), 1.0);
        assert_eq!(progress_fraction(50.0, 100.0, 0.0), 0.0);
    }

    #[test]
    fn progress_bar_variants_match_heroui_sizes_and_semantic_colors() {
        assert_eq!(ProgressBarSize::Small.girth(), 4.0);
        assert_eq!(ProgressBarSize::Medium.girth(), 8.0);
        assert_eq!(ProgressBarSize::Large.girth(), 12.0);
        assert_eq!(ProgressBarColor::Accent.fill(), super::BLUE_600);
        assert_eq!(ProgressBarColor::Success.fill(), super::SUCCESS);
        assert_eq!(ProgressBarColor::Warning.fill(), super::WARNING);
        assert_eq!(ProgressBarColor::Danger.fill(), super::DANGER);
    }

    #[test]
    fn indeterminate_progress_segment_moves_through_the_clipped_track() {
        let (start, width) = indeterminate_segment(100.0, 0.0);
        let (middle, middle_width) = indeterminate_segment(100.0, 0.5);
        let (late, _) = indeterminate_segment(100.0, 0.9);

        assert_eq!((start, width), (-40.0, 40.0));
        assert_eq!(middle_width, 40.0);
        assert!(middle > 0.0);
        assert!(late > 100.0);
    }

    #[test]
    fn progress_circle_matches_heroui_sizes_colors_and_custom_ranges() {
        let progress = ProgressCircle::new(750.0).range(0.0..=1000.0);

        assert_eq!(progress.fraction(), 0.75);
        assert_eq!(ProgressCircleSize::Small.diameter(), 20.0);
        assert_eq!(ProgressCircleSize::Medium.diameter(), 28.0);
        assert_eq!(ProgressCircleSize::Large.diameter(), 36.0);
        assert_eq!(ProgressCircleColor::default(), ProgressCircleColor::Accent);
    }

    #[test]
    fn progress_circle_uses_top_origin_and_a_quarter_indeterminate_arc() {
        let (determinate_start, determinate_sweep) = progress_circle_arc(0.5, false, 0.0);
        let (indeterminate_start, indeterminate_sweep) = progress_circle_arc(0.0, true, 0.25);

        assert!((determinate_start + std::f32::consts::FRAC_PI_2).abs() < f32::EPSILON);
        assert!((determinate_sweep - std::f32::consts::PI).abs() < f32::EPSILON);
        assert!(indeterminate_start.abs() < f32::EPSILON);
        assert!((indeterminate_sweep - std::f32::consts::FRAC_PI_2).abs() < f32::EPSILON);
    }

    #[test]
    fn separator_defaults_and_variants_match_heroui_geometry() {
        let separator = Separator::new();

        assert_eq!(separator.orientation, SeparatorOrientation::Horizontal);
        assert_eq!(separator.variant, SeparatorVariant::Default);
        assert_eq!(separator.thickness, 1.0);
        assert_eq!(SeparatorVariant::Default.color(), super::LINE);
        assert_ne!(
            SeparatorVariant::Default.color(),
            SeparatorVariant::Secondary.color()
        );
        assert_ne!(
            SeparatorVariant::Secondary.color(),
            SeparatorVariant::Tertiary.color()
        );
        assert_eq!(
            Separator::new()
                .orientation(SeparatorOrientation::Vertical)
                .thickness(0.0)
                .thickness,
            1.0
        );
    }

    #[test]
    fn typography_scale_uses_heroui_sizes_and_local_fonts() {
        let h1 = TypographyType::H1.metrics();
        let body = TypographyType::Body.metrics();
        let small = TypographyType::BodySmall.metrics();
        let caption = TypographyType::BodyExtraSmall.metrics();
        let code = TypographyType::Code.metrics();

        assert_eq!((h1.size, h1.line_height), (36.0, 40.0));
        assert_eq!((body.size, body.line_height), (16.0, 28.0));
        assert_eq!((small.size, small.line_height), (14.0, 21.0));
        assert_eq!((caption.size, caption.line_height), (12.0, 15.0));
        assert_eq!((code.size, code.line_height), (14.0, 20.0));
        assert_eq!(h1.font, crate::fonts::BOLD);
        assert_eq!(body.font, crate::fonts::REGULAR);
        assert_eq!(code.font, crate::fonts::MEDIUM);
        assert_eq!(
            typography_font(body.font, Some(TypographyWeight::Semibold)),
            crate::fonts::BOLD
        );
    }

    #[test]
    fn typography_without_copy_feedback_keeps_the_original_message_api() {
        struct NonCloneMessage;

        let _: iced::Element<'_, NonCloneMessage> = Typography::new("Selectable text").into();
    }

    #[test]
    fn typography_selection_preserves_utf8_boundaries_for_copying() {
        let typography = SelectableTypography {
            content: "A中文B".to_owned(),
            metrics: TypographyType::Body.metrics(),
            color: super::INK,
            align: iced::advanced::text::Alignment::Left,
            wrapping: iced::advanced::text::Wrapping::Word,
            width: iced::Length::Shrink,
            on_copy: None::<Box<dyn Fn()>>,
        };
        let mut state = SelectableTypographyState::default();

        state.begin_selection(1);
        assert!(state.extend_selection(7));
        assert!(state.dragging);
        assert_eq!(typography.selected_text(&state), Some("中文"));
        state.begin_selection(7);
        assert!(state.extend_selection(1));
        assert_eq!(typography.selected_text(&state), Some("中文"));
        assert_eq!(typography.char_boundary_at_or_before(2), 1);
        assert_eq!(typography.char_boundary_at_or_before(6), 4);
    }

    #[test]
    fn typography_selection_maps_logical_lines_to_full_text_offsets() {
        let typography = SelectableTypography {
            content: "第一行\nSecond line\n第三行".to_owned(),
            metrics: TypographyType::Body.metrics(),
            color: super::INK,
            align: iced::advanced::text::Alignment::Left,
            wrapping: iced::advanced::text::Wrapping::Word,
            width: iced::Length::Fill,
            on_copy: None::<Box<dyn Fn()>>,
        };

        assert_eq!(typography.logical_line_start(0), 0);
        assert_eq!(typography.logical_line_start(1), "第一行\n".len());
        assert_eq!(
            typography.logical_line_start(2),
            "第一行\nSecond line\n".len()
        );
    }

    #[test]
    fn typography_context_menu_position_accounts_for_scroll_translation() {
        assert_eq!(
            translated_typography_context_position(
                Point::new(320.0, 740.0),
                Vector::new(0.0, -500.0),
            ),
            Point::new(320.0, 240.0)
        );
        assert_eq!(
            translated_typography_context_position(Point::new(18.0, 26.0), Vector::ZERO),
            Point::new(18.0, 26.0)
        );
    }

    #[test]
    fn typography_context_menu_text_uses_the_vertical_center_anchor() {
        let menu = Rectangle::new(Point::new(972.0, 303.0), Size::new(112.0, 36.0));
        let (text_bounds, text_origin) = typography_context_menu_text_layout(menu);

        assert_eq!(text_bounds, Size::new(88.0, 36.0));
        assert_eq!(text_origin, Point::new(984.0, 321.0));
    }

    #[test]
    fn scroll_shadow_visibility_tracks_start_middle_and_end_boundaries() {
        let metrics = |translation_y| ScrollMetrics {
            bounds: Rectangle::new(Point::ORIGIN, Size::new(200.0, 100.0)),
            content_bounds: Rectangle::new(Point::ORIGIN, Size::new(200.0, 300.0)),
            translation: Vector::new(0.0, translation_y),
        };

        assert_eq!(
            automatic_scroll_shadow_edges(metrics(0.0), ScrollShadowOrientation::Vertical, 0.0,),
            ScrollShadowEdges::After
        );
        assert_eq!(
            automatic_scroll_shadow_edges(metrics(100.0), ScrollShadowOrientation::Vertical, 0.0,),
            ScrollShadowEdges::Both
        );
        assert_eq!(
            automatic_scroll_shadow_edges(metrics(200.0), ScrollShadowOrientation::Vertical, 0.0,),
            ScrollShadowEdges::Before
        );
        assert_eq!(
            automatic_scroll_shadow_edges(
                ScrollMetrics {
                    bounds: Rectangle::new(Point::ORIGIN, Size::new(200.0, 100.0)),
                    content_bounds: Rectangle::new(Point::ORIGIN, Size::new(200.0, 100.0)),
                    translation: Vector::ZERO,
                },
                ScrollShadowOrientation::Vertical,
                0.0,
            ),
            ScrollShadowEdges::None
        );
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
    fn switch_hit_target_is_borderless() {
        let theme = super::app_theme();

        assert_eq!(
            super::switch_button(&theme, iced::widget::button::Status::Hovered)
                .border
                .width,
            0.0
        );
    }

    #[test]
    fn outline_button_keeps_a_visible_border_without_a_hover_fill() {
        let theme = super::app_theme();
        let active = super::button_style(super::ButtonVariant::Outline)(
            &theme,
            iced::widget::button::Status::Active,
        );
        let hovered = super::button_style(super::ButtonVariant::Outline)(
            &theme,
            iced::widget::button::Status::Hovered,
        );
        let primary = super::button_style(super::ButtonVariant::Primary)(
            &theme,
            iced::widget::button::Status::Active,
        );

        assert_eq!(
            (active.border.color, active.border.width),
            (super::LINE, 1.0)
        );
        assert_eq!(
            (hovered.border.color, hovered.border.width),
            (super::BLUE_500, 1.0)
        );
        assert_eq!(hovered.background, None);
        assert_eq!(primary.border.width, 0.0);
    }

    #[test]
    fn selection_controls_keep_visible_stateful_borders() {
        let theme = super::app_theme();
        let checkbox = super::checkbox_style(
            &theme,
            iced::widget::checkbox::Status::Active { is_checked: false },
        );
        let checked_checkbox = super::checkbox_style(
            &theme,
            iced::widget::checkbox::Status::Active { is_checked: true },
        );
        let radio = super::radio_style(
            &theme,
            iced::widget::radio::Status::Active { is_selected: false },
        );
        let selected_radio = super::radio_style(
            &theme,
            iced::widget::radio::Status::Active { is_selected: true },
        );

        assert_eq!(
            (checkbox.border.color, checkbox.border.width),
            (super::LINE, 1.0)
        );
        assert_eq!(checked_checkbox.border.color, super::BLUE_600);
        assert_eq!((radio.border_color, radio.border_width), (super::LINE, 1.0));
        assert_eq!(selected_radio.border_color, super::BLUE_600);
    }

    #[test]
    fn form_fields_keep_visible_stateful_borders() {
        let theme = super::app_theme();
        let input = super::text_input_style(&theme, iced::widget::text_input::Status::Active);
        let focused_input = super::text_input_style(
            &theme,
            iced::widget::text_input::Status::Focused { is_hovered: true },
        );
        let select = super::pick_list_style(&theme, iced::widget::pick_list::Status::Active);
        let opened_select = super::pick_list_style(
            &theme,
            iced::widget::pick_list::Status::Opened { is_hovered: true },
        );
        let menu = super::pick_list_menu_style(&theme);

        assert_eq!((input.border.color, input.border.width), (super::LINE, 1.0));
        assert_eq!(
            (focused_input.border.color, focused_input.border.width),
            (super::BLUE_600, 2.0)
        );
        assert_eq!(
            (select.border.color, select.border.width),
            (super::LINE, 1.0)
        );
        assert_eq!(
            (opened_select.border.color, opened_select.border.width),
            (super::BLUE_600, 2.0)
        );
        assert_eq!((menu.border.color, menu.border.width), (super::LINE, 1.0));
    }

    #[test]
    fn toggle_controls_use_one_subtle_outer_border() {
        let theme = super::app_theme();
        let standalone = super::toggle_button_style(
            false,
            super::ToggleButtonVariant::Default,
            super::GroupPosition::Standalone,
            super::Orientation::Horizontal,
        )(&theme, iced::widget::button::Status::Active);
        let joined_item = super::toggle_button_style(
            false,
            super::ToggleButtonVariant::Default,
            super::GroupPosition::First,
            super::Orientation::Horizontal,
        )(&theme, iced::widget::button::Status::Active);
        let group = super::toggle_button_group_surface(&theme);

        assert_eq!(
            (standalone.border.color, standalone.border.width),
            (super::LINE, 1.0)
        );
        assert_eq!(joined_item.border.width, 0.0);
        assert_eq!((group.border.color, group.border.width), (super::LINE, 1.0));
    }

    #[test]
    fn temporary_interactions_do_not_add_light_backgrounds() {
        let theme = super::app_theme();

        assert_eq!(
            super::button_style(super::ButtonVariant::Outline)(
                &theme,
                iced::widget::button::Status::Hovered,
            )
            .background,
            None
        );
        assert_eq!(
            super::text_input_style(
                &theme,
                iced::widget::text_input::Status::Focused { is_hovered: true },
            )
            .background,
            iced::Background::Color(super::SURFACE)
        );
        assert_eq!(
            super::pick_list_style(
                &theme,
                iced::widget::pick_list::Status::Opened { is_hovered: true },
            )
            .background,
            iced::Background::Color(super::SURFACE)
        );
        assert_eq!(
            super::switch_button(&theme, iced::widget::button::Status::Hovered).background,
            None
        );
        assert_eq!(
            super::checkbox_style(
                &theme,
                iced::widget::checkbox::Status::Hovered { is_checked: false },
            )
            .background,
            iced::Background::Color(super::SURFACE)
        );
        assert_eq!(
            super::radio_style(
                &theme,
                iced::widget::radio::Status::Hovered { is_selected: false },
            )
            .background,
            iced::Background::Color(super::SURFACE)
        );
    }

    #[test]
    fn disclosure_uses_one_connected_surface_without_shadows() {
        let theme = super::app_theme();
        let disclosure = super::disclosure_surface(&theme);
        let panel = super::disclosure_panel_surface(&theme);
        let expected = Some(iced::Background::Color(super::SURFACE));

        assert_eq!(disclosure.background, expected);
        assert_eq!(panel.background, expected);
        assert_eq!(
            (disclosure.border.color, disclosure.border.width),
            (super::LINE, 1.0)
        );
        assert_eq!((panel.border.color, panel.border.width), (super::LINE, 1.0));
        assert_eq!(disclosure.shadow, iced::Shadow::default());
        assert_eq!(panel.shadow, iced::Shadow::default());
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
        assert_eq!(ToastPlacement::default(), ToastPlacement::Top);
        assert_eq!(
            ToastPlacement::TopEnd.alignment(),
            (iced::Alignment::End, iced::Alignment::Start)
        );
        assert_eq!(
            ToastPlacement::BottomStart.alignment(),
            (iced::Alignment::Start, iced::Alignment::End)
        );
        assert_eq!(
            ToastPlacement::Top.transition_offset(0.0, false),
            Vector::new(0.0, -14.0)
        );
        assert_eq!(
            ToastPlacement::Top.transition_offset(0.0, true),
            Vector::new(0.0, -14.0)
        );
        assert_eq!(
            ToastPlacement::TopStart.transition_offset(0.0, false),
            Vector::new(-14.0, 0.0)
        );
        assert_eq!(
            ToastPlacement::TopStart.transition_offset(0.0, true),
            Vector::new(-14.0, 0.0)
        );
        assert_eq!(
            ToastPlacement::TopEnd.transition_offset(0.0, false),
            Vector::new(14.0, 0.0)
        );
        assert_eq!(
            ToastPlacement::TopEnd.transition_offset(0.0, true),
            Vector::new(14.0, 0.0)
        );

        let _: iced::Element<'static, ()> = super::global_message_animated(
            "Saved",
            "Your changes are ready.",
            MessageKind::Success,
            (),
            0.5,
        );
        let _: iced::Element<'static, ()> = super::global_message_animated_with_placement(
            "Saved",
            "Your changes are ready.",
            MessageKind::Success,
            (),
            ToastPlacement::TopEnd,
            0.5,
            false,
        );
        let _: iced::Element<'static, ()> = super::toast_animated(
            "Published",
            "The component is available.",
            ToastVariant::Success,
            None,
            (),
            (),
            0.5,
        );
    }

    #[test]
    fn keyboard_navigation_wraps_at_collection_edges() {
        assert_eq!(previous_navigation_index(0, 4), 3);
        assert_eq!(previous_navigation_index(2, 4), 1);
        assert_eq!(next_navigation_index(3, 4), 0);
        assert_eq!(next_navigation_index(1, 4), 2);
        assert_eq!(next_navigation_index(0, 0), 0);
    }

    #[test]
    fn pagination_items_follow_the_heroui_ellipsis_window() {
        use PaginationItem::{Ellipsis, Page};

        assert_eq!(pagination_items(0, 0), vec![Page(1)]);
        assert_eq!(
            pagination_items(1, 12),
            vec![Page(1), Page(2), Ellipsis, Page(12)]
        );
        assert_eq!(
            pagination_items(6, 12),
            vec![
                Page(1),
                Ellipsis,
                Page(5),
                Page(6),
                Page(7),
                Ellipsis,
                Page(12),
            ]
        );
        assert_eq!(
            pagination_items(12, 12),
            vec![Page(1), Ellipsis, Page(11), Page(12)]
        );
        assert_eq!(
            pagination_items(4, 7),
            (1..=7).map(Page).collect::<Vec<_>>()
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

        let dialog = AlertDialog::new("Title", "Description", (), (), ())
            .animation_progress(0.35);
        assert_eq!(dialog.animation_progress, 0.35);
    }

    #[test]
    fn drawer_defaults_match_edge_aligned_overlay_usage() {
        let options = DrawerOptions::default();
        assert_eq!(options.placement, DrawerPlacement::Bottom);
        assert_eq!(options.backdrop, DrawerBackdrop::Opaque);
        assert!(options.close_on_backdrop);
        assert!(options.show_close_button);
        assert!(!options.show_handle);
        assert_eq!(options.size, 360.0);

        let drawer: Drawer<'static, ()> = Drawer::new("Details", text("Body"), (), ())
            .placement(DrawerPlacement::Right)
            .backdrop(DrawerBackdrop::Blur)
            .show_handle(true)
            .close_on_backdrop(false)
            .animation_progress(0.4);
        assert_eq!(drawer.animation_progress, 0.4);
        let _: iced::Element<'static, ()> = drawer.into();

        assert_eq!(
            drawer_transition_offset(DrawerPlacement::Right, 360.0, 0.0),
            Vector::new(360.0, 0.0)
        );
        assert_eq!(
            drawer_transition_offset(DrawerPlacement::Top, 240.0, 0.5),
            Vector::new(0.0, -120.0)
        );
    }

    #[test]
    fn tabs_defaults_match_heroui_primary_horizontal_behavior() {
        let tabs: Tabs<'static, ()> = Tabs::new(vec![TabItem::new(text("One"), text("Panel"))]);

        assert_eq!(tabs.selected, 0);
        assert_eq!(tabs.variant, TabsVariant::Primary);
        assert_eq!(tabs.orientation, TabsOrientation::Horizontal);
        assert_eq!(tabs.items[0].tab_id(), None);
        assert!(!tabs.items[0].is_disabled());
    }

    #[test]
    fn tabs_preserve_disabled_and_separator_item_metadata() {
        let item: TabItem<'static, ()> = TabItem::new(text("Reports"), text("Report panel"))
            .id("reports")
            .disabled(true)
            .separator(true);

        assert_eq!(item.tab_id(), Some("reports"));
        assert!(item.is_disabled());
        assert!(item.separator);
    }

    #[test]
    fn tabs_styles_distinguish_primary_secondary_and_disabled_states() {
        let theme = super::app_theme();
        let primary = tabs_tab_style(true, TabsVariant::Primary, false)(
            &theme,
            iced::widget::button::Status::Active,
        );
        let secondary = tabs_tab_style(true, TabsVariant::Secondary, false)(
            &theme,
            iced::widget::button::Status::Active,
        );
        let disabled = tabs_tab_style(false, TabsVariant::Primary, true)(
            &theme,
            iced::widget::button::Status::Disabled,
        );

        assert_eq!(primary.background, Some(iced::Background::Color(SURFACE)));
        assert_eq!(secondary.border.color, Color::TRANSPARENT);
        assert_eq!(secondary.border.width, 0.0);
        assert_eq!(disabled.text_color, INK_SUBTLE);
        assert_eq!(disabled.background, None);
    }

    #[test]
    fn surface_variants_map_to_semantic_backgrounds() {
        let theme = super::app_theme();
        assert_eq!(surface_style(SurfaceVariant::Default)(&theme).background, Some(iced::Background::Color(SURFACE)));
        assert_eq!(surface_style(SurfaceVariant::Secondary)(&theme).background, Some(iced::Background::Color(SURFACE_ALT)));
        assert_eq!(surface_style(SurfaceVariant::Transparent)(&theme).background, None);
        let _: iced::Element<'static, ()> = Surface::new(text("content")).variant(SurfaceVariant::Tertiary).into();
    }

    #[test]
    fn labels_expose_required_disabled_invalid_metadata() {
        let label = Label::new("Email")
            .for_id("email")
            .required(true)
            .disabled(true)
            .invalid(true);
        assert_eq!(label.label_id(), Some("email"));
        assert!(label.is_required());
        assert!(label.is_disabled());
        assert!(label.is_invalid());
        let _: iced::Element<'static, ()> = label.into();
    }

    #[test]
    fn text_area_and_otp_defaults_match_form_usage() {
        let content = text_editor::Content::with_text("Notes");
        let area: TextArea<'static, ()> = TextArea::new(Box::leak(Box::new(content)))
            .rows(0)
            .variant(TextAreaVariant::Secondary);
        assert_eq!(area.rows, 1);
        assert_eq!(area.variant, TextAreaVariant::Secondary);

        let otp: InputOtp<'static, ()> = InputOtp::new("12", 0, |_| ())
            .variant(InputOtpVariant::Secondary)
            .disabled(true)
            .separator_after(1);
        assert_eq!(otp.max_length(), 1);
        assert_eq!(otp.variant, InputOtpVariant::Secondary);
        assert!(otp.disabled);
        let _: iced::Element<'static, ()> = otp.into();
    }

    #[test]
    fn otp_focus_moves_forward_and_back_without_populated_value_bias() {
        use super::{input_otp_focus_index, InputOtpAction};

        assert_eq!(input_otp_focus_index(0, 1, 6, InputOtpAction::Input), 1);
        assert_eq!(input_otp_focus_index(4, 1, 6, InputOtpAction::Input), 5);
        assert_eq!(input_otp_focus_index(5, 1, 6, InputOtpAction::Input), 5);
        assert_eq!(input_otp_focus_index(3, 0, 6, InputOtpAction::Backspace), 2);
        assert_eq!(input_otp_focus_index(0, 0, 6, InputOtpAction::Backspace), 0);
    }

    #[test]
    fn otp_change_exposes_stable_focus_id() {
        use super::{InputOtpAction, InputOtpChange};

        let change = InputOtpChange {
            value: "12".to_owned(),
            index: 1,
            action: InputOtpAction::Input,
            focus_index: 2,
            focus_id: "verification-otp-2".to_owned(),
        };

        assert_eq!(change.focus_id, "verification-otp-2");
        assert_eq!(change.action, InputOtpAction::Input);
    }

    #[test]
    fn list_box_items_and_selection_modes_are_controlled() {
        let item: ListBoxItem<'static, ()> = ListBoxItem::new("danger", text("Delete"))
            .description(text("Remove the local copy"))
            .variant(ListBoxItemVariant::Danger)
            .disabled(true)
            .selected(true);
        assert_eq!(item.item_id(), "danger");
        assert!(item.is_disabled());

        let list: ListBox<'static, ()> = ListBox::new(vec![item])
            .selection_mode(ListBoxSelectionMode::Multiple)
            .selected_many(vec![0]);
        assert_eq!(list.selection_mode, ListBoxSelectionMode::Multiple);
        assert_eq!(list.selected, vec![0]);
        let _: iced::Element<'static, ()> = list.into();
    }

    #[test]
    fn kbd_maps_modifier_keys_for_mac_and_windows() {
        assert_eq!(KbdKey::Command.display(KbdPlatform::Mac), "⌘");
        assert_eq!(KbdKey::Command.display(KbdPlatform::Win), "Ctrl");
        assert_eq!(KbdKey::Option.display(KbdPlatform::Mac), "⌥");
        assert_eq!(KbdKey::Option.display(KbdPlatform::Win), "Alt");
        assert_eq!(KbdKey::Win.display(KbdPlatform::Mac), "Win");
        assert_eq!(KbdKey::Win.display(KbdPlatform::Win), "Win");
        assert_eq!(KbdKey::Up.display(KbdPlatform::Mac), "↑");
    }

    #[test]
    fn kbd_defaults_and_variants_are_publicly_composable() {
        let mac = Kbd::mac([KbdKey::Command, KbdKey::character("K")]);
        assert_eq!(mac.selected_platform(), KbdPlatform::Mac);
        assert_eq!(mac.selected_variant(), KbdVariant::Default);
        assert_eq!(mac.keys().len(), 2);

        let win = Kbd::win([KbdKey::Command, KbdKey::character("K")])
            .variant(KbdVariant::Light)
            .push(KbdKey::Enter);
        assert_eq!(win.selected_platform(), KbdPlatform::Win);
        assert_eq!(win.selected_variant(), KbdVariant::Light);
        assert_eq!(win.keys().len(), 3);
        let _: iced::Element<'static, ()> = win.into();
    }

    #[test]
    fn kbd_variants_match_default_and_light_treatments() {
        let theme = super::app_theme();
        let default = kbd_style(KbdVariant::Default)(&theme);
        let light = kbd_style(KbdVariant::Light)(&theme);

        assert_eq!(default.background, Some(iced::Background::Color(SURFACE_ALT)));
        assert_eq!(default.border.width, 1.0);
        assert_eq!(light.background, None);
        assert_eq!(light.border.width, 0.0);
    }
}
