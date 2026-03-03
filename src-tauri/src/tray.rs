use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Manager,
};

pub fn create_tray<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    // Halves
    let left = MenuItem::with_id(app, "resize_left", "Left", true, Some("Ctrl+Option+Left"))?;
    let right = MenuItem::with_id(
        app,
        "resize_right",
        "Right",
        true,
        Some("Ctrl+Option+Right"),
    )?;
    let top = MenuItem::with_id(app, "resize_top", "Top", true, Some("Ctrl+Option+Up"))?;
    let bottom = MenuItem::with_id(
        app,
        "resize_bottom",
        "Bottom",
        true,
        Some("Ctrl+Option+Down"),
    )?;

    // Corners
    let top_left = MenuItem::with_id(
        app,
        "resize_top_left",
        "Top Left",
        true,
        Some("Ctrl+Option+U"),
    )?;
    let top_right = MenuItem::with_id(
        app,
        "resize_top_right",
        "Top Right",
        true,
        Some("Ctrl+Option+I"),
    )?;
    let bottom_left = MenuItem::with_id(
        app,
        "resize_bottom_left",
        "Bottom Left",
        true,
        Some("Ctrl+Option+J"),
    )?;
    let bottom_right = MenuItem::with_id(
        app,
        "resize_bottom_right",
        "Bottom Right",
        true,
        Some("Ctrl+Option+K"),
    )?;

    // Thirds
    let left_third = MenuItem::with_id(
        app,
        "resize_left_third",
        "Left Third",
        true,
        Some("Ctrl+Option+D"),
    )?;
    let center_third = MenuItem::with_id(
        app,
        "resize_center_third",
        "Center Third",
        true,
        Some("Ctrl+Option+F"),
    )?;
    let right_third = MenuItem::with_id(
        app,
        "resize_right_third",
        "Right Third",
        true,
        Some("Ctrl+Option+G"),
    )?;

    // Two Thirds
    let left_two_thirds = MenuItem::with_id(
        app,
        "resize_left_two_thirds",
        "Left Two Thirds",
        true,
        Some("Ctrl+Option+E"),
    )?;
    let center_two_thirds = MenuItem::with_id(
        app,
        "resize_center_two_thirds",
        "Center Two Thirds",
        true,
        Some("Ctrl+Option+R"),
    )?;
    let right_two_thirds = MenuItem::with_id(
        app,
        "resize_right_two_thirds",
        "Right Two Thirds",
        true,
        Some("Ctrl+Option+T"),
    )?;

    // Actions
    let maximize = MenuItem::with_id(
        app,
        "resize_maximize",
        "Maximize",
        true,
        Some("Ctrl+Option+Return"),
    )?;
    let center = MenuItem::with_id(app, "resize_center", "Center", true, Some("Ctrl+Option+C"))?;

    // Standard items
    let shortcuts_i =
        MenuItem::with_id(app, "shortcuts", "Shortcut Reference…", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit Matrix Dust", true, None::<&str>)?;

    let sep1 = PredefinedMenuItem::separator(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let sep3 = PredefinedMenuItem::separator(app)?;
    let sep4 = PredefinedMenuItem::separator(app)?;
    let sep5 = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(
        app,
        &[
            &left,
            &right,
            &top,
            &bottom,
            &sep1,
            &top_left,
            &top_right,
            &bottom_left,
            &bottom_right,
            &sep2,
            &left_third,
            &center_third,
            &right_third,
            &sep3,
            &left_two_thirds,
            &center_two_thirds,
            &right_two_thirds,
            &sep4,
            &maximize,
            &center,
            &sep5,
            &shortcuts_i,
            &quit_i,
        ],
    )?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(move |app, event| {
            let id = event.id.as_ref();

            if id == "quit" {
                app.exit(0);
                return;
            }

            // Show the shortcut reference window
            if id == "shortcuts" {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
                return;
            }

            #[cfg(target_os = "macos")]
            {
                use crate::window_manager::macos::Layout;
                let layout = match id {
                    "resize_left" => Some(Layout::Left),
                    "resize_right" => Some(Layout::Right),
                    "resize_top" => Some(Layout::Top),
                    "resize_bottom" => Some(Layout::Bottom),
                    "resize_top_left" => Some(Layout::TopLeft),
                    "resize_top_right" => Some(Layout::TopRight),
                    "resize_bottom_left" => Some(Layout::BottomLeft),
                    "resize_bottom_right" => Some(Layout::BottomRight),
                    "resize_left_third" => Some(Layout::LeftThird),
                    "resize_center_third" => Some(Layout::CenterThird),
                    "resize_right_third" => Some(Layout::RightThird),
                    "resize_left_two_thirds" => Some(Layout::LeftTwoThirds),
                    "resize_center_two_thirds" => Some(Layout::CenterTwoThirds),
                    "resize_right_two_thirds" => Some(Layout::RightTwoThirds),
                    "resize_maximize" => Some(Layout::Maximize),
                    "resize_center" => Some(Layout::Center),
                    _ => None,
                };

                if let Some(l) = layout {
                    crate::window_manager::macos::resize_focused_window(l);
                }
            }
        })
        .build(app)?;

    Ok(())
}
