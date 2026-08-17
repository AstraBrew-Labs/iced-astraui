use iced::time::{self, Duration, Instant};
use iced::widget::text::LineHeight;
use iced::widget::{
    button, checkbox, column, container, mouse_area, pick_list, row, rule, scrollable, space, text,
    text_input,
};
use iced::{Alignment, Element, Fill, Pixels, Point, Subscription, Task, Theme};
use lucide_icons::Icon;

use crate::{fonts, icons, ui};

const PAGINATION_DEMO_TOTAL_PAGES: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Page {
    #[default]
    Components,
    Tokens,
    Patterns,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Channel {
    #[default]
    Stable,
    Preview,
    Nightly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DropdownAction {
    Duplicate,
    Rename,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModalKind {
    Form,
    Confirmation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToastDemo {
    Default,
    Success,
    Warning,
    Danger,
    Interactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum KeyboardScope {
    StandaloneToggle,
    TagGroup,
    AlignmentGroup,
    FormattingGroup,
    Toolbar,
}

#[derive(Debug, Clone)]
struct GlobalMessage {
    title: &'static str,
    description: String,
    kind: ui::MessageKind,
    expires_at: Instant,
}

#[derive(Debug, Clone)]
struct ToastNotice {
    id: u64,
    title: &'static str,
    description: &'static str,
    variant: ui::ToastVariant,
    action_label: Option<&'static str>,
    expires_at: Instant,
}

impl Channel {
    const ALL: [Self; 3] = [Self::Stable, Self::Preview, Self::Nightly];
}

impl std::fmt::Display for Channel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Stable => "Stable / production",
            Self::Preview => "Preview / release candidate",
            Self::Nightly => "Nightly / latest commit",
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Navigate(Page),
    InputChanged(String),
    ToggleChanged(bool),
    CheckChanged(bool),
    SliderChanged(f32),
    ChannelSelected(Channel),
    RadioChanged(usize),
    DisclosureToggled(bool),
    AccordionToggled(usize),
    DismissDemoAlert,
    RestoreDemoAlert,
    DropdownToggled(bool),
    ContextMenuOpened(Point),
    DismissMenus,
    DropdownSelected(DropdownAction),
    TabSelected(usize),
    Action(&'static str),
    DismissGlobalNotice,
    ToastPlacementSelected(ui::ToastPlacement),
    ShowToast(ToastDemo),
    DismissToast(u64),
    ToastAction(u64),
    TypographyCopied,
    PaginationChanged(usize),
    ControlFocused(KeyboardScope, usize),
    ControlActivated(KeyboardScope, usize),
    RemoveTag(usize),
    FocusNext,
    FocusPrevious,
    OpenModal(ModalKind),
    CloseModal,
    ConfirmModal,
    ModalInputChanged(String),
    Noop,
    Tick(Instant),
}

fn button_text<'a>(label: &'a str, size: f32) -> Element<'a, Message> {
    text(label)
        .size(size)
        .font(fonts::MEDIUM)
        .line_height(LineHeight::Absolute(Pixels(20.0)))
        .into()
}

#[derive(Debug, Default)]
pub struct Launcher {
    page: Page,
    input: String,
    toggled: bool,
    checked: bool,
    slider: f32,
    channel: Channel,
    radio_choice: usize,
    disclosure_open: bool,
    accordion_expanded: [bool; 3],
    show_demo_alert: bool,
    dropdown_open: bool,
    context_menu_position: Option<Point>,
    active_tab: usize,
    global_message: Option<GlobalMessage>,
    toasts: Vec<ToastNotice>,
    toast_placement: ui::ToastPlacement,
    next_toast_id: u64,
    pagination_page: usize,
    indeterminate_progress: f32,
    indeterminate_circle_progress: f32,
    progress_last_tick: Option<Instant>,
    keyboard_scope: Option<KeyboardScope>,
    standalone_toggle: bool,
    tag_labels: Vec<&'static str>,
    tag_selected: Vec<bool>,
    tag_focus: usize,
    alignment: usize,
    alignment_focus: usize,
    formatting: [bool; 3],
    formatting_focus: usize,
    toolbar_focus: usize,
    modal: Option<ModalKind>,
    modal_input: String,
    motion: ui::MotionState,
}

impl Launcher {
    pub fn new() -> (Self, Task<Message>) {
        let now = Instant::now();
        let mut motion = ui::MotionState::default();
        motion.start_progress(now);
        (
            Self {
                slider: 68.0,
                disclosure_open: true,
                accordion_expanded: [true, false, false],
                show_demo_alert: true,
                pagination_page: 6,
                tag_labels: vec!["Design", "Rust", "Desktop", "Accessible"],
                tag_selected: vec![true, true, false, false],
                alignment: 1,
                motion,
                ..Self::default()
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        "Astra UI / Local Iced component library".to_owned()
    }

    pub fn theme(&self) -> Theme {
        ui::app_theme()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let now = Instant::now();
        let timer = if self.motion.needs_ticks(now) || self.page == Page::Components {
            time::every(Duration::from_millis(16)).map(Message::Tick)
        } else if self.global_message.is_some() || !self.toasts.is_empty() {
            time::every(Duration::from_millis(100)).map(Message::Tick)
        } else {
            Subscription::none()
        };
        let keyboard = iced::event::listen_with(|event, status, _window| {
            if status == iced::event::Status::Ignored
                && let iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab),
                    modifiers,
                    ..
                }) = event
            {
                Some(if modifiers.shift() {
                    Message::FocusPrevious
                } else {
                    Message::FocusNext
                })
            } else {
                None
            }
        });

        Subscription::batch([timer, keyboard])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let now = Instant::now();
        match message {
            Message::Navigate(page) => {
                self.page = page;
                if page != Page::Components {
                    self.progress_last_tick = None;
                }
                self.motion.press(
                    match page {
                        Page::Components => "nav-components",
                        Page::Tokens => "nav-tokens",
                        Page::Patterns => "nav-patterns",
                    },
                    now,
                );
            }
            Message::InputChanged(value) => self.input = value,
            Message::ToggleChanged(value) => {
                self.toggled = value;
                self.motion.set_toggled(value, now);
            }
            Message::CheckChanged(value) => {
                self.checked = value;
                self.motion.set_checked(value, now);
            }
            Message::SliderChanged(value) => self.slider = value.clamp(0.0, 100.0),
            Message::ChannelSelected(channel) => self.channel = channel,
            Message::RadioChanged(choice) => self.radio_choice = choice,
            Message::DisclosureToggled(expanded) => self.disclosure_open = expanded,
            Message::AccordionToggled(index) => ui::toggle_accordion_item(
                &mut self.accordion_expanded,
                index,
                ui::AccordionSelectionMode::Single,
            ),
            Message::DismissDemoAlert => self.show_demo_alert = false,
            Message::RestoreDemoAlert => self.show_demo_alert = true,
            Message::DropdownToggled(expanded) => {
                self.dropdown_open = expanded;
                self.context_menu_position = None;
            }
            Message::ContextMenuOpened(position) => {
                self.context_menu_position = Some(position);
                self.dropdown_open = false;
            }
            Message::DismissMenus => {
                self.dropdown_open = false;
                self.context_menu_position = None;
            }
            Message::DropdownSelected(action) => {
                let notice = match action {
                    DropdownAction::Duplicate => "Component duplicated",
                    DropdownAction::Rename => "Rename action selected",
                    DropdownAction::Delete => "Component deleted",
                };
                self.dropdown_open = false;
                self.context_menu_position = None;
                self.motion.press(notice, now);
                self.show_global_message(
                    "Menu action selected",
                    notice,
                    ui::MessageKind::Info,
                    now,
                );
            }
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                self.motion.press(
                    match tab {
                        0 => "tab-0",
                        1 => "tab-1",
                        _ => "tab-2",
                    },
                    now,
                );
            }
            Message::Action(notice) => {
                self.motion.press(notice, now);
                self.show_global_message("Action completed", notice, ui::MessageKind::Success, now);
            }
            Message::DismissGlobalNotice => self.global_message = None,
            Message::ToastPlacementSelected(placement) => self.toast_placement = placement,
            Message::ShowToast(demo) => self.show_toast(demo, now),
            Message::DismissToast(id) => self.toasts.retain(|toast| toast.id != id),
            Message::ToastAction(id) => {
                self.toasts.retain(|toast| toast.id != id);
                self.push_toast(
                    "Update scheduled",
                    "The app will restart when current work is complete.",
                    ui::ToastVariant::Success,
                    None,
                    now,
                );
            }
            Message::TypographyCopied => self.push_toast(
                "复制成功",
                "选中的文本已复制到剪贴板。",
                ui::ToastVariant::Success,
                None,
                now,
            ),
            Message::PaginationChanged(page) => {
                self.pagination_page = page.clamp(1, PAGINATION_DEMO_TOTAL_PAGES);
            }
            Message::ControlFocused(scope, index) => {
                self.keyboard_scope = Some(scope);
                self.set_control_focus(scope, index);
            }
            Message::ControlActivated(scope, index) => {
                self.keyboard_scope = Some(scope);
                self.set_control_focus(scope, index);
                match scope {
                    KeyboardScope::StandaloneToggle => {
                        self.standalone_toggle = !self.standalone_toggle;
                    }
                    KeyboardScope::TagGroup => {
                        if let Some(selected) = self.tag_selected.get_mut(index) {
                            *selected = !*selected;
                        }
                    }
                    KeyboardScope::AlignmentGroup => self.alignment = index.min(2),
                    KeyboardScope::FormattingGroup => {
                        if let Some(selected) = self.formatting.get_mut(index) {
                            *selected = !*selected;
                        }
                    }
                    KeyboardScope::Toolbar => {
                        if let Some(selected) = index
                            .checked_sub(2)
                            .and_then(|format_index| self.formatting.get_mut(format_index))
                        {
                            *selected = !*selected;
                        } else {
                            self.motion.press(
                                if index == 0 {
                                    "toolbar-undo"
                                } else {
                                    "toolbar-redo"
                                },
                                now,
                            );
                        }
                    }
                }
            }
            Message::RemoveTag(index) => {
                if index < self.tag_labels.len() {
                    self.tag_labels.remove(index);
                    self.tag_selected.remove(index);
                    self.tag_focus = self.tag_focus.min(self.tag_labels.len().saturating_sub(1));
                    self.keyboard_scope = Some(KeyboardScope::TagGroup);
                }
            }
            Message::FocusNext => {
                self.keyboard_scope = None;
                return iced::widget::operation::focus_next();
            }
            Message::FocusPrevious => {
                self.keyboard_scope = None;
                return iced::widget::operation::focus_previous();
            }
            Message::OpenModal(kind) => {
                self.modal = Some(kind);
                self.dropdown_open = false;
                self.context_menu_position = None;
                self.motion.press(
                    match kind {
                        ModalKind::Form => "open-form-modal",
                        ModalKind::Confirmation => "open-confirmation-modal",
                    },
                    now,
                );
            }
            Message::CloseModal => self.modal = None,
            Message::ConfirmModal => match self.modal {
                Some(ModalKind::Form) if self.modal_input.trim().is_empty() => {
                    self.show_global_message(
                        "Required field",
                        "Enter a workspace name before submitting.",
                        ui::MessageKind::Danger,
                        now,
                    );
                }
                Some(ModalKind::Form) => {
                    self.modal = None;
                    self.show_global_message(
                        "Form submitted",
                        "The workspace configuration was saved.",
                        ui::MessageKind::Success,
                        now,
                    );
                }
                Some(ModalKind::Confirmation) => {
                    self.modal = None;
                    self.show_global_message(
                        "Action confirmed",
                        "The destructive operation was confirmed.",
                        ui::MessageKind::Warning,
                        now,
                    );
                }
                None => {}
            },
            Message::ModalInputChanged(value) => self.modal_input = value,
            Message::Noop => {}
            Message::Tick(now) => {
                self.motion.tick(now);
                if self.page == Page::Components {
                    let elapsed = self
                        .progress_last_tick
                        .and_then(|last_tick| now.checked_duration_since(last_tick))
                        .unwrap_or_default();
                    self.indeterminate_progress =
                        (self.indeterminate_progress + elapsed.as_secs_f32() / 1.5).rem_euclid(1.0);
                    self.indeterminate_circle_progress = (self.indeterminate_circle_progress
                        + elapsed.as_secs_f32())
                    .rem_euclid(1.0);
                    self.progress_last_tick = Some(now);
                } else {
                    self.progress_last_tick = None;
                }
                if self
                    .global_message
                    .as_ref()
                    .is_some_and(|message| now >= message.expires_at)
                {
                    self.global_message = None;
                }
                self.toasts.retain(|toast| now < toast.expires_at);
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let base = row![self.sidebar(), self.content()]
            .width(Fill)
            .height(Fill);
        let mut layers = iced::widget::Stack::new()
            .width(Fill)
            .height(Fill)
            .push(base);

        let mut global_stack = iced::widget::Stack::new().width(Fill).height(Fill);
        let mut has_global_layer = false;

        if let Some(kind) = self.modal {
            global_stack = global_stack.push(self.modal_layer(kind));
            has_global_layer = true;
        }
        if !self.toasts.is_empty() {
            global_stack = global_stack.push(self.toast_layer());
            has_global_layer = true;
        }
        if let Some(message) = self.global_message.as_ref() {
            global_stack = global_stack.push(self.message_layer(message));
            has_global_layer = true;
        }

        if has_global_layer {
            layers = layers.push(ui::global_layer(global_stack, ui::GlobalLayer::Message));
        }

        layers.into()
    }

    fn message_layer<'a>(&self, message: &'a GlobalMessage) -> Element<'a, Message> {
        container(
            mouse_area(ui::global_message(
                message.title,
                &message.description,
                message.kind,
                Message::DismissGlobalNotice,
            ))
            .on_press(Message::Noop),
        )
        .width(Fill)
        .height(Fill)
        .align_x(Alignment::End)
        .align_y(Alignment::Start)
        .padding([24, 28])
        .into()
    }

    fn toast_layer(&self) -> Element<'_, Message> {
        let toasts = self
            .toasts
            .iter()
            .fold(column![].spacing(10).width(420), |toasts, toast| {
                toasts.push(ui::toast(
                    toast.title,
                    toast.description,
                    toast.variant,
                    toast
                        .action_label
                        .map(|label| (label, Message::ToastAction(toast.id))),
                    Message::DismissToast(toast.id),
                    Message::Noop,
                ))
            });

        ui::toast_region(toasts, self.toast_placement)
    }

    fn show_global_message(
        &mut self,
        title: &'static str,
        description: impl Into<String>,
        kind: ui::MessageKind,
        now: Instant,
    ) {
        self.dropdown_open = false;
        self.context_menu_position = None;
        self.global_message = Some(GlobalMessage {
            title,
            description: description.into(),
            kind,
            expires_at: now + Duration::from_millis(3_200),
        });
    }

    fn show_toast(&mut self, demo: ToastDemo, now: Instant) {
        self.dropdown_open = false;
        self.context_menu_position = None;
        let (title, description, variant, action_label) = match demo {
            ToastDemo::Default => (
                "Draft saved",
                "Your local changes are ready for the next build.",
                ui::ToastVariant::Default,
                None,
            ),
            ToastDemo::Success => (
                "Component published",
                "The local component index has been refreshed.",
                ui::ToastVariant::Success,
                None,
            ),
            ToastDemo::Warning => (
                "Review required",
                "Two component tokens still use fallback values.",
                ui::ToastVariant::Warning,
                None,
            ),
            ToastDemo::Danger => (
                "Build failed",
                "The generated component module could not be loaded.",
                ui::ToastVariant::Danger,
                None,
            ),
            ToastDemo::Interactive => (
                "Update available",
                "A new component runtime is ready to install.",
                ui::ToastVariant::Accent,
                Some("Restart"),
            ),
        };

        self.push_toast(title, description, variant, action_label, now);
    }

    fn push_toast(
        &mut self,
        title: &'static str,
        description: &'static str,
        variant: ui::ToastVariant,
        action_label: Option<&'static str>,
        now: Instant,
    ) {
        self.next_toast_id = self.next_toast_id.wrapping_add(1);
        self.toasts.push(ToastNotice {
            id: self.next_toast_id,
            title,
            description,
            variant,
            action_label,
            expires_at: now + Duration::from_secs(4),
        });
        if self.toasts.len() > 3 {
            self.toasts.remove(0);
        }
    }

    fn set_control_focus(&mut self, scope: KeyboardScope, index: usize) {
        match scope {
            KeyboardScope::StandaloneToggle => {}
            KeyboardScope::TagGroup => {
                self.tag_focus = index.min(self.tag_labels.len().saturating_sub(1));
            }
            KeyboardScope::AlignmentGroup => self.alignment_focus = index.min(2),
            KeyboardScope::FormattingGroup => self.formatting_focus = index.min(2),
            KeyboardScope::Toolbar => self.toolbar_focus = index.min(4),
        }
    }

    fn sidebar(&self) -> Element<'_, Message> {
        let brand = row![
            container(icons::icon(Icon::Component, 20, ui::WHITE))
                .width(36)
                .height(36)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(ui::BLUE_600)),
                    border: iced::Border {
                        radius: 12.0.into(),
                        ..iced::Border::default()
                    },
                    ..container::Style::default()
                }),
            column![
                text("ASTRA UI").size(16).font(fonts::BLACK),
                text("LOCAL ICED KIT")
                    .size(10)
                    .font(fonts::LIGHT)
                    .color(ui::INK_MUTED),
            ]
            .spacing(1),
        ]
        .align_y(Alignment::Center)
        .spacing(11);

        let navigation = column![
            self.nav_button(
                Page::Components,
                Icon::Blocks,
                "Components",
                "12 primitives"
            ),
            self.nav_button(Page::Tokens, Icon::Palette, "Tokens", "Color + type"),
            self.nav_button(
                Page::Patterns,
                Icon::LayoutTemplate,
                "Patterns",
                "Composed UI"
            ),
        ]
        .spacing(7);

        let footer = column![
            text("DESIGN SYSTEM")
                .size(10)
                .font(fonts::LIGHT)
                .color(ui::INK_MUTED),
            row![
                icons::icon(Icon::Circle, 8, ui::CYAN_500),
                text("v0.1 / Blue + cyan").size(12).color(ui::INK_MUTED)
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text("Iced 0.14 / Rust 2024")
                .size(11)
                .font(fonts::LIGHT)
                .color(ui::INK_MUTED),
        ]
        .spacing(10);

        container(
            column![
                brand,
                space::vertical(),
                navigation,
                space::vertical(),
                footer
            ]
            .padding([28, 18])
            .height(Fill),
        )
        .width(246)
        .height(Fill)
        .style(ui::sidebar)
        .into()
    }

    fn nav_button(
        &self,
        page: Page,
        glyph: Icon,
        label: &'static str,
        hint: &'static str,
    ) -> Element<'_, Message> {
        let active = self.page == page;
        button(
            row![
                icons::icon(glyph, 17, if active { ui::BLUE_600 } else { ui::INK_MUTED }),
                column![
                    text(label).size(13).font(fonts::MEDIUM),
                    text(hint).size(10).color(ui::INK_MUTED)
                ]
                .spacing(2),
                space::horizontal(),
                icons::icon(
                    Icon::ChevronRight,
                    15,
                    if active { ui::BLUE_600 } else { ui::INK_SUBTLE }
                ),
            ]
            .spacing(11)
            .align_y(Alignment::Center),
        )
        .on_press(Message::Navigate(page))
        .padding([10, 11])
        .width(Fill)
        .style(ui::nav_button_animated(
            active,
            self.motion.press_progress(
                match page {
                    Page::Components => "nav-components",
                    Page::Tokens => "nav-tokens",
                    Page::Patterns => "nav-patterns",
                },
                Instant::now(),
            ),
        ))
        .into()
    }

    fn content(&self) -> Element<'_, Message> {
        let body = match self.page {
            Page::Components => self.components_page(),
            Page::Tokens => self.tokens_page(),
            Page::Patterns => self.patterns_page(),
        };
        container(scrollable(body).width(Fill).height(Fill))
            .width(Fill)
            .height(Fill)
            .style(ui::canvas)
            .into()
    }

    fn modal_layer(&self, kind: ModalKind) -> Element<'_, Message> {
        match kind {
            ModalKind::Form => ui::global_modal(
                "Create workspace",
                "Configure a reusable local workspace.",
                column![
                    text("Workspace name")
                        .size(11)
                        .font(fonts::MEDIUM)
                        .color(ui::INK_MUTED),
                    container(
                        text_input("Design system", &self.modal_input)
                            .on_input(Message::ModalInputChanged)
                            .padding([8, 12])
                            .size(14)
                            .line_height(LineHeight::Absolute(Pixels(20.0)))
                            .style(ui::text_input_style)
                    )
                    .style(ui::field_frame),
                    text("This name is used in local component metadata.")
                        .size(11)
                        .font(fonts::REGULAR)
                        .color(ui::INK_MUTED),
                ]
                .spacing(8),
                "Cancel",
                "Create",
                false,
                Message::CloseModal,
                Message::ConfirmModal,
                Message::Noop,
            ),
            ModalKind::Confirmation => ui::AlertDialog::new(
                "Delete local preset?",
                "The preset will be removed from this device. This operation cannot be undone.",
                Message::CloseModal,
                Message::ConfirmModal,
                Message::Noop,
            )
            .status(ui::AlertKind::Danger)
            .cancel_label("Cancel")
            .confirm_label("Delete")
            .destructive(true)
            .into(),
        }
    }

    fn components_page(&self) -> Element<'_, Message> {
        let now = Instant::now();
        let press = |id| self.motion.press_progress(id, now);
        let header = row![
            column![
                text("Astra UI").size(32).font(fonts::BLACK),
                text("A local component language for Iced apps.")
                    .size(14)
                    .color(ui::INK_MUTED)
            ]
            .spacing(5),
            space::horizontal(),
            container(
                row![
                    icons::icon(Icon::Sparkles, 15, ui::BLUE_600),
                    text("HeroUI V3 / Figma source")
                        .size(12)
                        .font(fonts::MEDIUM)
                ]
                .spacing(8)
                .align_y(Alignment::Center)
            )
            .padding([2, 8])
            .style(ui::tag_style(ui::BLUE_600)),
        ]
        .align_y(Alignment::Center);

        let buttons = self.component_card(
            "Buttons",
            "Semantic variants with shared hover, pressed, disabled, and focus treatment.",
            column![
                row![
                    button(button_text("Primary", 14.0))
                        .on_press(Message::Action("Primary action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Primary,
                            press("Primary action fired"),
                        )),
                    button(button_text("Secondary", 14.0))
                        .on_press(Message::Action("Secondary action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Secondary,
                            press("Secondary action fired"),
                        )),
                    button(button_text("Tertiary", 14.0))
                        .on_press(Message::Action("Tertiary action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Tertiary,
                            press("Tertiary action fired"),
                        )),
                    button(button_text("Ghost", 14.0))
                        .on_press(Message::Action("Ghost action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Ghost,
                            press("Ghost action fired")
                        )),
                    button(button_text("Danger", 14.0))
                        .on_press(Message::Action("Destructive action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Destructive,
                            press("Destructive action fired"),
                        )),
                    button(button_text("Danger soft", 14.0))
                        .on_press(Message::Action("Danger soft action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::DangerSoft,
                            press("Danger soft action fired"),
                        )),
                    button(button_text("Outline", 14.0))
                        .on_press(Message::Action("Outline action fired"))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Outline,
                            press("Outline action fired"),
                        )),
                ]
                .spacing(8)
                .wrap(),
                row![
                    button(
                        container(icons::icon(Icon::Plus, 16, ui::WHITE))
                            .width(Fill)
                            .height(Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                    )
                    .on_press(Message::Action("Icon action fired"))
                    .width(ui::CONTROL_HEIGHT_MD)
                    .height(ui::CONTROL_HEIGHT_MD)
                    .padding(0)
                    .style(ui::button_style_animated(
                        ui::ButtonVariant::Primary,
                        press("Icon action fired"),
                    )),
                    button(
                        container(icons::icon(Icon::Download, 16, ui::BLUE_700))
                            .width(Fill)
                            .height(Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                    )
                    .on_press(Message::Action("Download action fired"))
                    .width(ui::CONTROL_HEIGHT_MD)
                    .height(ui::CONTROL_HEIGHT_MD)
                    .padding(0)
                    .style(ui::button_style_animated(
                        ui::ButtonVariant::Secondary,
                        press("Download action fired"),
                    )),
                    button(
                        container(icons::icon(Icon::X, 16, ui::INK_MUTED))
                            .width(Fill)
                            .height(Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                    )
                    .on_press(Message::Action("Close action fired"))
                    .width(ui::CONTROL_HEIGHT_MD)
                    .height(ui::CONTROL_HEIGHT_MD)
                    .padding(0)
                    .style(ui::button_style_animated(
                        ui::ButtonVariant::Ghost,
                        press("Close action fired")
                    )),
                    text("Icon-only actions stay compact and discoverable.")
                        .size(11)
                        .color(ui::INK_MUTED),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            ]
            .spacing(18),
        );

        let fields = self.component_card(
            "Fields & controls",
            "Inputs use the same radius, focus ring, and semantic accent as every other control.",
            column![
                row![
                    column![
                        text("Email address")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        container(
                            text_input("you@example.com", &self.input)
                                .on_input(Message::InputChanged)
                                .padding([8, 12])
                                .size(14)
                                .line_height(LineHeight::Absolute(Pixels(20.0)))
                                .style(ui::text_input_style)
                        )
                        .style(ui::field_frame)
                    ]
                    .spacing(7)
                    .width(Fill),
                    column![
                        text("Release channel")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        container(
                            pick_list(Channel::ALL, Some(self.channel), Message::ChannelSelected)
                                .padding([8, 12])
                                .text_size(14)
                                .text_line_height(LineHeight::Absolute(Pixels(20.0)))
                                .font(fonts::REGULAR)
                                .handle(ui::pick_list_handle())
                                .width(Fill)
                                .style(ui::pick_list_style)
                                .menu_style(ui::pick_list_menu_style)
                        )
                        .style(ui::field_frame)
                    ]
                    .spacing(7)
                    .width(Fill),
                ]
                .spacing(16),
                row![
                    ui::switch(
                        "Enable notifications",
                        self.toggled,
                        self.motion.toggled_progress(now),
                        Message::ToggleChanged,
                    ),
                    checkbox(self.checked)
                        .label("I agree to the design contract")
                        .on_toggle(Message::CheckChanged)
                        .size(16)
                        .spacing(9)
                        .text_size(12)
                        .style(ui::checkbox_style_animated(
                            self.motion.checked_progress(now),
                        )),
                ]
                .spacing(28)
                .align_y(Alignment::Center),
                column![
                    row![
                        text("Density")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        space::horizontal(),
                        text(format!("{:.0}%", self.slider))
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::BLUE_700)
                    ]
                    .width(Fill),
                    ui::slider(0.0..=100.0, self.slider, Message::SliderChanged)
                ]
                .spacing(7),
                row![
                    text("Color mode")
                        .size(11)
                        .font(fonts::MEDIUM)
                        .color(ui::INK_MUTED),
                    ui::radio("Blue", 0, Some(self.radio_choice), Message::RadioChanged),
                    ui::radio("Cyan", 1, Some(self.radio_choice), Message::RadioChanged)
                ]
                .spacing(15)
                .align_y(Alignment::Center),
            ]
            .spacing(16),
        );

        let data_display = self.component_card(
            "Chip, disclosure & dropdown",
            "Compact metadata, collapsible details, and contextual actions share one visual language.",
            column![
                column![
                    text("Chip")
                        .size(11)
                        .font(fonts::MEDIUM)
                        .color(ui::INK_MUTED),
                    row![
                        ui::chip(
                            "Frontend",
                            Some(Icon::Tag),
                            ui::BLUE_600,
                            ui::ChipVariant::Flat,
                        ),
                        ui::chip(
                            "Stable",
                            Some(Icon::CircleCheck),
                            ui::SUCCESS,
                            ui::ChipVariant::Solid,
                        ),
                        ui::chip(
                            "API",
                            Some(Icon::Hash),
                            ui::CYAN_500,
                            ui::ChipVariant::Outline,
                        ),
                    ]
                    .spacing(8)
                    .wrap(),
                ]
                .spacing(8),
                ui::disclosure(
                    "Component conventions",
                    Some("Shared behavior and visual rules"),
                    self.disclosure_open,
                    Message::DisclosureToggled(!self.disclosure_open),
                    column![
                        row![
                            icons::icon(Icon::CircleCheck, 15, ui::SUCCESS),
                            text("Uses local HarmonyOS Sans fonts")
                                .size(11)
                                .font(fonts::REGULAR)
                                .color(ui::INK),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                        row![
                            icons::icon(Icon::CircleCheck, 15, ui::SUCCESS),
                            text("Uses the shared blue and cyan token palette")
                                .size(11)
                                .font(fonts::REGULAR)
                                .color(ui::INK),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(9),
                ),
                row![
                    text("Dropdown")
                        .size(11)
                        .font(fonts::MEDIUM)
                        .color(ui::INK_MUTED),
                    space::horizontal(),
                    ui::dropdown(
                        "Component actions",
                        Some(Icon::Ellipsis),
                        self.dropdown_open,
                        Message::DropdownToggled(!self.dropdown_open),
                        Message::DismissMenus,
                        Self::component_menu_items(),
                    ),
                ]
                .align_y(Alignment::Center),
                column![
                    text("Context menu")
                        .size(11)
                        .font(fonts::MEDIUM)
                        .color(ui::INK_MUTED),
                    ui::context_menu(
                        container(
                            row![
                                container(icons::icon(Icon::FileCode, 17, ui::BLUE_600))
                                    .width(34)
                                    .height(34)
                                    .align_x(Alignment::Center)
                                    .align_y(Alignment::Center)
                                    .style(ui::tag_style(ui::BLUE_600)),
                                column![
                                    text("astra-button.rs").size(12).font(fonts::MEDIUM),
                                    text("Local component source")
                                        .size(10)
                                        .font(fonts::REGULAR)
                                        .color(ui::INK_MUTED),
                                ]
                                .spacing(2),
                            ]
                            .spacing(10)
                            .align_y(Alignment::Center),
                        )
                        .width(Fill)
                        .height(60)
                        .padding([8, 12])
                        .style(ui::tint),
                        self.context_menu_position,
                        Message::ContextMenuOpened,
                        Message::DismissMenus,
                        Self::component_menu_items(),
                    ),
                ]
                .spacing(8),
            ]
            .spacing(16),
        );

        let feedback = self.component_card(
            "Feedback",
            "Status, progress, and inline messaging keep system state visible without noise.",
            column![
                row![
                    ui::badge(
                        button(
                            container(icons::icon(Icon::Bell, 17, ui::INK))
                                .width(Fill)
                                .height(Fill)
                                .align_x(Alignment::Center)
                                .align_y(Alignment::Center)
                        )
                        .on_press(Message::Action("Notifications opened"))
                        .width(ui::CONTROL_HEIGHT_MD)
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding(0)
                        .style(ui::button_style(ui::ButtonVariant::Secondary)),
                        ui::BadgeContent::Count(7),
                        ui::DANGER,
                        ui::BadgePosition::TopRight,
                    ),
                    ui::badge(
                        container(text("AB").size(11).font(fonts::BOLD).color(ui::WHITE))
                            .width(ui::CONTROL_HEIGHT_MD)
                            .height(ui::CONTROL_HEIGHT_MD)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .style(|_| container::Style {
                                background: Some(iced::Background::Color(ui::BLUE_700)),
                                border: iced::Border {
                                    radius: 18.0.into(),
                                    ..iced::Border::default()
                                },
                                ..container::Style::default()
                            }),
                        ui::BadgeContent::Dot,
                        ui::SUCCESS,
                        ui::BadgePosition::BottomRight,
                    ),
                    ui::badge(
                        button(button_text("Package", 12.0))
                            .on_press(Message::Action("Package opened"))
                            .height(ui::CONTROL_HEIGHT_MD)
                            .padding([8, 15])
                            .style(ui::button_style(ui::ButtonVariant::Outline)),
                        ui::BadgeContent::Label("NEW"),
                        ui::BLUE_600,
                        ui::BadgePosition::TopRight,
                    ),
                ]
                .spacing(28)
                .align_y(Alignment::Center),
                row![
                    self.badge("READY", ui::SUCCESS),
                    self.badge("PREVIEW", ui::BLUE_600),
                    self.badge("BETA", ui::WARNING),
                    self.badge("DEPRECATED", ui::DANGER),
                    ui::chip(
                        "component-kit",
                        Some(Icon::Hash),
                        ui::BLUE_700,
                        ui::ChipVariant::Flat,
                    ),
                    ui::chip(
                        "iced-0.14",
                        Some(Icon::Hash),
                        ui::BLUE_700,
                        ui::ChipVariant::Flat,
                    )
                ]
                .spacing(8)
                .wrap(),
                row![
                    container(
                        row![
                            icons::icon(Icon::Info, 16, ui::BLUE_600),
                            text("Info: styles are local and themeable.").size(11)
                        ]
                        .spacing(9)
                        .align_y(Alignment::Center)
                    )
                    .height(64)
                    .padding([0, 16])
                    .width(Fill)
                    .style(ui::alert(ui::AlertKind::Info)),
                    container(
                        row![
                            icons::icon(Icon::CircleCheck, 16, ui::SUCCESS),
                            text("Saved").size(11)
                        ]
                        .spacing(9)
                        .align_y(Alignment::Center)
                    )
                    .height(64)
                    .padding([0, 16])
                    .style(ui::alert(ui::AlertKind::Success)),
                ]
                .spacing(10),
                row![
                    button(button_text("Open form", 13.0))
                        .on_press(Message::OpenModal(ModalKind::Form))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Primary,
                            press("open-form-modal"),
                        )),
                    button(button_text("Confirm action", 13.0))
                        .on_press(Message::OpenModal(ModalKind::Confirmation))
                        .height(ui::CONTROL_HEIGHT_MD)
                        .padding([8, 16])
                        .style(ui::button_style_animated(
                            ui::ButtonVariant::Outline,
                            press("open-confirmation-modal"),
                        )),
                ]
                .spacing(8),
            ]
            .spacing(16),
        );

        let avatar_showcase = self.avatar_showcase();
        let alert_showcase = self.alert_showcase();
        let accordion_showcase = self.accordion_showcase();
        let alert_dialog_showcase = self.alert_dialog_showcase();
        let progress_bars = self.progress_bar_showcase();
        let progress_circles = self.progress_circle_showcase();
        let separator_showcase = self.separator_showcase();
        let typography_showcase = self.typography_showcase();
        let scroll_shadow_showcase = self.scroll_shadow_showcase();
        let card_and_toast = self.card_and_toast_showcase();
        let selection_and_navigation = self.selection_and_navigation_showcase();

        column![
            header,
            buttons,
            fields,
            avatar_showcase,
            alert_showcase,
            accordion_showcase,
            alert_dialog_showcase,
            progress_bars,
            progress_circles,
            separator_showcase,
            typography_showcase,
            scroll_shadow_showcase,
            data_display,
            card_and_toast,
            selection_and_navigation,
            feedback
        ]
        .spacing(22)
        .padding([34, 42])
        .width(Fill)
        .into()
    }

    fn avatar_showcase(&self) -> Element<'_, Message> {
        fn avatar_example<'a>(
            label: &'static str,
            avatar: Element<'a, Message>,
        ) -> Element<'a, Message> {
            column![
                avatar,
                text(label)
                    .size(10)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            ]
            .spacing(8)
            .align_x(Alignment::Center)
            .into()
        }

        self.component_card(
            "Avatar",
            "Profile images with circular, rounded, and square shapes plus resilient fallback content.",
            row![
                avatar_example(
                    "Circle",
                    ui::Avatar::new("Astra")
                        .size(ui::AvatarSize::Large)
                        .shape(ui::AvatarShape::Circle)
                        .into(),
                ),
                avatar_example(
                    "Rounded",
                    ui::Avatar::new("Brew")
                        .size(ui::AvatarSize::Large)
                        .shape(ui::AvatarShape::Rounded)
                        .color(ui::AvatarColor::Success)
                        .into(),
                ),
                avatar_example(
                    "Square",
                    ui::Avatar::new("Components")
                        .size(ui::AvatarSize::Large)
                        .shape(ui::AvatarShape::Square)
                        .color(ui::AvatarColor::Warning)
                        .into(),
                ),
                avatar_example(
                    "Image",
                    ui::Avatar::new("Studio")
                        .image("assets/icon/icon.png")
                        .size(ui::AvatarSize::Large)
                        .shape(ui::AvatarShape::Rounded)
                        .into(),
                ),
                avatar_example(
                    "Custom fallback",
                    ui::Avatar::new("Custom")
                        .fallback(icons::icon(Icon::UserRound, 20, ui::WHITE))
                        .size(ui::AvatarSize::Large)
                        .shape(ui::AvatarShape::Circle)
                        .color(ui::AvatarColor::Danger)
                        .into(),
                ),
            ]
            .spacing(24)
            .align_y(Alignment::Start),
        )
    }

    fn alert_showcase(&self) -> Element<'_, Message> {
        let dismissible: Element<'_, Message> = if self.show_demo_alert {
            ui::Alert::new("Workspace published")
                .description("The local component index is ready for use.")
                .kind(ui::AlertKind::Success)
                .on_close(Message::DismissDemoAlert)
                .into()
        } else {
            button(button_text("Restore success alert", 12.0))
                .on_press(Message::RestoreDemoAlert)
                .height(ui::CONTROL_HEIGHT_MD)
                .padding([8, 14])
                .style(ui::button_style(ui::ButtonVariant::Secondary))
                .into()
        };
        let retry: Element<'_, Message> = button(button_text("Retry", 12.0))
            .on_press(Message::Action("Connection retry requested"))
            .height(32)
            .padding([6, 12])
            .style(ui::button_style(ui::ButtonVariant::DangerSoft))
            .into();

        self.component_card(
            "Alert",
            "Important inline messages with semantic status indicators, actions, and dismissal.",
            column![
                ui::Alert::new("Component update available")
                    .description("Refresh the local index to load the newest primitives.")
                    .kind(ui::AlertKind::Info),
                dismissible,
                ui::Alert::new("Scheduled maintenance")
                    .description("Publishing will pause briefly at 02:00 UTC.")
                    .kind(ui::AlertKind::Warning),
                ui::Alert::new("Unable to reach registry")
                    .description("The last known local component index remains available.")
                    .kind(ui::AlertKind::Danger)
                    .action(retry),
            ]
            .spacing(10),
        )
    }

    fn accordion_showcase(&self) -> Element<'_, Message> {
        let items = vec![
            ui::AccordionItem::new(
                "Component structure",
                self.accordion_expanded[0],
                Message::AccordionToggled(0),
                text("Compose focused primitives and keep application state outside the view.")
                    .size(12)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            )
            .description("Ownership and composition rules"),
            ui::AccordionItem::new(
                "Theme tokens",
                self.accordion_expanded[1],
                Message::AccordionToggled(1),
                text("Blue and cyan accents share semantic success, warning, and danger states.")
                    .size(12)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            )
            .description("Color, radius, and typography"),
            ui::AccordionItem::new(
                "Keyboard behavior",
                self.accordion_expanded[2],
                Message::AccordionToggled(2),
                text("Triggers remain keyboard focusable and publish the same toggle messages.")
                    .size(12)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            )
            .description("Predictable interaction states"),
        ];

        self.component_card(
            "Accordion",
            "Connected disclosure items organize related information in a compact surface.",
            ui::Accordion::new(items).variant(ui::AccordionVariant::Surface),
        )
    }

    fn alert_dialog_showcase(&self) -> Element<'_, Message> {
        self.component_card(
            "AlertDialog",
            "A required-action global dialog for critical confirmation workflows.",
            row![
                container(icons::icon(Icon::ShieldAlert, 18, ui::DANGER))
                    .width(36)
                    .height(36)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(ui::tag_style(ui::DANGER)),
                column![
                    text("Delete local preset").size(13).font(fonts::MEDIUM),
                    text("Requires an explicit Cancel or Delete decision.")
                        .size(11)
                        .font(fonts::REGULAR)
                        .color(ui::INK_MUTED),
                ]
                .spacing(3),
                space::horizontal(),
                button(button_text("Open alert dialog", 13.0))
                    .on_press(Message::OpenModal(ModalKind::Confirmation))
                    .height(ui::CONTROL_HEIGHT_MD)
                    .padding([8, 16])
                    .style(ui::button_style(ui::ButtonVariant::Destructive)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
    }

    fn progress_bar_showcase(&self) -> Element<'_, Message> {
        let now = Instant::now();
        let colors = column![
            ui::ProgressBar::new(45.0)
                .label("Default")
                .show_value(false)
                .color(ui::ProgressBarColor::Default),
            ui::ProgressBar::new(55.0)
                .label("Accent")
                .show_value(false)
                .color(ui::ProgressBarColor::Accent),
            ui::ProgressBar::new(65.0)
                .label("Success")
                .show_value(false)
                .color(ui::ProgressBarColor::Success),
            ui::ProgressBar::new(75.0)
                .label("Warning")
                .show_value(false)
                .color(ui::ProgressBarColor::Warning),
            ui::ProgressBar::new(85.0)
                .label("Danger")
                .show_value(false)
                .color(ui::ProgressBarColor::Danger),
        ]
        .spacing(10);

        let sizes = column![
            ui::ProgressBar::new(40.0)
                .label("Small")
                .show_value(false)
                .size(ui::ProgressBarSize::Small),
            ui::ProgressBar::new(60.0)
                .label("Medium")
                .show_value(false)
                .size(ui::ProgressBarSize::Medium),
            ui::ProgressBar::new(80.0)
                .label("Large")
                .show_value(false)
                .size(ui::ProgressBarSize::Large),
        ]
        .spacing(10);

        self.component_card(
            "ProgressBar",
            "Determinate and indeterminate progress with semantic colors, sizes, labels, and custom ranges.",
            column![
                row![
                    ui::ProgressBar::new(72.0 * self.motion.progress_progress(now))
                        .label("Install progress")
                        .color(ui::ProgressBarColor::Accent)
                        .width(Fill),
                    ui::ProgressBar::new(0.0)
                        .label("Syncing components")
                        .is_indeterminate(true)
                        .animation_phase(self.indeterminate_progress)
                        .color(ui::ProgressBarColor::Accent)
                        .width(Fill),
                ]
                .spacing(20)
                .align_y(Alignment::End),
                row![
                    column![
                        text("Colors")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        colors,
                    ]
                    .spacing(10)
                    .width(Fill),
                    column![
                        text("Sizes")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        sizes,
                    ]
                    .spacing(10)
                    .width(Fill),
                ]
                .spacing(20)
                .align_y(Alignment::Start),
                row![
                    ui::ProgressBar::new(750.0)
                        .range(0.0..=1000.0)
                        .label("Custom range")
                        .value_label("750 / 1000")
                        .color(ui::ProgressBarColor::Success)
                        .width(Fill),
                    column![
                        text("Without visible label")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        ui::ProgressBar::new(45.0)
                            .color(ui::ProgressBarColor::Default)
                            .size(ui::ProgressBarSize::Small)
                            .width(Fill),
                    ]
                    .spacing(8)
                    .width(Fill),
                ]
                .spacing(20)
                .align_y(Alignment::End),
            ]
            .spacing(18),
        )
    }

    fn progress_circle_showcase(&self) -> Element<'_, Message> {
        let sizes: Vec<Element<'_, Message>> = [
            ("Small", ui::ProgressCircleSize::Small, 40.0),
            ("Medium", ui::ProgressCircleSize::Medium, 60.0),
            ("Large", ui::ProgressCircleSize::Large, 80.0),
        ]
        .into_iter()
        .map(|(label, size, value)| {
            column![
                ui::ProgressCircle::new(value).size(size),
                text(label)
                    .size(10)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            ]
            .spacing(7)
            .align_x(Alignment::Center)
            .into()
        })
        .collect();
        let colors: Vec<Element<'_, Message>> = [
            ("Default", ui::ProgressCircleColor::Default),
            ("Accent", ui::ProgressCircleColor::Accent),
            ("Success", ui::ProgressCircleColor::Success),
            ("Warning", ui::ProgressCircleColor::Warning),
            ("Danger", ui::ProgressCircleColor::Danger),
        ]
        .into_iter()
        .map(|(label, color)| {
            column![
                ui::ProgressCircle::new(60.0).color(color),
                text(label)
                    .size(10)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            ]
            .spacing(7)
            .align_x(Alignment::Center)
            .into()
        })
        .collect();

        self.component_card(
            "ProgressCircle",
            "Circular determinate and indeterminate progress with semantic colors, sizes, and labels.",
            column![
                row![
                    column![
                        text("Sizes")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        row(sizes).spacing(20).align_y(Alignment::End),
                    ]
                    .spacing(12)
                    .width(Fill),
                    column![
                        text("Colors")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        row(colors).spacing(20).align_y(Alignment::End),
                    ]
                    .spacing(12)
                    .width(Fill),
                ]
                .spacing(24)
                .align_y(Alignment::Start),
                row![
                    ui::ProgressCircle::new(0.0)
                        .is_indeterminate(true)
                        .animation_phase(self.indeterminate_circle_progress)
                        .size(ui::ProgressCircleSize::Large)
                        .label("Loading"),
                    ui::ProgressCircle::new(75.0)
                        .size(ui::ProgressCircleSize::Large)
                        .color(ui::ProgressCircleColor::Success)
                        .label("75% Complete"),
                    ui::ProgressCircle::new(750.0)
                        .range(0.0..=1000.0)
                        .size(ui::ProgressCircleSize::Large)
                        .color(ui::ProgressCircleColor::Warning)
                        .label("750 / 1000"),
                ]
                .spacing(32)
                .align_y(Alignment::Center),
            ]
            .spacing(20),
        )
    }

    fn separator_showcase(&self) -> Element<'_, Message> {
        let horizontal_variant = |label, variant| {
            column![
                text(label)
                    .size(11)
                    .font(fonts::MEDIUM)
                    .color(ui::INK_MUTED),
                ui::Separator::new().variant(variant),
            ]
            .spacing(7)
            .width(Fill)
        };

        self.component_card(
            "Separator",
            "Visual dividers for horizontal content sections and compact vertical groups.",
            column![
                horizontal_variant("Default", ui::SeparatorVariant::Default),
                horizontal_variant("Secondary", ui::SeparatorVariant::Secondary),
                horizontal_variant("Tertiary", ui::SeparatorVariant::Tertiary),
                row![
                    text("Overview").size(12).font(fonts::REGULAR),
                    ui::Separator::new().orientation(ui::SeparatorOrientation::Vertical),
                    text("Components").size(12).font(fonts::REGULAR),
                    ui::Separator::new().orientation(ui::SeparatorOrientation::Vertical),
                    text("Source").size(12).font(fonts::REGULAR),
                ]
                .height(22)
                .spacing(14)
                .align_y(Alignment::Center),
            ]
            .spacing(16),
        )
    }

    fn typography_showcase(&self) -> Element<'_, Message> {
        let heading_scale = column![
            ui::Typography::heading(1, "Build better interfaces")
                .on_copy(Message::TypographyCopied),
            ui::Typography::heading(2, "Typography stays semantic")
                .on_copy(Message::TypographyCopied),
            ui::Typography::heading(3, "Composable by default").on_copy(Message::TypographyCopied),
            ui::Typography::heading(4, "Application section").on_copy(Message::TypographyCopied),
            ui::Typography::heading(5, "Card title").on_copy(Message::TypographyCopied),
            ui::Typography::heading(6, "Compact heading").on_copy(Message::TypographyCopied),
        ]
        .spacing(8)
        .width(Fill);
        let body_scale = column![
            ui::Typography::paragraph(
                "Primary body text uses the bundled HarmonyOS Sans family and the HeroUI line-height scale.",
            )
            .width(Fill)
            .on_copy(Message::TypographyCopied),
            ui::Typography::new("Secondary body copy for descriptions and table content.")
                .kind(ui::TypographyType::BodySmall)
                .color(ui::TypographyColor::Muted)
                .width(Fill)
                .on_copy(Message::TypographyCopied),
            ui::Typography::new("Caption, badge helper text, and fine print.")
                .kind(ui::TypographyType::BodyExtraSmall)
                .color(ui::TypographyColor::Muted)
                .on_copy(Message::TypographyCopied),
            ui::Typography::code("cargo add iced").on_copy(Message::TypographyCopied),
            ui::Typography::new("Centered semantic text")
                .kind(ui::TypographyType::BodySmall)
                .weight(ui::TypographyWeight::Medium)
                .align(ui::TypographyAlign::Center)
                .width(Fill)
                .on_copy(Message::TypographyCopied),
        ]
        .spacing(12)
        .width(Fill);

        self.component_card(
            "Typography",
            "Semantic heading, paragraph, caption, and inline-code primitives using local fonts.",
            row![heading_scale, body_scale]
                .spacing(28)
                .align_y(Alignment::Start),
        )
    }

    fn scroll_shadow_showcase(&self) -> Element<'_, Message> {
        let vertical_items: Vec<Element<'_, Message>> = (1..=9)
            .map(|index| {
                container(
                    row![
                        container(text(format!("{index:02}")).size(10).font(fonts::BOLD)).width(28),
                        column![
                            text(format!("Component token {index}"))
                                .size(12)
                                .font(fonts::MEDIUM),
                            text("Shared spacing and semantic color")
                                .size(10)
                                .font(fonts::REGULAR)
                                .color(ui::INK_MUTED),
                        ]
                        .spacing(2),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                )
                .height(48)
                .padding([4, 8])
                .into()
            })
            .collect();
        let horizontal_items: Vec<Element<'_, Message>> = (1..=8)
            .map(|index| {
                container(
                    column![
                        text(format!("Panel {index}")).size(12).font(fonts::BOLD),
                        text("Scrollable item")
                            .size(10)
                            .font(fonts::REGULAR)
                            .color(ui::INK_MUTED),
                    ]
                    .spacing(4),
                )
                .width(132)
                .height(74)
                .padding(12)
                .style(|_| container::Style {
                    border: iced::Border {
                        color: ui::LINE,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..container::Style::default()
                })
                .into()
            })
            .collect();

        let vertical: Element<'_, Message> =
            ui::ScrollShadow::new(column(vertical_items).spacing(2).width(Fill))
                .height(166)
                .hide_scrollbar(true)
                .size(32.0)
                .into();
        let horizontal: Element<'_, Message> =
            ui::ScrollShadow::new(row(horizontal_items).spacing(10).height(74))
                .orientation(ui::ScrollShadowOrientation::Horizontal)
                .height(74)
                .hide_scrollbar(true)
                .size(32.0)
                .into();

        self.component_card(
            "ScrollShadow",
            "Automatic overflow fades follow the current scroll boundary in either direction.",
            column![
                row![
                    column![
                        text("Vertical")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        vertical,
                    ]
                    .spacing(8)
                    .width(Fill),
                    column![
                        text("Visibility")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        ui::Typography::new(
                            "The upper fade appears after leaving the start; the lower fade clears at the end.",
                        )
                        .kind(ui::TypographyType::BodySmall)
                        .color(ui::TypographyColor::Muted)
                        .width(Fill)
                        .on_copy(Message::TypographyCopied),
                    ]
                    .spacing(8)
                    .width(Fill),
                ]
                .spacing(24)
                .align_y(Alignment::Start),
                text("Horizontal")
                    .size(11)
                    .font(fonts::MEDIUM)
                    .color(ui::INK_MUTED),
                horizontal,
            ]
            .spacing(10),
        )
    }

    fn card_and_toast_showcase(&self) -> Element<'_, Message> {
        let flexible_card: Element<'_, Message> = ui::Card::new(
            column![
                row![
                    icons::icon(Icon::Layers3, 17, ui::BLUE_600),
                    text("Header, content, and footer compose independently.")
                        .size(11)
                        .font(fonts::REGULAR)
                        .color(ui::INK_MUTED),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                ui::chip(
                    "Default surface",
                    Some(Icon::PanelTop),
                    ui::BLUE_600,
                    ui::ChipVariant::Flat,
                ),
            ]
            .spacing(10),
        )
        .header(
            column![
                text("Reusable workspace").size(15).font(fonts::BOLD),
                text("Default card")
                    .size(11)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            ]
            .spacing(2),
        )
        .footer(
            row![
                space::horizontal(),
                button(button_text("Open", 12.0))
                    .on_press(Message::Action("Card action fired"))
                    .height(32)
                    .padding([6, 12])
                    .style(ui::button_style(ui::ButtonVariant::Primary)),
            ]
            .align_y(Alignment::Center),
        )
        .variant(ui::CardVariant::Default)
        .width(Fill)
        .into();

        let semantic_card: Element<'_, Message> = ui::Card::new(
            column![
                row![
                    container(icons::icon(Icon::CloudCog, 18, ui::CYAN_500))
                        .width(36)
                        .height(36)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(ui::tag_style(ui::CYAN_500)),
                    column![
                        text("12 components synced").size(12).font(fonts::MEDIUM),
                        text("Updated a moment ago")
                            .size(10)
                            .font(fonts::REGULAR)
                            .color(ui::INK_MUTED),
                    ]
                    .spacing(2),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
                ui::ProgressBar::new(76.0)
                    .color(ui::ProgressBarColor::Success)
                    .size(ui::ProgressBarSize::Small),
            ]
            .spacing(12),
        )
        .header(
            column![
                text("Component sync").size(15).font(fonts::BOLD),
                text("Secondary card")
                    .size(11)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            ]
            .spacing(2),
        )
        .variant(ui::CardVariant::Secondary)
        .width(Fill)
        .into();

        let transparent_card: Element<'_, Message> = ui::Card::new(
            row![
                icons::icon(Icon::Info, 16, ui::BLUE_600),
                text("Transparent cards group content without adding another surface.")
                    .size(11)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .variant(ui::CardVariant::Transparent)
        .padding(4)
        .width(Fill)
        .into();

        let toast_controls: Element<'_, Message> = ui::Card::new(
            column![
                pick_list(
                    ui::ToastPlacement::ALL,
                    Some(self.toast_placement),
                    Message::ToastPlacementSelected,
                )
                .padding([7, 11])
                .text_size(12)
                .text_line_height(LineHeight::Absolute(Pixels(18.0)))
                .font(fonts::REGULAR)
                .handle(ui::pick_list_handle())
                .width(200)
                .style(ui::pick_list_style)
                .menu_style(ui::pick_list_menu_style),
                row![
                    button(button_text("Default", 12.0))
                        .on_press(Message::ShowToast(ToastDemo::Default))
                        .height(32)
                        .padding([6, 11])
                        .style(ui::button_style(ui::ButtonVariant::Secondary)),
                    button(button_text("Success", 12.0))
                        .on_press(Message::ShowToast(ToastDemo::Success))
                        .height(32)
                        .padding([6, 11])
                        .style(ui::button_style(ui::ButtonVariant::Secondary)),
                    button(button_text("Warning", 12.0))
                        .on_press(Message::ShowToast(ToastDemo::Warning))
                        .height(32)
                        .padding([6, 11])
                        .style(ui::button_style(ui::ButtonVariant::Secondary)),
                    button(button_text("Danger", 12.0))
                        .on_press(Message::ShowToast(ToastDemo::Danger))
                        .height(32)
                        .padding([6, 11])
                        .style(ui::button_style(ui::ButtonVariant::DangerSoft)),
                    button(button_text("With action", 12.0))
                        .on_press(Message::ShowToast(ToastDemo::Interactive))
                        .height(32)
                        .padding([6, 11])
                        .style(ui::button_style(ui::ButtonVariant::Primary)),
                ]
                .spacing(7)
                .wrap(),
            ]
            .spacing(11),
        )
        .header(
            column![
                text("Toast").size(15).font(fonts::BOLD),
                text("Positioned, temporary, and optionally interactive")
                    .size(11)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            ]
            .spacing(2),
        )
        .variant(ui::CardVariant::Tertiary)
        .width(Fill)
        .into();

        column![
            column![
                text("Card & toast").size(17).font(fonts::BOLD),
                text("Composable surfaces and temporary interactive feedback.")
                    .size(12)
                    .font(fonts::REGULAR)
                    .color(ui::INK_MUTED),
            ]
            .spacing(3),
            row![flexible_card, semantic_card]
                .spacing(14)
                .align_y(Alignment::Start),
            row![transparent_card, toast_controls]
                .spacing(14)
                .align_y(Alignment::Center),
        ]
        .spacing(12)
        .width(Fill)
        .into()
    }

    fn selection_and_navigation_showcase(&self) -> Element<'_, Message> {
        let tag_icons = [Icon::Palette, Icon::Code2, Icon::Monitor, Icon::Sparkles];
        let tag_items = self
            .tag_labels
            .iter()
            .enumerate()
            .map(|(index, label)| {
                ui::TagGroupItem::new(
                    label,
                    Some(tag_icons[index.min(tag_icons.len() - 1)]),
                    self.tag_selected.get(index).copied().unwrap_or(false),
                )
                .removable(true)
            })
            .collect();
        let tag_group = ui::tag_group(
            iced::widget::Id::new("tag-group-demo"),
            tag_items,
            self.tag_focus,
            self.keyboard_scope.is_none() || self.keyboard_scope == Some(KeyboardScope::TagGroup),
            |index| Message::ControlFocused(KeyboardScope::TagGroup, index),
            |index| Message::ControlActivated(KeyboardScope::TagGroup, index),
            Message::RemoveTag,
        );

        let toggle_foreground = if self.standalone_toggle {
            ui::WHITE
        } else {
            ui::INK_MUTED
        };
        let standalone_toggle = ui::toggle_button(
            iced::widget::Id::new("toggle-button-demo"),
            row![
                icons::icon(Icon::Heart, 15, toggle_foreground),
                text(if self.standalone_toggle {
                    "Liked"
                } else {
                    "Like"
                })
                .size(11)
                .font(fonts::MEDIUM),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
            self.standalone_toggle,
            self.keyboard_scope.is_none()
                || self.keyboard_scope == Some(KeyboardScope::StandaloneToggle),
            Message::ControlFocused(KeyboardScope::StandaloneToggle, 0),
            Message::ControlActivated(KeyboardScope::StandaloneToggle, 0),
            ui::ToggleButtonVariant::Default,
        );

        let alignment_items = [
            ("Left", Icon::AlignLeft),
            ("Center", Icon::AlignCenter),
            ("Right", Icon::AlignRight),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (label, icon))| {
            ui::ToggleButtonGroupItem::new(Some(label), Some(icon), self.alignment == index)
        })
        .collect();
        let alignment_group = ui::toggle_button_group(
            iced::widget::Id::new("alignment-toggle-group"),
            alignment_items,
            self.alignment_focus,
            self.keyboard_scope.is_none()
                || self.keyboard_scope == Some(KeyboardScope::AlignmentGroup),
            ui::SelectionMode::Single,
            ui::Orientation::Horizontal,
            false,
            |index| Message::ControlFocused(KeyboardScope::AlignmentGroup, index),
            |index| Message::ControlActivated(KeyboardScope::AlignmentGroup, index),
        );

        let formatting_items = [Icon::Bold, Icon::Italic, Icon::Underline]
            .into_iter()
            .enumerate()
            .map(|(index, icon)| {
                ui::ToggleButtonGroupItem::new(None, Some(icon), self.formatting[index])
            })
            .collect();
        let formatting_group = ui::toggle_button_group(
            iced::widget::Id::new("formatting-toggle-group"),
            formatting_items,
            self.formatting_focus,
            self.keyboard_scope.is_none()
                || self.keyboard_scope == Some(KeyboardScope::FormattingGroup),
            ui::SelectionMode::Multiple,
            ui::Orientation::Horizontal,
            true,
            |index| Message::ControlFocused(KeyboardScope::FormattingGroup, index),
            |index| Message::ControlActivated(KeyboardScope::FormattingGroup, index),
        );

        let toolbar_icons = [
            Icon::Undo2,
            Icon::Redo2,
            Icon::Bold,
            Icon::Italic,
            Icon::Underline,
        ];
        let toolbar_items = toolbar_icons
            .into_iter()
            .enumerate()
            .map(|(index, icon)| {
                let selected = index
                    .checked_sub(2)
                    .and_then(|format_index| self.formatting.get(format_index))
                    .copied()
                    .unwrap_or(false);
                button(
                    container(icons::icon(
                        icon,
                        15,
                        if selected { ui::WHITE } else { ui::INK_MUTED },
                    ))
                    .width(Fill)
                    .height(Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                )
                .on_press(Message::ControlActivated(KeyboardScope::Toolbar, index))
                .width(34)
                .height(34)
                .padding(0)
                .style(ui::button_style(if selected {
                    ui::ButtonVariant::Primary
                } else {
                    ui::ButtonVariant::Ghost
                }))
                .into()
            })
            .collect();
        let toolbar = ui::toolbar(
            iced::widget::Id::new("toolbar-demo"),
            toolbar_items,
            self.toolbar_focus,
            self.keyboard_scope.is_none() || self.keyboard_scope == Some(KeyboardScope::Toolbar),
            ui::Orientation::Horizontal,
            true,
            |index| Message::ControlFocused(KeyboardScope::Toolbar, index),
            |index| Message::ControlActivated(KeyboardScope::Toolbar, index),
        );

        let tooltip_enabled =
            self.modal.is_none() && self.global_message.is_none() && self.toasts.is_empty();
        let tooltip = ui::tooltip(
            iced::widget::Id::new("tooltip-demo"),
            button(
                container(icons::icon(Icon::Info, 15, ui::BLUE_600))
                    .width(Fill)
                    .height(Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            )
            .on_press(Message::Noop)
            .width(34)
            .height(34)
            .padding(0)
            .style(ui::button_style(ui::ButtonVariant::Secondary)),
            "Hover or focus for component details",
            ui::TooltipPlacement::Top,
            tooltip_enabled,
        );

        self.component_card(
            "Selection & navigation",
            "Focusable collections and tool controls share consistent keyboard behavior.",
            column![
                row![
                    column![
                        text("TagGroup")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        tag_group,
                    ]
                    .spacing(8)
                    .width(Fill),
                    column![
                        text("ToggleButton & Tooltip")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        row![standalone_toggle, tooltip]
                            .spacing(8)
                            .align_y(Alignment::Center),
                    ]
                    .spacing(8),
                ]
                .spacing(18)
                .align_y(Alignment::Start),
                row![
                    column![
                        text("Single selection")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        alignment_group,
                    ]
                    .spacing(8),
                    column![
                        text("Multiple selection")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        formatting_group,
                    ]
                    .spacing(8),
                    space::horizontal(),
                    column![
                        text("Toolbar")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        toolbar,
                    ]
                    .spacing(8),
                ]
                .spacing(18)
                .align_y(Alignment::Start),
                column![
                    row![
                        text("Pagination")
                            .size(11)
                            .font(fonts::MEDIUM)
                            .color(ui::INK_MUTED),
                        space::horizontal(),
                        text(format!(
                            "Page {} of {}",
                            self.pagination_page, PAGINATION_DEMO_TOTAL_PAGES
                        ))
                        .size(11)
                        .font(fonts::REGULAR)
                        .color(ui::INK_MUTED),
                    ]
                    .align_y(Alignment::Center),
                    ui::pagination(
                        self.pagination_page,
                        PAGINATION_DEMO_TOTAL_PAGES,
                        Message::PaginationChanged,
                    ),
                ]
                .spacing(8)
                .width(Fill),
            ]
            .spacing(16),
        )
    }

    fn component_card<'a>(
        &self,
        title: &'a str,
        description: &'a str,
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        ui::Card::new(content)
            .header(
                column![
                    text(title).size(17).font(fonts::BOLD),
                    text(description).size(12).color(ui::INK_MUTED),
                    rule::horizontal(1)
                ]
                .spacing(9),
            )
            .width(Fill)
            .into()
    }

    fn component_menu_items() -> Vec<ui::MenuItem<'static, Message>> {
        vec![
            ui::MenuItem::new(
                "Duplicate",
                Some(Icon::Copy),
                Message::DropdownSelected(DropdownAction::Duplicate),
            ),
            ui::MenuItem::new(
                "Rename",
                Some(Icon::Pencil),
                Message::DropdownSelected(DropdownAction::Rename),
            ),
            ui::MenuItem::danger(
                "Delete",
                Some(Icon::Trash2),
                Message::DropdownSelected(DropdownAction::Delete),
            ),
        ]
    }

    fn badge(&self, label: &'static str, color: iced::Color) -> Element<'_, Message> {
        container(
            text(label)
                .size(12)
                .font(fonts::REGULAR)
                .line_height(LineHeight::Absolute(Pixels(20.0)))
                .color(color),
        )
        .height(24)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .padding([2, 8])
        .style(ui::tag_style(color))
        .into()
    }

    fn tokens_page(&self) -> Element<'_, Message> {
        let swatches = [
            ("blue-700", ui::BLUE_700),
            ("blue-600", ui::BLUE_600),
            ("blue-500", ui::BLUE_500),
            ("cyan-500", ui::CYAN_500),
            ("cyan-300", ui::CYAN_300),
            ("navy-950", ui::NAVY_950),
        ];
        let swatch_row = swatches
            .into_iter()
            .map(|(name, color)| {
                container(
                    column![
                        container(space::vertical().height(44))
                            .width(Fill)
                            .style(move |_| container::Style {
                                background: Some(iced::Background::Color(color)),
                                border: iced::Border {
                                    radius: 8.0.into(),
                                    ..iced::Border::default()
                                },
                                ..container::Style::default()
                            }),
                        text(name).size(10).font(fonts::MEDIUM),
                        text(format!(
                            "#{:02X}{:02X}{:02X}",
                            (color.r * 255.0) as u8,
                            (color.g * 255.0) as u8,
                            (color.b * 255.0) as u8
                        ))
                        .size(10)
                        .color(ui::INK_MUTED),
                    ]
                    .spacing(7),
                )
                .padding(8)
                .width(Fill)
                .style(ui::flat_card)
            })
            .fold(row![].spacing(10).width(Fill), |row, swatch| {
                row.push(swatch)
            });

        column![
            text("Tokens").size(32).font(fonts::BLACK),
            text("A compact source of truth for color, surface, spacing, and type.")
                .size(14)
                .color(ui::INK_MUTED),
            rule::horizontal(1),
            container(
                column![text("Color palette").size(17).font(fonts::BOLD), swatch_row].spacing(14)
            )
            .padding(20)
            .width(Fill)
            .style(ui::card),
            row![
                self.token_card(
                    "TYPE SCALE",
                    column![
                        text("Display / 32").size(26).font(fonts::BLACK),
                        text("Heading / 20").size(20).font(fonts::BOLD),
                        text("Body / 14").size(14),
                        text("Caption / 11").size(11).color(ui::INK_MUTED)
                    ]
                    .spacing(9)
                ),
                self.token_card(
                    "RADIUS",
                    column![
                        text("12 px")
                            .size(25)
                            .font(fonts::BLACK)
                            .color(ui::BLUE_700),
                        text("Fields / chips / progress")
                            .size(11)
                            .color(ui::INK_MUTED),
                        text("24 px").size(18).font(fonts::BOLD).color(ui::CYAN_500),
                        text("Buttons / cards / alerts / tabs")
                            .size(11)
                            .color(ui::INK_MUTED)
                    ]
                    .spacing(8)
                ),
                self.token_card(
                    "SPACING",
                    column![
                        text("4 / 8 / 12 / 16 / 24").size(16).font(fonts::BOLD),
                        text("Use the same rhythm for every layout.")
                            .size(11)
                            .color(ui::INK_MUTED),
                        container(space::vertical().height(8))
                            .width(Fill)
                            .style(|_| container::Style {
                                background: Some(iced::Background::Color(ui::CYAN_500)),
                                border: iced::Border {
                                    radius: 4.0.into(),
                                    ..iced::Border::default()
                                },
                                ..container::Style::default()
                            })
                    ]
                    .spacing(12)
                ),
            ]
            .spacing(16),
        ]
        .spacing(22)
        .padding([34, 42])
        .width(Fill)
        .into()
    }

    fn token_card<'a>(
        &self,
        title: &'a str,
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        container(
            column![
                text(title)
                    .size(10)
                    .font(fonts::MEDIUM)
                    .color(ui::INK_MUTED),
                rule::horizontal(1),
                content.into()
            ]
            .spacing(12),
        )
        .padding(18)
        .width(Fill)
        .style(ui::card)
        .into()
    }

    fn patterns_page(&self) -> Element<'_, Message> {
        let now = Instant::now();
        let press = |id| self.motion.press_progress(id, now);
        let labels = ["Overview", "Usage", "Accessibility"];
        let tabs =
            labels
                .into_iter()
                .enumerate()
                .fold(row![].spacing(3), |tabs, (index, label)| {
                    tabs.push(
                        button(button_text(label, 12.0))
                            .on_press(Message::TabSelected(index))
                            .height(32)
                            .padding([6, 12])
                            .style(ui::tab_animated(
                                self.active_tab == index,
                                press(if index == 0 {
                                    "tab-0"
                                } else if index == 1 {
                                    "tab-1"
                                } else {
                                    "tab-2"
                                }),
                            )),
                    )
                });
        let content = match self.active_tab {
            0 => column![text("Composable patterns").size(18).font(fonts::BOLD), text("Cards, toolbars, and settings rows are assembled from the same primitives shown on the Components page.").size(12).color(ui::INK_MUTED), row![self.pattern_tile(Icon::Command, "Command bar", "Search, actions, and keyboard hints"), self.pattern_tile(Icon::PanelTop, "Settings row", "Label, description, and trailing control")].spacing(14)].spacing(14),
            1 => column![text("Usage recipe").size(18).font(fonts::BOLD), container(text("button(text(\"Save\")).style(ui::button_style(ui::ButtonVariant::Primary))").size(12).font(fonts::MEDIUM).color(ui::CYAN_300)).padding(16).width(Fill).style(ui::code_block), text("Import the module once, then use semantic variants in each screen.").size(12).color(ui::INK_MUTED)].spacing(14),
            _ => column![text("Accessibility defaults").size(18).font(fonts::BOLD), text("Focus borders, disabled colors, readable contrast, and explicit labels are part of the local styles. Keep labels attached to controls and use icons with tooltips in product screens.").size(12).color(ui::INK_MUTED)].spacing(14),
        };
        column![
            text("Patterns").size(32).font(fonts::BLACK),
            text("Small compositions that make product screens feel coherent.")
                .size(14)
                .color(ui::INK_MUTED),
            container(tabs).padding(4).style(ui::flat_card),
            container(content).padding(20).width(Fill).style(ui::card)
        ]
        .spacing(22)
        .padding([34, 42])
        .width(Fill)
        .into()
    }

    fn pattern_tile(
        &self,
        glyph: Icon,
        title: &'static str,
        description: &'static str,
    ) -> Element<'_, Message> {
        container(
            row![
                container(icons::icon(glyph, 18, ui::BLUE_600))
                    .padding(10)
                    .style(ui::tint),
                column![
                    text(title).size(13).font(fonts::MEDIUM),
                    text(description).size(11).color(ui::INK_MUTED)
                ]
                .spacing(4)
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .padding(14)
        .width(Fill)
        .style(ui::flat_card)
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DropdownAction, KeyboardScope, Launcher, Message, ModalKind, PAGINATION_DEMO_TOTAL_PAGES,
        Page, ToastDemo,
    };
    use crate::ui;
    use iced::{Point, time::Duration, time::Instant};

    #[test]
    fn controls_update_without_side_effects() {
        let (mut launcher, _) = Launcher::new();
        let _ = launcher.update(Message::ToggleChanged(true));
        let _ = launcher.update(Message::InputChanged("hello".to_owned()));
        assert!(launcher.toggled);
        assert_eq!(launcher.input, "hello");
    }

    #[test]
    fn radio_selection_is_independent_from_tabs() {
        let (mut launcher, _) = Launcher::new();
        let original_tab = launcher.active_tab;

        let _ = launcher.update(Message::RadioChanged(1));

        assert_eq!(launcher.radio_choice, 1);
        assert_eq!(launcher.active_tab, original_tab);
    }

    #[test]
    fn disclosure_and_dropdown_actions_update_independent_state() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::DisclosureToggled(false));
        assert!(!launcher.disclosure_open);

        let _ = launcher.update(Message::DropdownToggled(true));
        assert!(launcher.dropdown_open);

        let context_position = Point::new(120.0, 240.0);
        let _ = launcher.update(Message::ContextMenuOpened(context_position));
        assert!(!launcher.dropdown_open);
        assert_eq!(launcher.context_menu_position, Some(context_position));

        let _ = launcher.update(Message::DropdownSelected(DropdownAction::Delete));
        assert!(!launcher.dropdown_open);
        assert!(launcher.context_menu_position.is_none());
        assert_eq!(
            launcher.global_message.as_ref().map(|message| message.kind),
            Some(ui::MessageKind::Info)
        );
        assert_eq!(
            launcher
                .global_message
                .as_ref()
                .map(|message| message.description.as_str()),
            Some("Component deleted")
        );
    }

    #[test]
    fn accordion_alert_and_alert_dialog_state_are_controlled() {
        let (mut launcher, _) = Launcher::new();

        assert_eq!(launcher.accordion_expanded, [true, false, false]);
        let _ = launcher.update(Message::AccordionToggled(1));
        assert_eq!(launcher.accordion_expanded, [false, true, false]);

        assert!(launcher.show_demo_alert);
        let _ = launcher.update(Message::DismissDemoAlert);
        assert!(!launcher.show_demo_alert);
        let _ = launcher.update(Message::RestoreDemoAlert);
        assert!(launcher.show_demo_alert);

        let _ = launcher.update(Message::OpenModal(ModalKind::Confirmation));
        assert_eq!(launcher.modal, Some(ModalKind::Confirmation));
        let _ = launcher.update(Message::ConfirmModal);
        assert!(launcher.modal.is_none());
        assert_eq!(
            launcher.global_message.as_ref().map(|message| message.kind),
            Some(ui::MessageKind::Warning)
        );
    }

    #[test]
    fn global_layers_clear_regular_popup_state() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::DropdownToggled(true));
        let _ = launcher.update(Message::OpenModal(ModalKind::Confirmation));
        assert!(!launcher.dropdown_open);
        assert!(launcher.context_menu_position.is_none());

        let _ = launcher.update(Message::ContextMenuOpened(Point::new(80.0, 120.0)));
        let _ = launcher.update(Message::Action("Global notice"));
        assert!(!launcher.dropdown_open);
        assert!(launcher.context_menu_position.is_none());
        assert!(launcher.global_message.is_some());
    }

    #[test]
    fn navigation_is_retained() {
        let (mut launcher, _) = Launcher::new();
        let _ = launcher.update(Message::Navigate(Page::Tokens));
        assert_eq!(launcher.page, Page::Tokens);
    }

    #[test]
    fn slider_updates_without_endpoint_lock() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::SliderChanged(0.0));
        assert_eq!(launcher.slider, 0.0);

        let _ = launcher.update(Message::SliderChanged(1.0));
        assert_eq!(launcher.slider, 1.0);

        let _ = launcher.update(Message::SliderChanged(99.0));
        assert_eq!(launcher.slider, 99.0);

        let _ = launcher.update(Message::SliderChanged(100.0));
        assert_eq!(launcher.slider, 100.0);

        let _ = launcher.update(Message::SliderChanged(98.0));
        assert_eq!(launcher.slider, 98.0);
    }

    #[test]
    fn pagination_updates_and_clamps_to_the_available_pages() {
        let (mut launcher, _) = Launcher::new();

        assert_eq!(launcher.pagination_page, 6);
        let _ = launcher.update(Message::PaginationChanged(11));
        assert_eq!(launcher.pagination_page, 11);

        let _ = launcher.update(Message::PaginationChanged(0));
        assert_eq!(launcher.pagination_page, 1);

        let _ = launcher.update(Message::PaginationChanged(usize::MAX));
        assert_eq!(launcher.pagination_page, PAGINATION_DEMO_TOTAL_PAGES);
    }

    #[test]
    fn indeterminate_progress_advances_on_the_component_page() {
        let (mut launcher, _) = Launcher::new();
        let start = Instant::now();

        let _ = launcher.update(Message::Tick(start));
        let _ = launcher.update(Message::Tick(start + Duration::from_millis(750)));
        assert!((launcher.indeterminate_progress - 0.5).abs() < f32::EPSILON);
        assert!((launcher.indeterminate_circle_progress - 0.75).abs() < f32::EPSILON);

        let _ = launcher.update(Message::Navigate(Page::Tokens));
        assert!(launcher.progress_last_tick.is_none());
    }

    #[test]
    fn global_message_can_expire_or_be_dismissed() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::Action("Saved"));
        let expires_at = launcher.global_message.as_ref().unwrap().expires_at;
        let _ = launcher.update(Message::Tick(expires_at + Duration::from_millis(1)));
        assert!(launcher.global_message.is_none());

        let _ = launcher.update(Message::Action("Saved again"));
        let _ = launcher.update(Message::DismissGlobalNotice);
        assert!(launcher.global_message.is_none());
    }

    #[test]
    fn toast_queue_supports_placement_actions_and_expiration() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::ToastPlacementSelected(ui::ToastPlacement::TopEnd));
        assert_eq!(launcher.toast_placement, ui::ToastPlacement::TopEnd);

        let _ = launcher.update(Message::ShowToast(ToastDemo::Default));
        let _ = launcher.update(Message::ShowToast(ToastDemo::Success));
        let _ = launcher.update(Message::ShowToast(ToastDemo::Warning));
        let _ = launcher.update(Message::ShowToast(ToastDemo::Interactive));
        assert_eq!(launcher.toasts.len(), 3);

        let interactive_id = launcher.toasts.last().unwrap().id;
        let _ = launcher.update(Message::ToastAction(interactive_id));
        assert_eq!(launcher.toasts.len(), 3);
        assert_eq!(
            launcher.toasts.last().map(|toast| toast.variant),
            Some(ui::ToastVariant::Success)
        );

        let expires_at = launcher
            .toasts
            .iter()
            .map(|toast| toast.expires_at)
            .max()
            .unwrap();
        let _ = launcher.update(Message::Tick(expires_at + Duration::from_millis(1)));
        assert!(launcher.toasts.is_empty());
    }

    #[test]
    fn typography_copy_feedback_uses_a_success_toast() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::TypographyCopied);

        let toast = launcher.toasts.last().unwrap();
        assert_eq!(toast.title, "复制成功");
        assert_eq!(toast.description, "选中的文本已复制到剪贴板。");
        assert_eq!(toast.variant, ui::ToastVariant::Success);
        assert_eq!(launcher.toast_placement, ui::ToastPlacement::Top);
    }

    #[test]
    fn keyboard_collections_select_toggle_and_remove_items() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::ControlActivated(
            KeyboardScope::StandaloneToggle,
            0,
        ));
        assert!(launcher.standalone_toggle);

        let _ = launcher.update(Message::ControlFocused(KeyboardScope::TagGroup, 2));
        let _ = launcher.update(Message::ControlActivated(KeyboardScope::TagGroup, 2));
        assert_eq!(launcher.tag_focus, 2);
        assert!(launcher.tag_selected[2]);

        let _ = launcher.update(Message::ControlActivated(KeyboardScope::AlignmentGroup, 2));
        assert_eq!(launcher.alignment, 2);

        let _ = launcher.update(Message::ControlActivated(KeyboardScope::FormattingGroup, 1));
        assert!(launcher.formatting[1]);

        let original_len = launcher.tag_labels.len();
        let _ = launcher.update(Message::RemoveTag(2));
        assert_eq!(launcher.tag_labels.len(), original_len - 1);
        assert_eq!(launcher.tag_selected.len(), launcher.tag_labels.len());

        let _ = launcher.update(Message::FocusNext);
        assert_eq!(launcher.keyboard_scope, None);
    }

    #[test]
    fn form_modal_validates_before_confirming() {
        let (mut launcher, _) = Launcher::new();

        let _ = launcher.update(Message::OpenModal(ModalKind::Form));
        let _ = launcher.update(Message::ConfirmModal);
        assert_eq!(launcher.modal, Some(ModalKind::Form));
        assert_eq!(
            launcher.global_message.as_ref().map(|message| message.kind),
            Some(ui::MessageKind::Danger)
        );

        let _ = launcher.update(Message::ModalInputChanged("Launcher".to_owned()));
        let _ = launcher.update(Message::ConfirmModal);
        assert!(launcher.modal.is_none());
        assert_eq!(
            launcher.global_message.as_ref().map(|message| message.kind),
            Some(ui::MessageKind::Success)
        );
    }
}
