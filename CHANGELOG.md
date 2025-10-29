# 更新日志 / Changelog

所有重要的项目更改都将记录在此文件中。
All notable changes to this project will be documented in this file.

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)。
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased]

### 计划中 / Planned
- 添加全局快捷键支持
- 支持自定义托盘图标
- 添加锁定时的视觉反馈

---

## [0.1.0] - 2025-10-29

### 新增 / Added
- ✨ 初始版本发布
- 🔒 一键锁定/解锁键盘功能
- 🖱️ 系统托盘控制界面
- 🔄 自动恢复机制
- 🌍 跨平台支持（macOS、Windows、Linux）
- 📦 自动构建和发布流程
- 📚 中英文文档

### 功能特性 / Features
- 通过系统托盘菜单控制键盘锁定状态
- 支持 macOS 辅助功能权限自动检测
- 支持 Windows 管理员权限检测
- 支持 Linux input 组权限检测
- 轻量级设计，内存占用 < 10MB
- 纯后台运行，不占用 Dock/任务栏空间

### 技术栈 / Tech Stack
- Tauri 2.x
- Rust
- rdev (跨平台输入事件库)

---

## 版本说明 / Version Notes

### 版本号格式 / Version Format
- **主版本号 (Major)**: 重大功能更新或不兼容的 API 变更
- **次版本号 (Minor)**: 向后兼容的功能新增
- **修订号 (Patch)**: 向后兼容的问题修复

### 更新类型 / Change Types
- `新增 / Added`: 新功能
- `变更 / Changed`: 现有功能的变更
- `弃用 / Deprecated`: 即将移除的功能
- `移除 / Removed`: 已移除的功能
- `修复 / Fixed`: 问题修复
- `安全 / Security`: 安全相关的修复

---

[Unreleased]: https://github.com/Jack12138-jkp/lock-keyboard/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Jack12138-jkp/lock-keyboard/releases/tag/v0.1.0
