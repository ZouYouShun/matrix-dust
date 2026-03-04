---
name: Tauri Release Management
description: Automated workflow for bumping versions, building Tauri apps, and creating GitHub releases with artifacts.
---

# Tauri Release Management Skill

This skill provides a standardized workflow for releasing new versions of the Matrix Dust application.

## Prerequisites

- GitHub CLI (`gh`) installed and authenticated.
- Node.js environment with `npm` or `bun`.
- Rust/Tauri development environment.
- `zip` utility for macOS.

## Release Workflow

### 1. Version Bumping

Synchronize the version across the following files:

- `package.json`: `"version": "x.y.z"`
- `src-tauri/tauri.conf.json`: `"version": "x.y.z"`
- `src-tauri/Cargo.toml`: `version = "x.y.z"`

### 2. Commit and Tag

```bash
git add .
git commit -m "chore: bump version to vx.y.z"
git tag vx.y.z
```

### 3. Build Production Assets

Run the build command to generate DMG and application bundles:

```bash
npm run build:app
```

_Note: This generates assets in `src-tauri/target/release/bundle/`._

### 4. Package Artifacts

Compress the `.app` bundle for easier distribution:

```bash
zip -r "Matrix_Dust_x.y.z_aarch64_app.zip" "src-tauri/target/release/bundle/macos/Matrix Dust.app"
```

### 5. Push and Release

Push the code and tag, then create the GitHub release:

```bash
git push origin main
git push origin vx.y.z
gh release create vx.y.z --title "vx.y.z" --notes "Release vx.y.z" \
  "src-tauri/target/release/bundle/dmg/Matrix Dust_x.y.z_aarch64.dmg" \
  "Matrix_Dust_x.y.z_aarch64_app.zip"
```

### 6. Cleanup

```bash
rm "Matrix_Dust_x.y.z_aarch64_app.zip"
```

## Tips

- Verify the build logs for any warnings or errors before pushing the tag.
- Ensure the version number follows Semantic Versioning (SemVer) guidelines.
- If using `git-cliff` or similar, update the `CHANGELOG.md` before Step 2.
