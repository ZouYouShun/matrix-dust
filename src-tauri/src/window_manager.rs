#[cfg(target_os = "macos")]
pub mod macos {
    use cocoa::appkit::NSScreen;
    use cocoa::base::nil;
    use cocoa::foundation::{NSArray, NSRect};
    use core_foundation::base::{CFRelease, TCFType};
    use core_foundation::string::{CFString, CFStringRef};
    use core_graphics::geometry::{CGPoint, CGSize};
    use objc::{msg_send, sel, sel_impl};
    use serde::{Deserialize, Serialize};
    use std::ffi::c_void;

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum Layout {
        Left,
        Right,
        Top,
        Bottom,
        TopLeft,
        TopRight,
        BottomLeft,
        BottomRight,
        LeftThird,
        CenterThird,
        RightThird,
        LeftTwoThirds,
        CenterTwoThirds,
        RightTwoThirds,
        Maximize,
        Center,
    }

    type AXUIElementRef = *mut c_void;
    type AXError = i32;
    const K_AX_ERROR_SUCCESS: AXError = 0;

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn AXUIElementCreateSystemWide() -> AXUIElementRef;
        fn AXUIElementCopyAttributeValue(
            element: AXUIElementRef,
            attribute: CFStringRef,
            value: *mut *mut c_void,
        ) -> AXError;
        fn AXUIElementSetAttributeValue(
            element: AXUIElementRef,
            attribute: CFStringRef,
            value: *mut c_void,
        ) -> AXError;
        fn AXValueCreate(type_: u32, value: *const c_void) -> *mut c_void;
        fn AXValueGetValue(value: *mut c_void, type_: u32, value_out: *mut c_void) -> bool;
        fn AXUIElementCreateApplication(pid: i32) -> AXUIElementRef;
    }

    const K_AX_VALUE_CT_POINT: u32 = 1;
    const K_AX_VALUE_CT_SIZE: u32 = 2;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Rect {
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WindowInfo {
        pub window_rect: Rect,
        pub screen_visible_rect: Rect,
        pub screen_frame: Rect,
    }

    pub fn get_focused_window_info() -> Result<WindowInfo, String> {
        let (window, system_wide) = get_focused_element()?;

        unsafe {
            let k_pos_attr = CFString::new("AXPosition");
            let k_size_attr = CFString::new("AXSize");

            let mut pos_val: *mut c_void = std::ptr::null_mut();
            let mut size_val: *mut c_void = std::ptr::null_mut();

            let mut current_pos = CGPoint { x: 0.0, y: 0.0 };
            let mut current_size = CGSize {
                width: 0.0,
                height: 0.0,
            };

            if AXUIElementCopyAttributeValue(window, k_pos_attr.as_concrete_TypeRef(), &mut pos_val)
                == K_AX_ERROR_SUCCESS
                && !pos_val.is_null()
            {
                AXValueGetValue(
                    pos_val,
                    K_AX_VALUE_CT_POINT,
                    &mut current_pos as *mut _ as *mut c_void,
                );
                CFRelease(pos_val);
            }
            if AXUIElementCopyAttributeValue(
                window,
                k_size_attr.as_concrete_TypeRef(),
                &mut size_val,
            ) == K_AX_ERROR_SUCCESS
                && !size_val.is_null()
            {
                AXValueGetValue(
                    size_val,
                    K_AX_VALUE_CT_SIZE,
                    &mut current_size as *mut _ as *mut c_void,
                );
                CFRelease(size_val);
            }

            let screens = get_sorted_screens();
            let window_rect = Rect {
                x: current_pos.x,
                y: current_pos.y,
                width: current_size.width,
                height: current_size.height,
            };

            let screen_idx = find_screen_index(&screens, &window_rect);
            let (visible_rect, frame_rect) = screens[screen_idx].clone();

            cleanup_element(window);
            CFRelease(system_wide);

            Ok(WindowInfo {
                window_rect,
                screen_visible_rect: visible_rect,
                screen_frame: frame_rect,
            })
        }
    }

    pub fn set_focused_window_bounds(rect: Rect) -> Result<(), String> {
        let (window, system_wide) = get_focused_element()?;
        unsafe {
            let k_pos_attr = CFString::new("AXPosition");
            let k_size_attr = CFString::new("AXSize");

            let set_pos = |x: f64, y: f64| {
                let pos = CGPoint { x, y };
                let val = AXValueCreate(K_AX_VALUE_CT_POINT, &pos as *const _ as *const c_void);
                if !val.is_null() {
                    AXUIElementSetAttributeValue(
                        window,
                        k_pos_attr.as_concrete_TypeRef(),
                        val as *mut c_void,
                    );
                    CFRelease(val);
                }
            };

            let set_size = |w: f64, h: f64| {
                let size = CGSize {
                    width: w,
                    height: h,
                };
                let val = AXValueCreate(K_AX_VALUE_CT_SIZE, &size as *const _ as *const c_void);
                if !val.is_null() {
                    AXUIElementSetAttributeValue(
                        window,
                        k_size_attr.as_concrete_TypeRef(),
                        val as *mut c_void,
                    );
                    CFRelease(val);
                }
            };

            // Improved set_focused_window_bounds with cross-screen awareness

            // 1. Get current position to detect cross-screen jumps
            let mut current_pos = CGPoint { x: 0.0, y: 0.0 };
            let mut pos_val: *mut c_void = std::ptr::null_mut();
            if AXUIElementCopyAttributeValue(window, k_pos_attr.as_concrete_TypeRef(), &mut pos_val)
                == K_AX_ERROR_SUCCESS
                && !pos_val.is_null()
            {
                AXValueGetValue(
                    pos_val,
                    K_AX_VALUE_CT_POINT,
                    &mut current_pos as *mut _ as *mut c_void,
                );
                CFRelease(pos_val);
            }

            let screens = get_sorted_screens();
            let current_idx = find_screen_index(
                &screens,
                &Rect {
                    x: current_pos.x,
                    y: current_pos.y,
                    width: 10.0,
                    height: 10.0,
                },
            );
            let target_idx = find_screen_index(&screens, &rect);
            let is_cross_screen = current_idx != target_idx;

            println!(
                "[WindowManager] SetBounds: CrossScreen={}, TargetX={:.1}, TargetY={:.1}",
                is_cross_screen, rect.x, rect.y
            );

            if is_cross_screen {
                // Cross-screen strategy:
                // a. Shrink to move safely without monitor boundary rejection
                set_size(300.0, 300.0);
                // b. Move to new screen area
                set_pos(rect.x + 1.0, rect.y);
                set_pos(rect.x, rect.y);
                // c. CRITICAL: Allow macOS coordinate registration to complete
                std::thread::sleep(std::time::Duration::from_millis(50));
                // d. Final scale to target proportions
                set_size(rect.width, rect.height);
                set_pos(rect.x, rect.y);
            } else {
                // Single-screen strategy: Priority is speed and breaking tiling
                set_pos(rect.x + 1.0, rect.y);
                set_size(rect.width, rect.height);
                set_pos(rect.x, rect.y);
                // Double-set for apps with rigid layout engines
                set_size(rect.width, rect.height);
            }

            cleanup_element(window);
            CFRelease(system_wide);
        }
        Ok(())
    }

    pub fn resize_focused_window(layout: Layout) {
        if let Ok(info) = get_focused_window_info() {
            let screens = get_sorted_screens();
            if screens.is_empty() {
                return;
            }

            let current_screen_idx = find_screen_index(&screens, &info.window_rect);
            let (visible, _) = &screens[current_screen_idx];

            let target = calc_layout_rect(layout, visible, &info.window_rect);

            // Detailed Diagnostic Log
            println!(
                "[WindowManager] Layout={:?}, CurScreen={}, WinRect={:?}, TargetRect={:?}",
                layout, current_screen_idx, info.window_rect, target
            );

            // Smarter matching: Different apps have title bars that affect perfect height match
            let t = 45.0; // Tolerance
            let dx = (info.window_rect.x - target.x).abs();
            let dy = (info.window_rect.y - target.y).abs();
            let dw = (info.window_rect.width - target.width).abs();
            let dh = (info.window_rect.height - target.height).abs();

            let already_at_target = match layout {
                Layout::Left | Layout::Right | Layout::LeftThird | Layout::RightThird => {
                    dx < t && dw < t // Focus only on X and Width for vertical splits
                }
                Layout::Maximize => dx < t && dy < t && dw < t && dh < t,
                _ => dx < t && dy < t && dw < t && dh < t,
            };

            if already_at_target {
                if let Some(direction) = layout_screen_direction(layout) {
                    let next_idx = current_screen_idx as i32 + direction;
                    if next_idx >= 0 && (next_idx as usize) < screens.len() {
                        let (next_visible, _) = &screens[next_idx as usize];
                        let next_target = calc_layout_rect(layout, next_visible, &info.window_rect);
                        println!(
                            "[WindowManager] Cycling Screen {} -> {}",
                            current_screen_idx, next_idx
                        );
                        let _ = set_focused_window_bounds(next_target);
                        return;
                    }
                }
            }

            let _ = set_focused_window_bounds(target);
        }
    }

    fn cleanup_element(element: AXUIElementRef) {
        unsafe {
            CFRelease(element);
        }
    }

    /// Get all screens sorted by their X position (left to right), returning
    /// each screen's visible rect and full frame in AX coordinates.
    fn get_sorted_screens() -> Vec<(Rect, Rect)> {
        unsafe {
            let screens = NSScreen::screens(nil);
            let screen_count = screens.count();
            let primary_screen = screens.objectAtIndex(0);
            let primary_frame: NSRect = msg_send![primary_screen, frame];
            let primary_height = primary_frame.size.height;

            let mut result: Vec<(Rect, Rect)> = Vec::new();

            for i in 0..screen_count {
                let s = screens.objectAtIndex(i);
                let frame: NSRect = msg_send![s, frame];
                let visible_frame: NSRect = msg_send![s, visibleFrame];

                let ax_screen_x = frame.origin.x;
                let ax_screen_y = primary_height - (frame.origin.y + frame.size.height);

                let ax_visible_x = visible_frame.origin.x;
                let ax_visible_y =
                    primary_height - (visible_frame.origin.y + visible_frame.size.height);

                result.push((
                    Rect {
                        x: ax_visible_x,
                        y: ax_visible_y,
                        width: visible_frame.size.width,
                        height: visible_frame.size.height,
                    },
                    Rect {
                        x: ax_screen_x,
                        y: ax_screen_y,
                        width: frame.size.width,
                        height: frame.size.height,
                    },
                ));
            }

            // Sort by X position (left to right)
            result.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap());
            result
        }
    }

    /// Find the index of the screen that contains the given window rect.
    /// Uses max overlap area for robustness.
    fn find_screen_index(screens: &[(Rect, Rect)], window: &Rect) -> usize {
        let mut best_idx = 0;
        let mut max_overlap = -1.0;

        for (i, (_, frame)) in screens.iter().enumerate() {
            let x_overlap =
                (window.x + window.width).min(frame.x + frame.width) - window.x.max(frame.x);
            let y_overlap =
                (window.y + window.height).min(frame.y + frame.height) - window.y.max(frame.y);

            if x_overlap > 0.0 && y_overlap > 0.0 {
                let overlap_area = x_overlap * y_overlap;
                if overlap_area > max_overlap {
                    max_overlap = overlap_area;
                    best_idx = i;
                }
            }
        }

        // Fallback: use window center
        if max_overlap <= 0.0 {
            let cx = window.x + window.width / 2.0;
            let cy = window.y + window.height / 2.0;
            for (i, (_, frame)) in screens.iter().enumerate() {
                if cx >= frame.x
                    && cx <= frame.x + frame.width
                    && cy >= frame.y
                    && cy <= frame.y + frame.height
                {
                    return i;
                }
            }
        }

        best_idx
    }

    /// Calculate the target rect for a given layout on a given screen visible area.
    fn calc_layout_rect(layout: Layout, v: &Rect, current_window: &Rect) -> Rect {
        let (x, y, w, h) = match layout {
            Layout::Left => (v.x, v.y, v.width / 2.0, v.height),
            Layout::Right => (v.x + v.width / 2.0, v.y, v.width / 2.0, v.height),
            Layout::Top => (v.x, v.y, v.width, v.height / 2.0),
            Layout::Bottom => (v.x, v.y + v.height / 2.0, v.width, v.height / 2.0),
            Layout::TopLeft => (v.x, v.y, v.width / 2.0, v.height / 2.0),
            Layout::TopRight => (v.x + v.width / 2.0, v.y, v.width / 2.0, v.height / 2.0),
            Layout::BottomLeft => (v.x, v.y + v.height / 2.0, v.width / 2.0, v.height / 2.0),
            Layout::BottomRight => (
                v.x + v.width / 2.0,
                v.y + v.height / 2.0,
                v.width / 2.0,
                v.height / 2.0,
            ),
            Layout::LeftThird => (v.x, v.y, v.width / 3.0, v.height),
            Layout::CenterThird => (v.x + v.width / 3.0, v.y, v.width / 3.0, v.height),
            Layout::RightThird => (v.x + (2.0 * v.width) / 3.0, v.y, v.width / 3.0, v.height),
            Layout::LeftTwoThirds => (v.x, v.y, (2.0 * v.width) / 3.0, v.height),
            Layout::CenterTwoThirds => (v.x + v.width / 6.0, v.y, (2.0 * v.width) / 3.0, v.height),
            Layout::RightTwoThirds => (v.x + v.width / 3.0, v.y, (2.0 * v.width) / 3.0, v.height),
            Layout::Maximize => (v.x, v.y, v.width, v.height),
            Layout::Center => (
                v.x + (v.width - current_window.width) / 2.0,
                v.y + (v.height - current_window.height) / 2.0,
                current_window.width,
                current_window.height,
            ),
        };
        Rect {
            x,
            y,
            width: w,
            height: h,
        }
    }

    /// Determine the direction to cycle screens for repeating the same layout.
    /// Returns Some(offset) if the layout implies a direction, or None for Center.
    fn layout_screen_direction(layout: Layout) -> Option<i32> {
        match layout {
            // Layouts that have a "right" bias → move to next screen (+1)
            Layout::Right
            | Layout::TopRight
            | Layout::BottomRight
            | Layout::RightThird
            | Layout::RightTwoThirds => Some(1),
            // Layouts that have a "left" bias → move to previous screen (-1)
            Layout::Left
            | Layout::TopLeft
            | Layout::BottomLeft
            | Layout::LeftThird
            | Layout::LeftTwoThirds => Some(-1),
            // Neutral layouts → no screen cycling
            _ => None,
        }
    }

    fn get_focused_element() -> Result<(AXUIElementRef, AXUIElementRef), String> {
        unsafe {
            let system_wide = AXUIElementCreateSystemWide();
            if system_wide.is_null() {
                return Err("Failed to create system wide element".to_string());
            }

            let k_focused_attr = CFString::new("AXFocusedUIElement");
            let mut focused_ref: *mut c_void = std::ptr::null_mut();

            let status = AXUIElementCopyAttributeValue(
                system_wide,
                k_focused_attr.as_concrete_TypeRef(),
                &mut focused_ref,
            );

            if status == K_AX_ERROR_SUCCESS && !focused_ref.is_null() {
                println!("[WindowManager] Successfully got focused element from SystemWide");
                // 1. Try to find the Application element by traversing up or using AXApplication
                let k_parent_attr = CFString::new("AXParent");
                let k_role_attr = CFString::new("AXRole");

                let mut current = focused_ref;
                let mut app_ref: *mut c_void = std::ptr::null_mut();

                // Traverse up to find the Application element
                for _ in 0..10 {
                    let mut role_ref: *mut c_void = std::ptr::null_mut();
                    if AXUIElementCopyAttributeValue(
                        current,
                        k_role_attr.as_concrete_TypeRef(),
                        &mut role_ref,
                    ) == K_AX_ERROR_SUCCESS
                    {
                        let role_str =
                            CFString::wrap_under_create_rule(role_ref as CFStringRef).to_string();
                        if role_str == "AXApplication" {
                            println!("[WindowManager] Found AXApplication element");
                            app_ref = current;
                            break;
                        }
                        CFRelease(role_ref);
                    }

                    let mut next_parent: *mut c_void = std::ptr::null_mut();
                    if AXUIElementCopyAttributeValue(
                        current,
                        k_parent_attr.as_concrete_TypeRef(),
                        &mut next_parent,
                    ) == K_AX_ERROR_SUCCESS
                        && !next_parent.is_null()
                    {
                        if current != focused_ref && current != app_ref {
                            CFRelease(current);
                        }
                        current = next_parent;
                    } else {
                        break;
                    }
                }

                if !app_ref.is_null() {
                    // Try setting AXEnhancedUserInterface = true on the app (helps with Electron)
                    let k_enhanced_attr = CFString::new("AXEnhancedUserInterface");
                    use core_foundation::boolean::CFBoolean;
                    AXUIElementSetAttributeValue(
                        app_ref,
                        k_enhanced_attr.as_concrete_TypeRef(),
                        CFBoolean::true_value().as_CFTypeRef() as *mut c_void,
                    );

                    let k_focused_window_attr = CFString::new("AXFocusedWindow");
                    let mut window_ref: *mut c_void = std::ptr::null_mut();
                    if AXUIElementCopyAttributeValue(
                        app_ref,
                        k_focused_window_attr.as_concrete_TypeRef(),
                        &mut window_ref,
                    ) == K_AX_ERROR_SUCCESS
                        && !window_ref.is_null()
                    {
                        if app_ref != focused_ref {
                            CFRelease(app_ref);
                        }
                        if focused_ref != window_ref {
                            CFRelease(focused_ref);
                        }
                        println!("[WindowManager] Found focused window via AXApplication");
                        return Ok((window_ref as AXUIElementRef, system_wide));
                    }
                    if app_ref != focused_ref {
                        CFRelease(app_ref);
                    }
                }

                // Fallback to original logic
                let k_window_attr = CFString::new("AXWindow");
                let k_top_level_attr = CFString::new("AXTopLevelElement");
                let mut window_ref: *mut c_void = std::ptr::null_mut();

                if AXUIElementCopyAttributeValue(
                    focused_ref,
                    k_window_attr.as_concrete_TypeRef(),
                    &mut window_ref,
                ) == K_AX_ERROR_SUCCESS
                    && !window_ref.is_null()
                {
                    CFRelease(focused_ref);
                    println!("[WindowManager] Found window via AXWindow attribute");
                    return Ok((window_ref as AXUIElementRef, system_wide));
                }

                if AXUIElementCopyAttributeValue(
                    focused_ref,
                    k_top_level_attr.as_concrete_TypeRef(),
                    &mut window_ref,
                ) == K_AX_ERROR_SUCCESS
                    && !window_ref.is_null()
                {
                    CFRelease(focused_ref);
                    println!("[WindowManager] Found window via AXTopLevelElement attribute");
                    return Ok((window_ref as AXUIElementRef, system_wide));
                }

                println!("[WindowManager] Falling back to using focused element as window");
                return Ok((focused_ref as AXUIElementRef, system_wide));
            }

            println!(
                "[WindowManager] SystemWide failed (Status: {:?}). Trying NSWorkspace fallback...",
                status
            );

            // FALLBACK: Use NSWorkspace to find the frontmost application
            let workspace_class = objc::runtime::Class::get("NSWorkspace").unwrap();
            let workspace: *mut objc::runtime::Object = msg_send![workspace_class, sharedWorkspace];
            let front_app: *mut objc::runtime::Object = msg_send![workspace, frontmostApplication];
            if front_app.is_null() {
                CFRelease(system_wide);
                return Err("Failed to get frontmost application".to_string());
            }

            let pid: i32 = msg_send![front_app, processIdentifier];
            println!("[WindowManager] Frontmost app PID: {:?}", pid);

            let app_element = AXUIElementCreateApplication(pid);
            if app_element.is_null() {
                CFRelease(system_wide);
                return Err("Failed to create application element from PID".to_string());
            }

            // Try to set AXEnhancedUserInterface on this app as well
            let k_enhanced_attr = CFString::new("AXEnhancedUserInterface");
            use core_foundation::boolean::CFBoolean;
            AXUIElementSetAttributeValue(
                app_element,
                k_enhanced_attr.as_concrete_TypeRef(),
                CFBoolean::true_value().as_CFTypeRef() as *mut c_void,
            );

            let k_focused_window_attr = CFString::new("AXFocusedWindow");
            let mut window_ref: *mut c_void = std::ptr::null_mut();
            if AXUIElementCopyAttributeValue(
                app_element,
                k_focused_window_attr.as_concrete_TypeRef(),
                &mut window_ref,
            ) == K_AX_ERROR_SUCCESS
                && !window_ref.is_null()
            {
                CFRelease(app_element);
                println!("[WindowManager] Found focused window via NSWorkspace fallback");
                return Ok((window_ref as AXUIElementRef, system_wide));
            }

            CFRelease(app_element);
            CFRelease(system_wide);
            Err("Failed to find focused window even via NSWorkspace fallback".to_string())
        }
    }
}
