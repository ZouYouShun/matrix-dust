---
description: Create a new release for the Matrix Dust application
---

# New Release Workflow

This workflow automates the process of bumping the version, building the application, and creating a GitHub release.

1.  **Determine New Version**
    Decide on the new version (e.g., `0.1.4`) based on SemVer.

2.  **Bump Versions**
    Update the version in:
    - `package.json`
    - `src-tauri/tauri.conf.json`
    - `src-tauri/Cargo.toml`

3.  **Commit and Tag**

    ```bash
    git add .
    git commit -m "chore: bump version to v<VERSION>"
    git tag v<VERSION>
    ```

4.  **Build the Application**
    // turbo

    ```bash
    npm run build:app
    ```

5.  **Clean and Package Assets**

    ```bash
    zip -r "Matrix_Dust_<VERSION>_aarch64_app.zip" "src-tauri/target/release/bundle/macos/Matrix Dust.app"
    ```

6.  **Push to Origin**
    // turbo

    ```bash
    git push origin main && git push origin v<VERSION>
    ```

7.  **Create GitHub Release**
    // turbo

    ```bash
    gh release create v<VERSION> --title "v<VERSION>" --notes "Release v<VERSION>" \
      "src-tauri/target/release/bundle/dmg/Matrix Dust_<VERSION>_aarch64.dmg" \
      "Matrix_Dust_<VERSION>_aarch64_app.zip"
    ```

8.  **Cleanup Temporary Files**
    ```bash
    rm "Matrix_Dust_<VERSION>_aarch64_app.zip"
    ```
