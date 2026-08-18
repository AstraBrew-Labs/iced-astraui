#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPlacement {
    TopStart,
    #[default]
    Top,
    TopEnd,
    BottomStart,
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

    /// Returns the offset used while a toast is entering or leaving.
    ///
    /// Centered placements use the nearest vertical edge. Corner placements
    /// use the horizontal edge. The same origin is used for both phases, so
    /// dismissal retraces the path used during entry.
    pub fn transition_offset(self, progress: f32, _closing: bool) -> iced::Vector {
        let distance = 14.0 * (1.0 - progress.clamp(0.0, 1.0));
        match self {
            Self::Top => iced::Vector::new(0.0, -distance),
            Self::Bottom => iced::Vector::new(0.0, distance),
            Self::TopStart | Self::BottomStart => iced::Vector::new(-distance, 0.0),
            Self::TopEnd | Self::BottomEnd => iced::Vector::new(distance, 0.0),
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
