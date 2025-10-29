# 🔒 LockKeyboard

<div align="center">

A lightweight keyboard lock tool for macOS, Windows, and Linux built with Tauri and Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/tauri-2.x-blue.svg)](https://tauri.app/)
[![GitHub release](https://img.shields.io/github/v/release/Jack12138-jkp/lock-keyboard)](https://github.com/Jack12138-jkp/lock-keyboard/releases)
[![GitHub downloads](https://img.shields.io/github/downloads/Jack12138-jkp/lock-keyboard/total)](https://github.com/Jack12138-jkp/lock-keyboard/releases)

[中文文档](README.md) | English

</div>

## ✨ Features

- 🔒 **One-Click Lock**: Quickly lock all keyboard input via tray menu
- 🔓 **Quick Unlock**: Click to restore keyboard functionality
- 🖱️ **Tray Control**: Clean system tray interface without occupying Dock/Taskbar space
- 🔄 **Auto Recovery**: Automatic thread restart mechanism ensures stable operation
- ⚡ **Lightweight**: Pure Rust implementation, memory usage < 10MB
- 🎯 **Background Only**: Runs in background, focused on core functionality
- 🌍 **Cross-Platform**: Supports macOS, Windows, and Linux

## 📸 Screenshots

> System tray menu interface

## 🚀 Quick Start

### System Requirements

#### macOS
- macOS 10.15 (Catalina) or higher
- Requires "Accessibility" permission (automatic guidance on first run)

#### Windows
- Windows 10 or higher
- Administrator privileges may be required

#### Linux
- Most modern Linux distributions
- X11 or Wayland display server

### Installation

#### Method 1: Download Pre-built Binary (Recommended)

**[📥 Click here to download the latest version](https://github.com/Jack12138-jkp/lock-keyboard/releases/latest)**

Download the latest version for your platform from [Releases](https://github.com/Jack12138-jkp/lock-keyboard/releases):

| Platform | File Type | Description |
|----------|-----------|-------------|
| **macOS** | `.dmg` | Supports Intel & Apple Silicon (M1/M2/M3) |
| **Windows** | `.msi` or `.exe` | Windows 10 and above |
| **Linux** | `.deb` | Debian/Ubuntu systems |
| **Linux** | `.rpm` | Fedora/RHEL/CentOS systems |
| **Linux** | `.AppImage` | Universal format for all distributions |

#### Method 2: Build from Source

```bash
# Clone repository
git clone https://github.com/yourusername/lock-keyboard.git
cd lock-keyboard

# Install dependencies
npm install

# Build application
cargo tauri build
```

Built application will be located in `src-tauri/target/release/bundle/`

## 📖 Usage

1. **Launch App**: Double-click to run, app icon will appear in system tray
2. **Lock Keyboard**: Click tray icon → Select "🔒 Lock Keyboard"
3. **Unlock Keyboard**: Click tray icon → Select "🔓 Unlock Keyboard"
4. **Exit App**: Click tray icon → Select "❌ Quit"

### Platform-Specific Setup

#### macOS - Grant Permissions

On first run, the app will automatically open System Preferences:

1. Go to **System Preferences** → **Security & Privacy** → **Privacy** → **Accessibility**
2. Click the lock icon at bottom-left to unlock
3. Check **LockKeyboard**
4. Restart the application

#### Windows - Administrator Privileges

If keyboard locking doesn't work:
1. Right-click the application
2. Select "Run as administrator"
3. Confirm the UAC prompt

#### Linux - Input Device Access

Depending on your distribution, you may need to:
1. Add your user to the `input` group:
   ```bash
   sudo usermod -a -G input $USER
   ```
2. Log out and log back in for changes to take effect

## 🛠️ Tech Stack

- **Framework**: [Tauri 2.x](https://tauri.app/) - Lightweight desktop application framework
- **Language**: [Rust](https://www.rust-lang.org/) - High-performance systems programming language
- **Keyboard Monitoring**: [rdev](https://github.com/Narsil/rdev) - Cross-platform input event library

## 🏗️ Project Structure

```
lock-keyboard/
├── src-tauri/
│   ├── src/
│   │   └── lib.rs          # Core logic
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri configuration
│   └── icons/              # Application icons
├── LICENSE                 # MIT License
├── README.md              # Chinese documentation
├── README.en.md           # English documentation
└── package.json           # npm configuration
```

## 🤝 Contributing

Issues and Pull Requests are welcome!

1. Fork this repository
2. Create feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to branch (`git push origin feature/AmazingFeature`)
5. Open Pull Request

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details

## ⚠️ Disclaimer

This tool is for learning and personal use only. When using this tool to lock your keyboard, ensure you can unlock it via mouse to avoid inconvenience.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - Excellent desktop application framework
- [rdev](https://github.com/Narsil/rdev) - Powerful input event library

---

<div align="center">
Made with ❤️ by Jiang
</div>
