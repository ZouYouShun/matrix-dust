import { getCurrentWindow } from "@tauri-apps/api/window";

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
    name: "Left⅔",
    key: "E",
    visual: { left: "0%", top: "0%", width: "67%", height: "100%" },
  },
  {
    name: "Center⅔",
    key: "R",
    visual: { left: "17%", top: "0%", width: "67%", height: "100%" },
  },
  {
    name: "Right⅔",
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

function renderLayoutCard(entry: LayoutEntry): string {
  return `
    <div class="layout-preview" title="${entry.name}">
      <div class="layout-visual">
        <div class="layout-fill" style="
          left:${entry.visual.left};
          top:${entry.visual.top};
          width:${entry.visual.width};
          height:${entry.visual.height};
        "></div>
      </div>
      <div class="layout-name">${entry.name}</div>
      <div class="layout-key">${MODIFIER} ${entry.key}</div>
    </div>
  `;
}

const app = document.getElementById("app")!;

app.innerHTML = `
  <div class="header">
    <div class="header-icon">⊞</div>
    <div class="header-text">
      <h1>Window Tuner</h1>
      <p>Global keyboard shortcuts</p>
    </div>
    <div class="status-dot">Active</div>
    <button id="btn-close" class="close-btn" title="Hide to tray">✕</button>
  </div>

  <div class="content">
    <div class="section-label">Layouts</div>
    <div class="layout-grid">
      ${layouts.map(renderLayoutCard).join("")}
    </div>

    <div class="section-label" style="margin-top:4px">Shortcut Guide</div>
    <div class="shortcut-list">
      <div class="shortcut-row">
        <span class="shortcut-name">Halves</span>
        <div class="keys">
          <span class="key">⌃</span><span class="key">⌥</span>
          <span class="key-sep">+</span>
          <span class="key">← → ↑ ↓</span>
        </div>
      </div>
      <div class="shortcut-row">
        <span class="shortcut-name">Quarters</span>
        <div class="keys">
          <span class="key">⌃</span><span class="key">⌥</span>
          <span class="key-sep">+</span>
          <span class="key">U</span><span class="key">I</span><span class="key">J</span><span class="key">K</span>
        </div>
      </div>
      <div class="shortcut-row">
        <span class="shortcut-name">Thirds</span>
        <div class="keys">
          <span class="key">⌃</span><span class="key">⌥</span>
          <span class="key-sep">+</span>
          <span class="key">D</span><span class="key">F</span><span class="key">G</span>
        </div>
      </div>
      <div class="shortcut-row">
        <span class="shortcut-name">Two-Thirds</span>
        <div class="keys">
          <span class="key">⌃</span><span class="key">⌥</span>
          <span class="key-sep">+</span>
          <span class="key">E</span><span class="key">R</span><span class="key">T</span>
        </div>
      </div>
      <div class="shortcut-row">
        <span class="shortcut-name">Maximize</span>
        <div class="keys">
          <span class="key">⌃</span><span class="key">⌥</span>
          <span class="key-sep">+</span>
          <span class="key">↵ Enter</span>
        </div>
      </div>
      <div class="shortcut-row">
        <span class="shortcut-name">Center</span>
        <div class="keys">
          <span class="key">⌃</span><span class="key">⌥</span>
          <span class="key-sep">+</span>
          <span class="key">C</span>
        </div>
      </div>
    </div>
  </div>

  <div class="footer">
    <span class="footer-note">macOS only · Accessibility permission required</span>
    <span class="footer-badge">Matrix-Dust</span>
  </div>
`;

// Close button -> hide window back to tray
const closeBtn = document.getElementById("btn-close");
if (closeBtn) {
  closeBtn.addEventListener("click", async () => {
    console.log("Close button clicked, hiding window...");
    try {
      const win = getCurrentWindow();
      await win.hide();
    } catch (err) {
      console.error("Failed to hide window:", err);
    }
  });
} else {
  console.error("Could not find btn-close element");
}
