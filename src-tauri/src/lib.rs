// App 的入口：打开一个窗口，里面显示 src/index.html（就是我们的群聊网页）
// 以后做 iOS 版时，也是从这里启动
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("启动 AI 群聊失败");
}
