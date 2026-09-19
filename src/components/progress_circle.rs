impl ProgressCircleSize {
    const fn diameter(self) -> f32 {
        match self {
            Self::Small => 20.0,
            Self::Medium => 28.0,
            Self::Large => 36.0,
        }
    }
}
/// HeroUI 风格的确定或不确定环形进度指示器。
#[derive(Debug, Clone)]
pub struct ProgressCircle {
    value: f32,
    min_value: f32,
    max_value: f32,
    label: Option<String>,
    is_indeterminate: bool,
    animation_phase: f32,
    color: ProgressCircleColor,
    size: ProgressCircleSize,
}

impl ProgressCircle {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            min_value: 0.0,
            max_value: 100.0,
            label: None,
            is_indeterminate: false,
            animation_phase: 0.0,
            color: ProgressCircleColor::default(),
            size: ProgressCircleSize::default(),
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn range(mut self, range: std::ops::RangeInclusive<f32>) -> Self {
        (self.min_value, self.max_value) = range.into_inner();
        self
    }

    pub fn is_indeterminate(mut self, is_indeterminate: bool) -> Self {
        self.is_indeterminate = is_indeterminate;
        self
    }

    pub fn animation_phase(mut self, animation_phase: f32) -> Self {
        self.animation_phase = animation_phase;
        self
    }

    pub fn color(mut self, color: ProgressCircleColor) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: ProgressCircleSize) -> Self {
        self.size = size;
        self
    }

    fn fraction(&self) -> f32 {
        progress_fraction(self.value, self.min_value, self.max_value)
    }
}

#[derive(Debug, Clone, Copy)]
struct ProgressCircleCanvas {
    fraction: f32,
    is_indeterminate: bool,
    animation_phase: f32,
    fill: Color,
}

impl<Message> iced_canvas::Program<Message> for ProgressCircleCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<iced_canvas::Geometry> {
        // `iced_tiny_skia`（无 GPU 时的软件渲染后端，常见于云电脑/远程会话）在
        // 0.14.0 中会把 Canvas 几何的裁剪矩形错误地二次应用变换
        // （clip_bounds 已含 transformation，渲染时又乘了一次），
        // 导致 Canvas 在 Scrollable 中滚动后错位并留下残影。
        // 这里将 Frame 的裁剪边界设为无穷大，使二次变换后仍是全屏范围，
        // 裁剪回退到图层（视口）边界，从而在该后端下保持几何位置正确；
        // wgpu 后端（macOS/本地 GPU）本身不受影响，此改动对其无副作用。
        let mut frame = iced_canvas::Frame::with_bounds(renderer, Rectangle::INFINITE);
        let diameter = bounds.width.min(bounds.height);
        let stroke_width = diameter * (4.0 / 36.0);
        let radius = diameter * (16.0 / 36.0);
        // 注意：不能再用 `frame.center()`，无穷大 Frame 的中心是 NaN。
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let track = iced_canvas::Path::circle(center, radius);
        frame.stroke(
            &track,
            iced_canvas::Stroke::default()
                .with_color(LINE)
                .with_width(stroke_width),
        );

        let (start, sweep) =
            progress_circle_arc(self.fraction, self.is_indeterminate, self.animation_phase);
        if sweep > 0.0 {
            let fill = if !self.is_indeterminate && self.fraction >= 1.0 {
                iced_canvas::Path::circle(center, radius)
            } else {
                iced_canvas::Path::new(|builder| {
                    builder.arc(iced_canvas::path::Arc {
                        center,
                        radius,
                        start_angle: Radians(start),
                        end_angle: Radians(start + sweep),
                    });
                })
            };
            frame.stroke(
                &fill,
                iced_canvas::Stroke::default()
                    .with_color(self.fill)
                    .with_width(stroke_width)
                    .with_line_cap(iced_canvas::LineCap::Round),
            );
        }

        vec![frame.into_geometry()]
    }
}

impl<'a, Message> From<ProgressCircle> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(progress: ProgressCircle) -> Self {
        let diameter = progress.size.diameter();
        let circle: Element<'a, Message> = iced_canvas(ProgressCircleCanvas {
            fraction: progress.fraction(),
            is_indeterminate: progress.is_indeterminate,
            animation_phase: progress.animation_phase,
            fill: progress.color.fill(),
        })
        .width(diameter)
        .height(diameter)
        .into();

        if let Some(label) = progress.label {
            row![
                circle,
                text(label).size(12).font(crate::fonts::MEDIUM).color(INK)
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center)
            .into()
        } else {
            circle
        }
    }
}

