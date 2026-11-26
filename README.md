# AI 群聊

一个像微信群一样的 AI 聊天 App：Claude、ChatGPT、Gemini、DeepSeek 在同一个群里，会接话、会互相@、会吵架。

- 通过 [OpenRouter](https://openrouter.ai) 调用各家模型，用你自己的 API Key（只保存在本机）
- 桌面版用 [Tauri](https://tauri.app) 打包，GitHub Actions 自动构建

## 下载

到 [Releases](../../releases) 页面下载最新的 `*-setup.exe`。

## 项目结构

- `src/index.html` —— 整个群聊界面和逻辑（一个文件）
- `src-tauri/` —— 把网页包装成桌面 App 的部分
- `.github/workflows/build.yml` —— 自动打包流程
