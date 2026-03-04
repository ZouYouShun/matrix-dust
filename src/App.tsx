import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import AccessibilityPrompt from "./components/AccessibilityPrompt";
import ShortcutReference from "./components/ShortcutReference";
import Preferences from "./components/Preferences";

type View = "checking" | "prompt" | "granted" | "preferences";

export default function App() {
  const [view, setView] = useState<View>("checking");
  const [isGranted, setIsGranted] = useState(false);

  // Poll accessibility status every 2 seconds
  const checkAccess = useCallback(async () => {
    try {
      const granted: boolean = await invoke("check_accessibility");
      setIsGranted(granted);

      // Only auto-switch to "granted" if we are currently checking or on the prompt screen.
      // If we are on the preferences screen, we stay there.
      if (granted && (view === "checking" || view === "prompt")) {
        setView("granted");
      } else if (!granted && view !== "preferences") {
        setView("prompt");
      }
    } catch {
      if (view !== "preferences") setView("prompt");
    }
  }, [view]);

  useEffect(() => {
    checkAccess();
  }, [checkAccess]);

  // Continuous polling
  useEffect(() => {
    const id = setInterval(checkAccess, 2_000);
    return () => clearInterval(id);
  }, [checkAccess]);

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

  if (view === "preferences") {
    return (
      <Preferences isGranted={isGranted} onBack={() => setView("granted")} />
    );
  }

  return (
    <ShortcutReference
      isGranted={isGranted}
      onOpenPreferences={() => setView("preferences")}
      onClose={handleClose}
    />
  );
}
