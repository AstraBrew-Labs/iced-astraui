<div align="center">
  <img src="assets/icon/icon.png" alt="Astra UI" width="96" height="96">

# Astra UI

**参考 HeroUI v3 设计语言，为 iced 打造的 Rust 桌面 UI 组件库。**

用一致的视觉规范、语义化 API 和可复用交互组件，帮助你更快完成现代桌面应用界面。

[![Rust](https://img.shields.io/badge/Rust-2024-CE422B?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![iced](https://img.shields.io/badge/iced-0.14-08A5E5)](https://iced.rs/)
[![License](https://img.shields.io/badge/license-MIT-22C55E)](LICENSE)
[![Status](https://img.shields.io/badge/status-early%20development-F59E0B)](#项目状态)

</div>

## 关于 Astra UI

Astra UI 是一个面向 [iced](https://iced.rs/) 应用的本地 UI 设计系统。它借鉴 [HeroUI v3](https://www.heroui.com/) 清晰、克制的视觉语言，并结合原生桌面应用的交互方式重新实现，而不是对 Web 组件的逐项移植。

项目提供统一的颜色、圆角、间距、字号和动效规范，以及可以直接组合进 iced `view` 的组件与样式。目标是减少重复的样式代码，让应用开发更专注于业务状态和交互逻辑。

> Astra UI 是独立开发的项目，与 HeroUI 官方没有隶属或合作关系。

## 特性

- **HeroUI 风格**：简洁的表面层级、柔和圆角、语义色彩和一致的控件尺寸。
- **原生 iced 组件**：基于 Rust 与 iced 构建，不依赖 WebView 或前端运行时。
- **语义化变体**：通过 `Primary`、`Secondary`、`Ghost`、`Danger` 等变体快速表达组件用途。
- **可组合 API**：卡片、提示、菜单、弹窗等组件可以和 iced 原生 widget 一起组合。
- **完整交互状态**：覆盖 hover、pressed、focused、selected、disabled 等常见状态。
- **短时动效**：内置按钮、开关、复选框和进度组件的轻量过渡动画。
- **统一设计令牌**：集中管理颜色、圆角、控件高度和排版规范。
- **本地字体与图标**：内置 HarmonyOS Sans 字重，并使用 Lucide 图标。

## 组件

当前 Showcase 已覆盖以下组件和模式：

| 分类 | 组件 |
| --- | --- |
| 基础 | Button、Typography、Separator、Card、Avatar、Badge、Chip |
| 输入 | Text Input、Select、Checkbox、Radio、Switch、Slider |
| 导航 | Tabs、Pagination、Disclosure、Accordion |
| 操作 | Dropdown、Context Menu、Toolbar、Toggle Button、Tag Group、Tooltip |
| 反馈 | Alert、Global Message、Toast、Progress Bar、Progress Circle |
| 浮层 | Modal、Alert Dialog、Global Layer、Scroll Shadow |

组件、设计令牌和常见组合模式都可以在仓库内置的 Showcase 中查看。

## 快速开始

### 环境要求

- Rust stable
- Cargo
- macOS（当前主要开发与验证平台）

### 运行 Showcase

```bash
git clone https://github.com/AstraBrew-Labs/iced-astraui.git
cd iced-astraui
cargo run --release
```

仅检查项目是否能够正常编译：

```bash
cargo check
```

### 在项目中使用

当前版本以本地组件源码和 Showcase 的形式维护，尚未发布到 crates.io。你可以将以下模块和资源集成到现有 iced 项目中：

```text
src/ui.rs
src/fonts.rs
src/icons.rs
assets/fonts/
```

并添加对应依赖：

```toml
[dependencies]
iced = { version = "0.14.0", features = ["advanced", "canvas", "image", "tokio"] }
lucide-icons = { version = "1.31.0", features = ["iced"] }
```

一个简单的组件组合示例：

```rust
use iced::widget::{button, column, text};
use iced::{Element, Fill};

use crate::ui::{self, Alert, AlertKind, ButtonVariant, Card};

#[derive(Debug, Clone)]
enum Message {
    Save,
    Dismiss,
}

fn view() -> Element<'static, Message> {
    Card::new(
        column![
            text("工作区设置"),
            Alert::new("配置已同步")
                .description("所有更改均已保存到本地")
                .kind(AlertKind::Success)
                .on_close(Message::Dismiss),
            button("保存")
                .on_press(Message::Save)
                .style(ui::button_style(ButtonVariant::Primary)),
        ]
        .spacing(16)
        .width(Fill),
    )
    .width(Fill)
    .into()
}
```

应用主题可以直接使用 `ui::app_theme()`：

```rust
iced::application(App::new, App::update, App::view)
    .theme(|_| ui::app_theme())
    .run()
```

## 项目结构

```text
astra-ui/
├── assets/
│   ├── fonts/          # HarmonyOS Sans 字体资源
│   └── icon/           # 应用图标
├── src/
│   ├── app.rs          # Showcase 状态、消息与页面
│   ├── fonts.rs        # 字体注册与字重映射
│   ├── icons.rs        # Lucide 图标适配
│   ├── main.rs         # Showcase 入口
│   └── ui.rs           # 组件、样式、设计令牌与交互实现
├── Cargo.toml
└── README.md
```

## 开发

提交代码前请运行：

```bash
cargo fmt --check
cargo check
cargo test
```

新增组件时，请尽量保持以下原则：

- 优先复用现有设计令牌与语义色，不在组件内随意增加孤立样式。
- 同时处理默认、悬停、按下、聚焦和禁用状态。
- API 与 iced 的消息驱动模型保持一致，并支持与原生 widget 组合。
- 在 Showcase 中补充可交互示例，展示主要变体和边界状态。

## 项目状态

Astra UI 当前处于早期开发阶段，API 可能随组件完善而调整。`Cargo.toml` 暂时设置为 `publish = false`，建议在固定提交版本的前提下集成到应用中。

## 许可证

本项目基于 [MIT License](LICENSE) 开源。第三方字体与图标资源遵循各自的许可证。
