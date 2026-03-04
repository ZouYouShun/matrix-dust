import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import AccessibilityPrompt from "./components/AccessibilityPrompt";
import ShortcutReference from "./components/ShortcutReference";
import Preferences from "./components/Preferences";
import WelcomeTutorial from "./components/WelcomeTutorial";

type View = "checking" | "prompt" | "granted" | "preferences" | "tutorial";

export default function App() {
  const [windowLabel, setWindowLabel] = useState<string | null>(null);
  const [view, setView] = useState<View>("checking");
  const [isGranted, setIsGranted] = useState(false);

  useEffect(() => {
    setWindowLabel(getCurrentWindow().label);
  }, []);

  const checkAccess = useCallback(async () => {
    if (!windowLabel) return;

    try {
      const granted: boolean = await invoke("check_accessibility");
      setIsGranted(granted);

      // We only want to auto-navigate if the user is NOT in preferences.
      if (view === "preferences") return;

      if (windowLabel === "main") {
        if (granted) {
          const hasTutorial = localStorage.getItem("hasCompletedTutorial");
          if (hasTutorial !== "true") {
            // Success! Open new tutorial window and hide this one
            await invoke("open_tutorial_window");
            await getCurrentWindow().hide();
            // Important: update view to granted so if main window is
            // reopened later, it shows the shortcuts, not the prompt.
            setView("granted");
          } else {
            // Tutorial already done, show main shortcuts if not already there
            if (view !== "granted") setView("granted");
          }
        } else {
          // No access, show prompt
          if (view !== "prompt") setView("prompt");
        }
      } else if (windowLabel === "tutorial") {
        // In the tutorial window, always show tutorial view
        if (view !== "tutorial") setView("tutorial");
      }
    } catch (err) {
      console.error("Access check failed:", err);
    }
  }, [view, windowLabel]);

  useEffect(() => {
    if (windowLabel) {
      checkAccess();
    }
  }, [checkAccess, windowLabel]);

  // Polling only if granted status might change (e.g. in prompt or preferences)
  useEffect(() => {
    const id = setInterval(checkAccess, 2000);
    return () => clearInterval(id);
  }, [checkAccess]);

  const handleClose = useCallback(async () => {
    try {
      await getCurrentWindow().hide();
    } catch {
      /* ignore */
    }
  }, []);

  const handleTutorialComplete = useCallback(async () => {
    localStorage.setItem("hasCompletedTutorial", "true");
    try {
      await getCurrentWindow().close();
    } catch {
      /* ignore */
    }
  }, []);

  // Show nothing until we know which window we are in to avoid flashing
  if (!windowLabel) return null;

  if (view === "checking") {
    return (
      <div className="checking-view">
        <div className="spinner" />
        <p>Initializing…</p>
      </div>
    );
  }

  if (view === "tutorial") {
    return <WelcomeTutorial onComplete={handleTutorialComplete} />;
  }

  if (view === "prompt") {
    return (
      <AccessibilityPrompt
        onGranted={() => {
          // checkAccess will handle the transition
        }}
        onClose={handleClose}
      />
    );
  }

  if (view === "preferences") {
    return (
      <Preferences isGranted={isGranted} onBack={() => setView("granted")} />
    );
  }

  // Final fallback: show ShortcutReference (the "granted" view)
  return (
    <ShortcutReference
      isGranted={isGranted}
      onOpenPreferences={() => setView("preferences")}
      onClose={handleClose}
    />
  );
}
