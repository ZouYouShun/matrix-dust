/// macOS Accessibility permission detection and prompt helpers.
///
/// Uses the public `AXIsProcessTrusted` / `AXIsProcessTrustedWithOptions`
/// APIs from the ApplicationServices framework — no private API involved.
#[cfg(target_os = "macos")]
pub mod macos {
    use std::ffi::c_void;

    /// Returns `true` when the current process already holds Accessibility permission.
    ///
    /// Calls `AXIsProcessTrusted()` which is side-effect-free (no prompt).
    pub fn is_accessibility_granted() -> bool {
        extern "C" {
            fn AXIsProcessTrusted() -> bool;
        }
        unsafe { AXIsProcessTrusted() }
    }

    /// Opens the *System Settings → Privacy & Security → Accessibility* pane
    /// so the user can toggle the permission themselves, then returns whether
    /// the permission is currently granted (before or after the toggle).
    ///
    /// `AXIsProcessTrustedWithOptions` with `kAXTrustedCheckOptionPrompt = true`
    /// is the canonical way to show the system prompt on macOS ≥ 10.9.
    pub fn prompt_accessibility() -> bool {
        use core_foundation::base::TCFType;
        use core_foundation::boolean::CFBoolean;
        use core_foundation::dictionary::CFDictionary;
        use core_foundation::string::CFString;

        extern "C" {
            // kAXTrustedCheckOptionPrompt is a CFStringRef constant declared in
            // <ApplicationServices/ApplicationServices.h>.
            static kAXTrustedCheckOptionPrompt: *const c_void;

            fn AXIsProcessTrustedWithOptions(options: *const c_void) -> bool;
        }

        unsafe {
            let key = CFString::wrap_under_get_rule(
                kAXTrustedCheckOptionPrompt as *const core_foundation::string::__CFString,
            );
            let value = CFBoolean::true_value();

            // Build {kAXTrustedCheckOptionPrompt: kCFBooleanTrue}
            let dict = CFDictionary::from_CFType_pairs(&[(key.as_CFType(), value.as_CFType())]);

            AXIsProcessTrustedWithOptions(dict.as_concrete_TypeRef() as *const c_void)
        }
    }
}

// ── Tauri commands ──────────────────────────────────────────────────────────

/// Returns whether macOS Accessibility permission is currently granted.
#[tauri::command]
pub fn check_accessibility() -> bool {
    #[cfg(target_os = "macos")]
    {
        macos::is_accessibility_granted()
    }
    #[cfg(not(target_os = "macos"))]
    {
        true // Non-macOS platforms don't need this permission.
    }
}

/// Opens the system Accessibility prompt (if not already granted) and
/// returns the current grant status.
#[tauri::command]
pub fn request_accessibility() -> bool {
    #[cfg(target_os = "macos")]
    {
        macos::prompt_accessibility()
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}
