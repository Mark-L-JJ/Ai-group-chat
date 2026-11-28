// App 的入口：打开一个窗口，里面显示 src/index.html（复调的界面）
// dialog + fs 两个插件：让声部发来的文件和图片可以「另存为」到电脑上
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("启动复调失败");
}
