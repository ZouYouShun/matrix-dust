mod accessibility;
mod shortcuts;
mod tray;
mod window_manager;

use tauri::Manager;

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .on_window_event(|window, event| {
            // Close button hides the window instead of quitting
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            accessibility::check_accessibility,
            accessibility::request_accessibility,
        ])
        .setup(|app| {
            // Hide from Dock and App Switcher — tray-only app
            #[cfg(target_os = "macos")]
            set_activation_policy_accessory();

            shortcuts::setup_shortcuts(app)?;
            tray::create_tray(app.handle())?;

            // Hide main window on start — this app lives in the tray
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_app_handle, _event| {});
}

/// Set NSApp activation policy to Accessory so the app does not appear
/// in the Dock or the Cmd+Tab App Switcher.
#[cfg(target_os = "macos")]
fn set_activation_policy_accessory() {
    use objc::{msg_send, sel, sel_impl};
    unsafe {
        // NSApplicationActivationPolicyAccessory = 1
        let ns_app: *mut objc::runtime::Object =
            msg_send![objc::class!(NSApplication), sharedApplication];
        let _: () = msg_send![ns_app, setActivationPolicy: 1_i64];
    }
}
