#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PaginationItem {
    Page(usize),
    Ellipsis,
}

fn pagination_items(current_page: usize, total_pages: usize) -> Vec<PaginationItem> {
    let total_pages = total_pages.max(1);
    let current_page = current_page.clamp(1, total_pages);

    if total_pages <= 7 {
        return (1..=total_pages).map(PaginationItem::Page).collect();
    }

    let mut items = vec![PaginationItem::Page(1)];
    if current_page > 3 {
        items.push(PaginationItem::Ellipsis);
    }

    let start = current_page.saturating_sub(1).max(2);
    let end = current_page.saturating_add(1).min(total_pages - 1);
    items.extend((start..=end).map(PaginationItem::Page));

    if current_page < total_pages.saturating_sub(2) {
        items.push(PaginationItem::Ellipsis);
    }
    items.push(PaginationItem::Page(total_pages));
    items
}

fn pagination_button_style(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        let disabled = matches!(status, button::Status::Disabled);

        button::Style {
            background: active.then_some(Background::Color(BLUE_600)),
            text_color: if active {
                WHITE
            } else if disabled {
                INK_SUBTLE
            } else if hovered || pressed {
                BLUE_700
            } else {
                INK_MUTED
            },
            border: Border {
                color: if active || pressed {
                    BLUE_600
                } else if disabled {
                    Color::from_rgba(LINE.r, LINE.g, LINE.b, 0.55)
                } else if hovered {
                    BLUE_500
                } else {
                    LINE
                },
                width: 1.0,
                radius: RADIUS_FIELD.into(),
            },
            ..button::Style::default()
        }
    }
}

fn pagination_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    active: bool,
    on_press: Option<Message>,
    width: f32,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let control = button(
        container(content)
            .width(Fill)
            .height(Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center),
    )
    .width(width)
    .height(PAGINATION_ITEM_SIZE)
    .padding(0)
    .style(pagination_button_style(active));

    match on_press {
        Some(message) => control.on_press(message).into(),
        None => control.into(),
    }
}

/// 创建可独立组合的分页页码链接。
pub fn pagination_link<'a, Message>(
    page: usize,
    active: bool,
    on_press: Option<Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    pagination_button(
        text(page.to_string())
            .size(12)
            .font(crate::fonts::MEDIUM)
            .line_height(iced::widget::text::LineHeight::Absolute(Pixels(18.0))),
        active,
        on_press,
        PAGINATION_ITEM_SIZE,
    )
}

fn pagination_navigation<'a, Message>(
    label: &'a str,
    icon: LucideIcon,
    icon_first: bool,
    on_press: Option<Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let icon_color = if on_press.is_some() {
        INK_MUTED
    } else {
        INK_SUBTLE
    };
    let label = text(label)
        .size(12)
        .font(crate::fonts::MEDIUM)
        .line_height(iced::widget::text::LineHeight::Absolute(Pixels(18.0)));
    let content = if icon_first {
        row![crate::icons::icon(icon, 15, icon_color), label]
    } else {
        row![label, crate::icons::icon(icon, 15, icon_color)]
    }
    .spacing(6)
    .align_y(iced::Alignment::Center);

    pagination_button(content, false, on_press, PAGINATION_NAV_WIDTH)
}

/// 创建分页的上一页按钮；传入 `None` 时按钮为禁用状态。
pub fn pagination_previous<'a, Message>(on_press: Option<Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    pagination_navigation("Previous", LucideIcon::ChevronLeft, true, on_press)
}

/// 创建分页的下一页按钮；传入 `None` 时按钮为禁用状态。
pub fn pagination_next<'a, Message>(on_press: Option<Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    pagination_navigation("Next", LucideIcon::ChevronRight, false, on_press)
}

/// 创建不参与交互的分页省略号。
pub fn pagination_ellipsis<'a, Message>() -> Element<'a, Message>
where
    Message: 'a,
{
    container(crate::icons::icon(LucideIcon::Ellipsis, 16, INK_SUBTLE))
        .width(PAGINATION_ITEM_SIZE)
        .height(PAGINATION_ITEM_SIZE)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .into()
}

/// 将自定义分页项组合为一行分页内容。
pub fn pagination_content<'a, Message>(items: Vec<Element<'a, Message>>) -> Element<'a, Message>
where
    Message: 'a,
{
    row(items)
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into()
}

/// 创建包含上一页、页码、省略号和下一页的受控分页组件。
pub fn pagination<'a, Message>(
    current_page: usize,
    total_pages: usize,
    on_change: impl Fn(usize) -> Message + Clone + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let total_pages = total_pages.max(1);
    let current_page = current_page.clamp(1, total_pages);
    let mut items = vec![pagination_previous(
        (current_page > 1).then(|| on_change.clone()(current_page - 1)),
    )];

    for item in pagination_items(current_page, total_pages) {
        items.push(match item {
            PaginationItem::Page(page) => pagination_link(
                page,
                page == current_page,
                (page != current_page).then(|| on_change.clone()(page)),
            ),
            PaginationItem::Ellipsis => pagination_ellipsis(),
        });
    }

    items.push(pagination_next(
        (current_page < total_pages).then(|| on_change(total_pages.min(current_page + 1))),
    ));
    pagination_content(items)
}

fn toolbar_surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            radius: RADIUS_CONTROL.into(),
            ..Border::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.10),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 14.0,
        },
        ..container::Style::default()
    }
}

