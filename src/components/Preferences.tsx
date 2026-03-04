import { invoke } from "@tauri-apps/api/core";

interface Props {
  isGranted: boolean;
  onBack: () => void;
}

export default function Preferences({ isGranted, onBack }: Props) {
  const handleOpenSettings = async () => {
    await invoke("open_accessibility_settings");
  };

  return (
    <div className="ref-view preferences-view">
      <div className="header">
        <button className="back-btn" onClick={onBack} title="Back">
          ←
        </button>
        <div className="header-text">
          <h1>Preferences</h1>
          <p>Application settings & permissions</p>
        </div>
      </div>

      <div className="content">
        <div className="section-label">Permissions</div>
        <div className="settings-card">
          <div className="settings-row">
            <div className="settings-info">
              <div className="settings-name">Accessibility Access</div>
              <div className="settings-desc">
                Required to move and resize other application windows.
              </div>
            </div>
            <div
              className={`status-badge ${isGranted ? "--granted" : "--denied"}`}
            >
              {isGranted ? "Granted" : "Not Granted"}
            </div>
          </div>

          <button className="secondary-btn" onClick={handleOpenSettings}>
            {isGranted
              ? "Modify in System Settings"
              : "Enable in System Settings"}
          </button>
        </div>

        <div className="section-label" style={{ marginTop: 24 }}>
          About
        </div>
        <div className="settings-card">
          <div className="settings-row">
            <div className="settings-info">
              <div className="settings-name">Matrix Dust</div>
              <div className="settings-desc">v1.0.0</div>
            </div>
          </div>
          <p className="about-text">
            A lightweight window manager for macOS. All window adjustments
            happen locally on your machine.
          </p>
        </div>
      </div>

      <div className="footer">
        <span className="footer-note">
          Matrix-Dust · Precision Window Management
        </span>
      </div>
    </div>
  );
}
