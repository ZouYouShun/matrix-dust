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

            // Find screen containing the window center
            let center_x = current_pos.x + current_size.width / 2.0;
            let center_y = current_pos.y + current_size.height / 2.0;

            let screens = NSScreen::screens(nil);
            let screen_count = screens.count();
            let primary_screen = screens.objectAtIndex(0);
            let primary_frame: NSRect = msg_send![primary_screen, frame];
            let primary_height = primary_frame.size.height;
            let cocoa_center_y = primary_height - center_y;

            let mut target_screen = primary_screen;
            let mut target_visible_frame: NSRect = msg_send![target_screen, visibleFrame];
            let mut target_frame: NSRect = msg_send![target_screen, frame];

            for i in 0..screen_count {
                let s = screens.objectAtIndex(i);
                let frame: NSRect = msg_send![s, frame];
                if center_x >= frame.origin.x
                    && center_x <= (frame.origin.x + frame.size.width)
                    && cocoa_center_y >= frame.origin.y
                    && cocoa_center_y <= (frame.origin.y + frame.size.height)
                {
                    target_screen = s;
                    target_visible_frame = msg_send![target_screen, visibleFrame];
                    target_frame = frame;
                    break;
                }
            }

            // Convert Cocoa coordinates to AX coordinates
            let ax_visible_y =
                primary_height - (target_visible_frame.origin.y + target_visible_frame.size.height);
            let ax_visible_x = target_visible_frame.origin.x;

            let ax_screen_y = primary_height - (target_frame.origin.y + target_frame.size.height);
            let ax_screen_x = target_frame.origin.x;

            cleanup_element(window);
            CFRelease(system_wide);

            Ok(WindowInfo {
                window_rect: Rect {
                    x: current_pos.x,
                    y: current_pos.y,
                    width: current_size.width,
                    height: current_size.height,
                },
                screen_visible_rect: Rect {
                    x: ax_visible_x,
                    y: ax_visible_y,
                    width: target_visible_frame.size.width,
                    height: target_visible_frame.size.height,
                },
                screen_frame: Rect {
                    x: ax_screen_x,
                    y: ax_screen_y,
                    width: target_frame.size.width,
                    height: target_frame.size.height,
                },
            })
        }
    }

    pub fn set_focused_window_bounds(rect: Rect) -> Result<(), String> {
        let (window, system_wide) = get_focused_element()?;
        unsafe {
            let k_pos_attr = CFString::new("AXPosition");
            let k_size_attr = CFString::new("AXSize");

            let new_pos_struct = CGPoint {
                x: rect.x,
                y: rect.y,
            };
            let new_size_struct = CGSize {
                width: rect.width,
                height: rect.height,
            };

            let new_pos_val = AXValueCreate(
                K_AX_VALUE_CT_POINT,
                &new_pos_struct as *const _ as *const c_void,
            );
            let new_size_val = AXValueCreate(
                K_AX_VALUE_CT_SIZE,
                &new_size_struct as *const _ as *const c_void,
            );

            if !new_size_val.is_null() {
                AXUIElementSetAttributeValue(
                    window,
                    k_size_attr.as_concrete_TypeRef(),
                    new_size_val as *mut c_void,
                );
                CFRelease(new_size_val);
            }
            if !new_pos_val.is_null() {
                AXUIElementSetAttributeValue(
                    window,
                    k_pos_attr.as_concrete_TypeRef(),
                    new_pos_val as *mut c_void,
                );
                CFRelease(new_pos_val);
            }

            cleanup_element(window);
            CFRelease(system_wide);
        }
        Ok(())
    }

    fn get_focused_element() -> Result<(AXUIElementRef, AXUIElementRef), String> {
        unsafe {
            let system_wide = AXUIElementCreateSystemWide();
            if system_wide.is_null() {
                return Err("Failed to create system wide element".to_string());
            }

            let k_focused_attr = CFString::new("AXFocusedUIElement");
            let mut focused_ref: *mut c_void = std::ptr::null_mut();

            if AXUIElementCopyAttributeValue(
                system_wide,
                k_focused_attr.as_concrete_TypeRef(),
                &mut focused_ref,
            ) != K_AX_ERROR_SUCCESS
                || focused_ref.is_null()
            {
                CFRelease(system_wide);
                return Err("Failed to get focused element".to_string());
            }

            let k_window_attr = CFString::new("AXWindow");
            let mut window_ref: *mut c_void = std::ptr::null_mut();

            let target_window = if AXUIElementCopyAttributeValue(
                focused_ref,
                k_window_attr.as_concrete_TypeRef(),
                &mut window_ref,
            ) == K_AX_ERROR_SUCCESS
                && !window_ref.is_null()
            {
                window_ref
            } else {
                focused_ref
            };

            if target_window != focused_ref {
                CFRelease(focused_ref);
            }

            Ok((target_window, system_wide))
        }
    }

    unsafe fn cleanup_element(element: AXUIElementRef) {
        CFRelease(element);
    }

    pub fn resize_focused_window(layout: Layout) {
        if let Ok(info) = get_focused_window_info() {
            let v = info.screen_visible_rect;
            let (new_x, new_y, new_w, new_h) = match layout {
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
                Layout::CenterTwoThirds => {
                    (v.x + v.width / 6.0, v.y, (2.0 * v.width) / 3.0, v.height)
                }
                Layout::RightTwoThirds => {
                    (v.x + v.width / 3.0, v.y, (2.0 * v.width) / 3.0, v.height)
                }
                Layout::Maximize => (v.x, v.y, v.width, v.height),
                Layout::Center => (
                    v.x + (v.width - info.window_rect.width) / 2.0,
                    v.y + (v.height - info.window_rect.height) / 2.0,
                    info.window_rect.width,
                    info.window_rect.height,
                ),
            };

            let _ = set_focused_window_bounds(Rect {
                x: new_x,
                y: new_y,
                width: new_w,
                height: new_h,
            });
        }
    }
}

#[cfg(target_os = "macos")]
pub use macos::Layout;
