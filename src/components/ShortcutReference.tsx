interface LayoutEntry {
  name: string;
  key: string;
  visual: { left: string; top: string; width: string; height: string };
}

const MODIFIER = "⌃ ⌥"; // Ctrl + Option

const layouts: LayoutEntry[] = [
  {
    name: "Left Half",
    key: "←",
    visual: { left: "0%", top: "0%", width: "50%", height: "100%" },
  },
  {
    name: "Right Half",
    key: "→",
    visual: { left: "50%", top: "0%", width: "50%", height: "100%" },
  },
  {
    name: "Top Half",
    key: "↑",
    visual: { left: "0%", top: "0%", width: "100%", height: "50%" },
  },
  {
    name: "Bottom Half",
    key: "↓",
    visual: { left: "0%", top: "50%", width: "100%", height: "50%" },
  },
  {
    name: "Top Left",
    key: "U",
    visual: { left: "0%", top: "0%", width: "50%", height: "50%" },
  },
  {
    name: "Top Right",
    key: "I",
    visual: { left: "50%", top: "0%", width: "50%", height: "50%" },
  },
  {
    name: "Bottom Left",
    key: "J",
    visual: { left: "0%", top: "50%", width: "50%", height: "50%" },
  },
  {
    name: "Bottom Right",
    key: "K",
    visual: { left: "50%", top: "50%", width: "50%", height: "50%" },
  },
  {
    name: "Left Third",
    key: "D",
    visual: { left: "0%", top: "0%", width: "33%", height: "100%" },
  },
  {
    name: "Center Third",
    key: "F",
    visual: { left: "33%", top: "0%", width: "34%", height: "100%" },
  },
  {
    name: "Right Third",
    key: "G",
    visual: { left: "67%", top: "0%", width: "33%", height: "100%" },
  },
  {
    name: "Left ⅔",
    key: "E",
    visual: { left: "0%", top: "0%", width: "67%", height: "100%" },
  },
  {
    name: "Center ⅔",
    key: "R",
    visual: { left: "17%", top: "0%", width: "67%", height: "100%" },
  },
  {
    name: "Right ⅔",
    key: "T",
    visual: { left: "33%", top: "0%", width: "67%", height: "100%" },
  },
  {
    name: "Maximize",
    key: "↵",
    visual: { left: "0%", top: "0%", width: "100%", height: "100%" },
  },
  {
    name: "Center",
    key: "C",
    visual: { left: "20%", top: "15%", width: "60%", height: "70%" },
  },
];

const shortcutGroups = [
  { label: "Halves", keys: ["←", "→", "↑", "↓"] },
  { label: "Quarters", keys: ["U", "I", "J", "K"] },
  { label: "Thirds", keys: ["D", "F", "G"] },
  { label: "Two-Thirds", keys: ["E", "R", "T"] },
  { label: "Maximize", keys: ["↵"] },
  { label: "Center", keys: ["C"] },
];

interface Props {
  isGranted: boolean;
  onOpenPreferences: () => void;
  onClose: () => void;
}

export default function ShortcutReference({
  isGranted,
  onOpenPreferences,
  onClose,
}: Props) {
  return (
    <div className="ref-view">
      <div className="header">
        <div className="header-icon">⊞</div>
        <div className="header-text">
          <h1>Matrix Dust</h1>
          <p>Global keyboard shortcuts</p>
        </div>
        <div className="status-dot">Active</div>
        <button
          className="settings-toggle-btn"
          title="Preferences"
          onClick={onOpenPreferences}
        >
          ⚙
        </button>
        <button
          id="btn-close"
          className="close-btn"
          title="Hide to tray"
          onClick={onClose}
        >
          ✕
        </button>
      </div>

      <div className="content">
        <div className="section-label">Layouts</div>
        <div className="layout-grid">
          {layouts.map((entry) => (
            <div key={entry.name} className="layout-preview" title={entry.name}>
              <div className="layout-visual">
                <div
                  className="layout-fill"
                  style={{
                    left: entry.visual.left,
                    top: entry.visual.top,
                    width: entry.visual.width,
                    height: entry.visual.height,
                  }}
                />
              </div>
              <div className="layout-name">{entry.name}</div>
              <div className="layout-key">
                {MODIFIER} {entry.key}
              </div>
            </div>
          ))}
        </div>

        <div className="section-label" style={{ marginTop: 4 }}>
          Shortcut Guide
        </div>
        <div className="shortcut-list">
          {shortcutGroups.map((group) => (
            <div key={group.label} className="shortcut-row">
              <span className="shortcut-name">{group.label}</span>
              <div className="keys">
                <span className="key">⌃</span>
                <span className="key">⌥</span>
                <span className="key-sep">+</span>
                {group.keys.map((k) => (
                  <span key={k} className="key">
                    {k}
                  </span>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>

      <div className="footer">
        <span className="footer-note">
          macOS only · Accessibility {isGranted ? "granted ✓" : "not granted ⚠"}
        </span>
        <span className="footer-badge">Matrix-Dust</span>
      </div>
    </div>
  );
}
