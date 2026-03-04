import { useEffect, useState, useCallback, useRef } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

interface Props {
  onComplete: () => void;
}

export default function WelcomeTutorial({ onComplete }: Props) {
  const [isMaximized, setIsMaximized] = useState(false);
  const [countdown, setCountdown] = useState<number | null>(null);
  const initialSize = useRef<{ width: number; height: number } | null>(null);

  useEffect(() => {
    // Capture the initial size when tutorial mounts
    getCurrentWindow()
      .innerSize()
      .then((size) => {
        initialSize.current = { width: size.width, height: size.height };
      });
  }, []);

  const checkWindowState = useCallback(async () => {
    if (!initialSize.current || isMaximized) return;

    try {
      const window = getCurrentWindow();
      const currentSize = await window.innerSize();

      // Detection: If the size has changed more than a tiny threshold,
      // it means our window adjustment logic (Shortcut) has taken effect.
      const diffW = Math.abs(currentSize.width - initialSize.current.width);
      const diffH = Math.abs(currentSize.height - initialSize.current.height);

      if (diffW > 10 || diffH > 10) {
        setIsMaximized(true);
        setCountdown(5);
      }
    } catch (err) {
      console.error("Failed to check window state", err);
    }
  }, [isMaximized]);

  useEffect(() => {
    // Poll for maximization status
    const interval = setInterval(checkWindowState, 500);

    return () => {
      clearInterval(interval);
    };
  }, [checkWindowState]);

  useEffect(() => {
    if (countdown === null) return;
    if (countdown <= 0) {
      onComplete();
      return;
    }

    const timer = setTimeout(() => {
      setCountdown((prev) => (prev !== null ? prev - 1 : null));
    }, 1000);

    return () => clearTimeout(timer);
  }, [countdown, onComplete]);

  return (
    <div className="prompt-view tutorial-view">
      <div className="prompt-icon-ring success-ring">
        <span className="prompt-icon">✨</span>
      </div>

      <h1 className="prompt-title">Welcome!</h1>
      <p className="prompt-subtitle">Accessibility Permission Granted</p>

      <div className="permission-card success-card">
        {!isMaximized ? (
          <>
            <div className="tutorial-step">
              <span className="step-num">Final Step</span>
              <p className="tutorial-text">
                Try pressing <strong>⌃ ⌥ ↵</strong> (Ctrl + Option + Enter) to
                maximize this window.
              </p>
            </div>
            <div className="shortcut-hint">
              <span className="key">⌃</span>
              <span className="key">⌥</span>
              <span className="key">↵</span>
            </div>
          </>
        ) : (
          <div className="success-message">
            <h2>Perfect!</h2>
            <p>Matrix Dust is now fully operational.</p>
            <div className="closing-hint">
              Closing in <strong>{countdown}</strong> seconds...
            </div>
          </div>
        )}
      </div>

      <p className="prompt-footer">
        Matrix Dust will continue to run in your system tray.
      </p>
    </div>
  );
}
