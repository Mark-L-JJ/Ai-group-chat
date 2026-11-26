// 在 Windows 正式版里不弹出黑色命令行窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    ai_group_chat_lib::run()
}
