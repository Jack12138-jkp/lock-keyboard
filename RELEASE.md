# 📦 发布指南 / Release Guide

本文档说明如何发布新版本并自动构建跨平台安装包。

This document explains how to release a new version and automatically build cross-platform installers.

---

## 🚀 发布新版本 / Release New Version

### 1️⃣ 更新版本号 / Update Version Number

编辑以下文件中的版本号：
Edit the version number in the following files:

- `src-tauri/Cargo.toml` - 修改 `version = "x.x.x"`
- `src-tauri/tauri.conf.json` - 修改 `"version": "x.x.x"`
- `package.json` - 修改 `"version": "x.x.x"`

**示例 / Example:**
```toml
# src-tauri/Cargo.toml
[package]
version = "0.2.0"
```

```json
// src-tauri/tauri.conf.json
{
  "version": "0.2.0"
}
```

```json
// package.json
{
  "version": "0.2.0"
}
```

### 2️⃣ 提交更改 / Commit Changes

```bash
git add .
git commit -m "chore: bump version to v0.2.0"
git push origin main
```

### 3️⃣ 创建 Git 标签 / Create Git Tag

```bash
# 创建标签 / Create tag
git tag v0.2.0

# 推送标签到 GitHub / Push tag to GitHub
git push origin v0.2.0
```

### 4️⃣ 自动构建 / Automatic Build

推送标签后，GitHub Actions 会自动：
After pushing the tag, GitHub Actions will automatically:

1. ✅ 在 macOS、Windows、Linux 三个平台上构建应用
2. ✅ 生成各平台的安装包
3. ✅ 创建 GitHub Release
4. ✅ 上传所有安装包到 Release

**构建时间约 15-30 分钟**
**Build time: approximately 15-30 minutes**

### 5️⃣ 查看发布 / View Release

访问 / Visit: https://github.com/Jack12138-jkp/lock-keyboard/releases

---

## 📋 生成的文件 / Generated Files

每次发布会自动生成以下文件：
Each release automatically generates the following files:

### macOS
- `LockKeyboard_x.x.x_universal.dmg` - Universal binary (Intel + Apple Silicon)

### Windows
- `LockKeyboard_x.x.x_x64-setup.exe` - 安装程序 / Installer
- `LockKeyboard_x.x.x_x64_en-US.msi` - MSI 安装包 / MSI Package

### Linux
- `lock-keyboard_x.x.x_amd64.deb` - Debian/Ubuntu
- `lock-keyboard-x.x.x-1.x86_64.rpm` - Fedora/RHEL/CentOS
- `lock-keyboard_x.x.x_amd64.AppImage` - Universal Linux

---

## 🔧 手动触发构建 / Manual Build Trigger

如果需要手动触发构建（不创建 Release）：
To manually trigger a build without creating a Release:

1. 访问 / Visit: https://github.com/Jack12138-jkp/lock-keyboard/actions
2. 选择 "Release" workflow
3. 点击 "Run workflow"
4. 选择分支并运行

---

## ⚠️ 注意事项 / Notes

### 版本号格式 / Version Format
- 必须使用 `v` 前缀，如 `v0.2.0`
- 遵循语义化版本规范：`v主版本.次版本.修订号`
- Must use `v` prefix, e.g., `v0.2.0`
- Follow semantic versioning: `vMAJOR.MINOR.PATCH`

### 标签命名 / Tag Naming
```bash
✅ 正确 / Correct: v0.2.0, v1.0.0, v1.2.3
❌ 错误 / Wrong: 0.2.0, version-0.2.0, release-0.2.0
```

### 删除标签 / Delete Tag
如果需要删除错误的标签：
To delete an incorrect tag:

```bash
# 删除本地标签 / Delete local tag
git tag -d v0.2.0

# 删除远程标签 / Delete remote tag
git push origin :refs/tags/v0.2.0
```

---

## 🐛 故障排除 / Troubleshooting

### 构建失败 / Build Failed
1. 检查 GitHub Actions 日志
2. 确认所有平台的依赖都已正确配置
3. 验证 `Cargo.toml` 和 `tauri.conf.json` 中的版本号一致

### Release 未创建 / Release Not Created
1. 确认标签格式正确（必须以 `v` 开头）
2. 检查 GitHub Actions 权限设置
3. 查看 workflow 运行日志

### 下载链接失效 / Download Links Broken
1. 等待构建完成（约 15-30 分钟）
2. 刷新 Releases 页面
3. 检查是否所有平台都构建成功

---

## 📚 相关链接 / Related Links

- [GitHub Releases](https://github.com/Jack12138-jkp/lock-keyboard/releases)
- [GitHub Actions](https://github.com/Jack12138-jkp/lock-keyboard/actions)
- [Tauri Documentation](https://tauri.app/v1/guides/building/)
- [Semantic Versioning](https://semver.org/)
