import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  onGranted: () => void;
  onClose: () => void;
}

type Step = "idle" | "waiting" | "denied";

export default function AccessibilityPrompt({ onGranted, onClose }: Props) {
  const [step, setStep] = useState<Step>("idle");

  const handleEnable = async () => {
    setStep("waiting");
    try {
      // This opens System Settings → Accessibility and returns the current status.
      const granted: boolean = await invoke("request_accessibility");
      if (granted) {
        onGranted();
      } else {
        // Permission dialog was shown but user hasn't toggled yet.
        // The parent App component polls every 2 s and will call onGranted automatically.
        setStep("idle");
      }
    } catch {
      setStep("denied");
    }
  };

  return (
    <div className="prompt-view">
      {/* Close button */}
      <button className="close-btn" onClick={onClose} title="Hide to tray">
        ✕
      </button>

      {/* Icon area */}
      <div className="prompt-icon-ring">
        <span className="prompt-icon">⊞</span>
      </div>

      <h1 className="prompt-title">Matrix Dust</h1>
      <p className="prompt-subtitle">Precision Window Management for macOS</p>

      {/* Permission card */}
      <div className="permission-card">
        <div className="permission-card-header">
          <span className="lock-icon">🔐</span>
          <span>Accessibility Permission Required</span>
        </div>
        <p className="permission-desc">
          Matrix Dust needs <strong>Accessibility access</strong> to move and
          resize other application windows using global keyboard shortcuts.
        </p>

        <div className="permission-steps">
          <div className="permission-step">
            <span className="step-num">1</span>
            <span>
              Click <strong>Enable Accessibility</strong> below
            </span>
          </div>
          <div className="permission-step">
            <span className="step-num">2</span>
            <span>
              System Settings will open — find <strong>Matrix Dust</strong> in
              the list
            </span>
          </div>
          <div className="permission-step">
            <span className="step-num">3</span>
            <span>
              Toggle it <strong>ON</strong>, then come back here
            </span>
          </div>
        </div>

        {step === "waiting" ? (
          <button className="enable-btn --waiting" disabled>
            <span className="spinner-sm" />
            Opening System Settings…
          </button>
        ) : (
          <button className="enable-btn" onClick={handleEnable}>
            Enable Accessibility
          </button>
        )}

        {step === "denied" && (
          <p className="denied-msg">
            Something went wrong. Please open{" "}
            <strong>
              System Settings → Privacy &amp; Security → Accessibility
            </strong>{" "}
            manually and add Matrix Dust.
          </p>
        )}
      </div>

      <p className="prompt-footer">
        Permission is only used for window resizing — no data is collected.
      </p>
    </div>
  );
}
