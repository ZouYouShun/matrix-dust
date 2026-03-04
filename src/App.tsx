import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import AccessibilityPrompt from "./components/AccessibilityPrompt";
import ShortcutReference from "./components/ShortcutReference";

type View = "checking" | "prompt" | "granted";

export default function App() {
  const [view, setView] = useState<View>("checking");

  // Poll accessibility status every 2 seconds while waiting
  const checkAccess = useCallback(async () => {
    try {
      const granted: boolean = await invoke("check_accessibility");
      setView(granted ? "granted" : "prompt");
    } catch {
      setView("prompt");
    }
  }, []);

  useEffect(() => {
    checkAccess();
  }, [checkAccess]);

  // Continuous polling while on the prompt screen so we detect when the user
  // toggles the permission in System Settings without needing to re-open the window.
  useEffect(() => {
    if (view !== "prompt") return;
    const id = setInterval(checkAccess, 2_000);
    return () => clearInterval(id);
  }, [view, checkAccess]);

  const handleClose = useCallback(async () => {
    try {
      await getCurrentWindow().hide();
    } catch {
      /* ignore */
    }
  }, []);

  if (view === "checking") {
    return (
      <div className="checking-view">
        <div className="spinner" />
        <p>Checking permissions…</p>
      </div>
    );
  }

  if (view === "prompt") {
    return (
      <AccessibilityPrompt
        onGranted={() => setView("granted")}
        onClose={handleClose}
      />
    );
  }

  return <ShortcutReference onClose={handleClose} />;
}
