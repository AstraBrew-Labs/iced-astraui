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

