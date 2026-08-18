# 更新日志

本文件记录 Astra UI 组件库的所有版本更新内容。

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/) 规范。

---
## [0.0.1] - 2026-08-18

- 初始版本
- 提供 iced 0.14 组件、设计令牌、字体与图标辅助 API。
- 增加可独立运行的 Showcase 示例。
- 新增受控 `Tabs` 组件，支持 primary/secondary 变体、水平/垂直方向、禁用项、分隔线与面板组合。
- 新增 `Surface`、`TextArea`、`Label`、`InputOTP` 和 `ListBox` 组件，覆盖语义表面、多行编辑、表单标签、验证码输入及单选/多选列表。
- 修复 `InputOTP` 连续输入与退格导航：`InputOtpChange` 提供下一槽位焦点 ID，应用可通过 `iced::widget::operation::focus` 执行焦点移动。
- 新增受控 `Drawer` 抽屉组件，支持四边停靠、遮罩变体、可滚动正文、Footer、关闭按钮和不可关闭模式。
- 为 `AlertDialog`、全局 Modal 和 `Drawer` 增加打开/关闭过渡动画，支持遮罩淡入淡出与按方向滑入滑出。
- 为全局消息和 Toast 增加出现/消失过渡动画，支持调用方传入进度并在关闭完成后清理状态。
- 调整全局消息和 Toast 的动效方向：根据所在边缘进入，并沿原路径返回；新增阶段与位置感知的 animated API。
- 全局消息改为复用 Toast 的 `ToastPlacement` 动效映射，确保实际布局位置与进入/退出方向一致。
- 修复异步替换状态时重复播放入场动画的问题：已有全局消息直接更新，交互 Toast 点击 Action 后直接替换为结果提示，不播放出入场动画。
- 新增 `Kbd` 键盘按键组件，支持 Mac/Win 修饰键映射、特殊按键、导航按键及 default/light 变体。
