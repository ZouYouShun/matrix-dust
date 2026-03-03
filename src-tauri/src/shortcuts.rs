use tauri::Runtime;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

pub fn setup_shortcuts<R: Runtime>(app: &tauri::App<R>) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        use crate::window_manager::macos::Layout;

        // Ctrl+Alt (Ctrl+Option on macOS) shortcuts mapped to window layouts
        let resize_shortcuts = [
            ("Ctrl+Alt+Left", Layout::Left),
            ("Ctrl+Alt+Right", Layout::Right),
            ("Ctrl+Alt+Up", Layout::Top),
            ("Ctrl+Alt+Down", Layout::Bottom),
            ("Ctrl+Alt+U", Layout::TopLeft),
            ("Ctrl+Alt+I", Layout::TopRight),
            ("Ctrl+Alt+J", Layout::BottomLeft),
            ("Ctrl+Alt+K", Layout::BottomRight),
            ("Ctrl+Alt+D", Layout::LeftThird),
            ("Ctrl+Alt+F", Layout::CenterThird),
            ("Ctrl+Alt+G", Layout::RightThird),
            ("Ctrl+Alt+E", Layout::LeftTwoThirds),
            ("Ctrl+Alt+R", Layout::CenterTwoThirds),
            ("Ctrl+Alt+T", Layout::RightTwoThirds),
            ("Ctrl+Alt+Enter", Layout::Maximize),
            ("Ctrl+Alt+C", Layout::Center),
        ];

        for (shortcut, layout) in resize_shortcuts {
            let layout_clone = layout;
            app.global_shortcut()
                .on_shortcut(shortcut, move |_app, _shortcut, event| {
                    if event.state == ShortcutState::Released {
                        crate::window_manager::macos::resize_focused_window(layout_clone);
                    }
                })?;
        }
    }

    Ok(())
}
