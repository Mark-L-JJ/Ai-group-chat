# 复调 · Fuga

一场由几个 AI 声部合唱的群聊。Claude、ChatGPT、Gemini、DeepSeek 坐在合唱席上，
你在指挥台起一个主题，它们轮流登台、互相接话、互相反驳。

- **舞台**：一次只演一段，谁开口谁的席位亮起，聚光灯跟着走；← → 可以一句句回看
- **乐谱**：左边是完整记录，点任意一句，舞台就演那一段
- **速度记号**：*Rubato* 自然 / *Tutti* 同时 / *Canon* 轮流；*Fuga* 让大家互相评论，*Solo* 只请一位
- **主题板**：AI 按立场整理观点，单独列出「刺耳但有用」
- **全景**：观点越被其他声部接话越靠近中心
- 通过 [OpenRouter](https://openrouter.ai) 调用模型，用你自己的 API Key（只保存在本机）
- 桌面版用 [Tauri](https://tauri.app) 打包，GitHub Actions 自动构建

## 下载

到 [Releases](../../releases) 页面下载最新的 `Fuga_*-setup.exe`。

## 项目结构

- `src/index.html` —— 整个界面和逻辑（一个文件）
- `src-tauri/` —— 把网页包装成桌面 App 的部分
- `app-icon.png` —— 图标原图（四根音柱），`npx tauri icon app-icon.png` 生成各种尺寸
