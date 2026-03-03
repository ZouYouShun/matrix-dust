# window-tuner

A lightweight macOS window layout manager extracted from [screen-craft](../../).  
Runs as a background Tauri app and listens for global shortcuts using the Accessibility API.

## Shortcuts

All shortcuts use **Ctrl + Option** (`⌃ ⌥`) as the modifier.

| Layout            | Shortcut |
| ----------------- | -------- |
| Left Half         | `⌃ ⌥ ←`  |
| Right Half        | `⌃ ⌥ →`  |
| Top Half          | `⌃ ⌥ ↑`  |
| Bottom Half       | `⌃ ⌥ ↓`  |
| Top Left          | `⌃ ⌥ U`  |
| Top Right         | `⌃ ⌥ I`  |
| Bottom Left       | `⌃ ⌥ J`  |
| Bottom Right      | `⌃ ⌥ K`  |
| Left Third        | `⌃ ⌥ D`  |
| Center Third      | `⌃ ⌥ F`  |
| Right Third       | `⌃ ⌥ G`  |
| Left Two-Thirds   | `⌃ ⌥ E`  |
| Center Two-Thirds | `⌃ ⌥ R`  |
| Right Two-Thirds  | `⌃ ⌥ T`  |
| Maximize          | `⌃ ⌥ ↵`  |
| Center            | `⌃ ⌥ C`  |

## Requirements

- macOS only
- Grant **Accessibility** permission when prompted (System Settings → Privacy → Accessibility)

## Development

```bash
bun install
bun run start   # tauri dev
```

## Build

```bash
bun run build:app
# or for Apple Silicon
bun run build:mac-silicon
```
