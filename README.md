# 🔒 LockKeyboard

<div align="center">

一个基于 Tauri 和 Rust 开发的轻量级 macOS 键盘锁定工具

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/tauri-2.x-blue.svg)](https://tauri.app/)

</div>

## ✨ 功能特性

- 🔒 **一键锁定**：通过托盘菜单快速锁定所有键盘输入
- 🔓 **快速解锁**：点击即可恢复键盘功能
- 🖱️ **托盘控制**：简洁的系统托盘界面，不占用 Dock 空间
- 🔄 **自动恢复**：监听线程自动重启机制，确保稳定运行
- ⚡ **轻量高效**：纯 Rust 实现，内存占用 < 10MB
- 🎯 **无界面设计**：纯后台运行，专注核心功能

## 📸 截图

> 系统托盘菜单界面

## 🚀 快速开始

### 系统要求

- macOS 10.15 (Catalina) 或更高版本
- 需要授予"辅助功能"权限（首次运行会自动引导）

### 安装

#### 方式 1：下载预编译版本（推荐）

从 [Releases](https://github.com/你的用户名/lock-keyboard/releases) 页面下载最新版本的 `.dmg` 文件。

#### 方式 2：从源码构建

```bash
# 克隆仓库
git clone https://github.com/你的用户名/lock-keyboard.git
cd lock-keyboard

# 安装依赖
npm install

# 构建应用
cargo tauri build
```

构建完成后，应用位于 `src-tauri/target/release/bundle/`

## 📖 使用说明

1. **启动应用**：双击运行，应用会在系统托盘显示图标
2. **锁定键盘**：点击托盘图标 → 选择 "🔒 锁定键盘"
3. **解锁键盘**：点击托盘图标 → 选择 "🔓 解锁键盘"
4. **退出应用**：点击托盘图标 → 选择 "❌ 退出"

### 授予权限

首次运行时，应用会自动打开系统偏好设置：

1. 前往 **系统偏好设置** → **安全性与隐私** → **隐私** → **辅助功能**
2. 点击左下角的锁图标解锁
3. 勾选 **LockKeyboard**
4. 重启应用

## 🛠️ 技术栈

- **框架**：[Tauri 2.x](https://tauri.app/) - 轻量级桌面应用框架
- **语言**：[Rust](https://www.rust-lang.org/) - 高性能系统编程语言
- **键盘监听**：[rdev](https://github.com/Narsil/rdev) - 跨平台输入事件库

## 🏗️ 项目结构

```
lock-keyboard/
├── src-tauri/
│   ├── src/
│   │   └── lib.rs          # 核心逻辑（169 行）
│   ├── Cargo.toml          # Rust 依赖
│   ├── tauri.conf.json     # Tauri 配置
│   └── icons/              # 应用图标
├── LICENSE                 # MIT 许可证
├── README.md              # 项目文档
└── package.json           # npm 配置
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## ⚠️ 免责声明

本工具仅供学习和个人使用。使用本工具锁定键盘时，请确保您能够通过鼠标操作解锁，避免造成不便。

## 🙏 致谢

- [Tauri](https://tauri.app/) - 优秀的桌面应用框架
- [rdev](https://github.com/Narsil/rdev) - 强大的输入事件库

---

<div align="center">
Made with ❤️ by Jiang
</div>
