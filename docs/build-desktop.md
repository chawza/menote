# Build Desktop Guide

This guide explains how to build the MeNote desktop application for distribution.

## Prerequisites

Before building, ensure you have completed all development setup steps in [dev-guide.md](./dev-guide.md).

## Build Process

### Development Build

For testing the production build locally:

```bash
yarn tauri build --debug
```

This creates a debug version that's faster to build but includes debug symbols.

### Production Build

To create a release build for distribution:

```bash
yarn tauri build
```

This command:
1. Builds the frontend in production mode (`yarn build`)
2. Compiles the Rust backend with optimizations
3. Packages everything into platform-specific installers/binaries

## Build Output

After a successful build, the output will be in:

```
src-tauri/target/release/bundle/
├── dmg/                    # macOS DMG installer (macOS only)
├── appimage/               # Linux AppImage (Linux only)
├── deb/                    # Linux Debian package (Linux only)
├── rpm/                    # Linux RPM package (Linux only)
├── msi/                    # Windows installer (Windows only)
└── nsis/                   # Windows NSIS installer (Windows only)
```

### Platform-Specific Outputs

#### macOS
- `.app` bundle in `src-tauri/target/release/`
- `.dmg` disk image in `src-tauri/target/release/bundle/dmg/`

#### Linux
- AppImage: `src-tauri/target/release/bundle/appimage/`
- Debian package: `src-tauri/target/release/bundle/deb/`
- RPM package: `src-tauri/target/release/bundle/rpm/`

#### Windows
- `.exe` in `src-tauri/target/release/`
- MSI installer: `src-tauri/target/release/bundle/msi/`
- NSIS installer: `src-tauri/target/release/bundle/nsis/`

## Build Configuration

### Tauri Configuration

Build settings are defined in `src-tauri/tauri.conf.json`:

```json
{
  "build": {
    "beforeBuildCommand": "yarn build",
    "frontendDist": "../build"
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

### Customizing Bundle Targets

To build only specific targets, modify `targets` in `tauri.conf.json`:

```json
"bundle": {
  "targets": ["dmg", "appimage"]  // Only build DMG and AppImage
}
```

Or use the CLI:

```bash
# Build only specific targets
yarn tauri build --target dmg
yarn tauri build --target appimage
```

## Icons

Application icons are located in `src-tauri/icons/`:

- `icon.icns` - macOS icon
- `icon.ico` - Windows icon
- `icon.png` - Linux icon (and fallback)
- Various sizes (32x32, 128x128, 128x128@2x, etc.)

To update icons, replace these files with your new icon images.

## Code Signing (Optional)

### macOS Code Signing

For distribution outside the App Store, sign your application:

```bash
# Using a Developer ID certificate
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" src-tauri/target/release/bundle/dmg/*.dmg
```

### Windows Code Signing

Sign Windows executables with a certificate:

```bash
# Using signtool (Windows SDK required)
signtool sign /f certificate.pfx /p password /tr http://timestamp.digicert.com src-tauri/target/release/bundle/msi/*.msi
```

## Notarization (macOS)

For macOS distribution, notarize your application:

```bash
# Using notarytool (macOS 10.15.7+)
xcrun notarytool submit src-tauri/target/release/bundle/dmg/*.dmg --apple-id your@email.com --team-id TEAM_ID --wait
```

## Troubleshooting

### Build Fails on Windows

If the build fails on Windows, ensure you have:
- Visual Studio Build Tools installed
- Windows 10 SDK
- C++ build tools

### Build Fails on macOS

Ensure Xcode Command Line Tools are installed:
```bash
xcode-select --install
```

### Build Fails on Linux

Install required system dependencies:

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
```

**Fedora:**
```bash
sudo dnf install gtk3-devel webkit2gtk3-devel libappindicator-gtk3-devel librsvg2-devel patchelf
```

**Arch Linux:**
```bash
sudo pacman -S gtk3 webkit2gtk libappindicator-gtk3 librsvg patchelf
```

### Large Bundle Size

To reduce bundle size:
1. Enable frontend code splitting
2. Remove unused dependencies
3. Use `yarn build --minify` for aggressive minification
4. Configure Tauri's allowlist to only include needed APIs

## Automated Builds

### GitHub Actions

Create `.github/workflows/build.yml` for automated builds:

```yaml
name: Build
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        platform: [macos-latest, ubuntu-latest, windows-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
      - uses: dtolnay/rust-action@stable
      - run: yarn install
      - run: yarn tauri build
```

## Distribution

After building, distribute your application:

1. **Direct Download**: Upload installers to your website
2. **GitHub Releases**: Create a release and attach build artifacts
3. **App Stores**: Submit to Mac App Store, Microsoft Store, etc.

## Next Steps

- Test your builds thoroughly on each target platform
- Consider setting up automatic updates using Tauri's updater plugin
- Read the [Tauri distribution guide](https://tauri.app/v1/guides/distribution/) for more details
