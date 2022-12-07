use tauri::{generate_context, Menu, MenuItem, Submenu};
use tauri_utils::config::{Config, WindowConfig, WindowUrl};
#[cfg(target_os = "macos")]
use wry::application::platform::macos::WindowBuilderExtMacOS;
#[cfg(target_os = "windows")]
use wry::application::platform::windows::WindowBuilderExtWindows;

fn main() {
    let first_menu = Menu::new()
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::EnterFullScreen)
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::SelectAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::CloseWindow)
        .add_native_item(MenuItem::Quit);

    let menu_bar_menu = Menu::new().add_submenu(Submenu::new("App", first_menu));

    tauri::Builder::default()
        .menu(menu_bar_menu)
        .setup(|app| {
            let WindowConfig {
                url,
                width,
                height,
                resizable,
                transparent,
                fullscreen,
                ..
            } = get_windows_config().unwrap_or_default();

            let _window = tauri::WindowBuilder::new(
                app,
                "label",
                WindowUrl::External(url.to_string().parse().unwrap()),
            )
            .initialization_script(include_str!("pake.js"))
            .build()?;
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}

fn get_windows_config() -> Option<WindowConfig> {
    let config_file = include_str!("../pake.conf.json");
    let config: Config = serde_json::from_str(config_file).expect("failed to parse windows config");

    config.tauri.windows.first().cloned()
}
