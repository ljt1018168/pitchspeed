# 音速调 / PitchSpeed

一款跨平台桌面音频处理应用，支持独立调整音高和速度，不影响彼此。

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## ✨ 功能特性

### 🎵 音高调整
- 调整范围：-24 到 +24 半音
- 精度：0.5 半音步进
- 独立调整，不影响音频时长

### ⚡ 速度调整
- 调整范围：0.1x 到 5.0x
- 精度：0.01x 步进
- 独立调整，不影响音高

### 🎯 音高检测
- 自动检测音频主音调
- 显示检测置信度
- 实时显示调整后的目标音调

### 📁 格式支持
**输入格式**：MP3, WAV, M4A, OGG, FLAC, AAC

**输出格式**：
- WAV（24-bit / 48kHz）
- MP3（320kbps）

### 🔊 实时预览
- 导出前可预览调整效果
- 支持播放/停止控制

## 📥 下载安装

### macOS
1. 下载 `.dmg` 文件
2. 打开后拖拽到 Applications 文件夹
3. 首次运行可能需要在"系统偏好设置 → 安全性与隐私"中允许

### Windows
1. 下载 `.exe` 安装包或 `.msi` 文件
2. 双击运行安装
3. 按照安装向导完成安装

## 🚀 使用方法

1. **上传音频** - 点击上传区域选择音频文件
2. **查看音调** - 自动检测并显示原始音调
3. **调整参数** - 使用滑块调整音高和速度
4. **预览效果** - 点击预览按钮试听效果
5. **导出文件** - 选择格式后点击转换并导出

## 📁 导出文件命名

导出文件自动命名为：`原文件名_目标音调_速度.格式`

示例：`song_G_1.25x.mp3`
- `song` - 原文件名
- `G` - 调整后的目标音调
- `1.25x` - 速度倍率

## 🛠️ 技术栈

- **前端**: SvelteKit 5 + TypeScript
- **后端**: Tauri 2 + Rust
- **音频处理**:
  - 解码：Symphonia
  - 编码：Hound (WAV) + LAME (MP3)
  - 音高/速度：signalsmith-stretch
  - 音高检测：pitch-detection (McLeod 算法)

## 📦 本地开发

```bash
# 克隆仓库
git clone https://github.com/ljt1018168/pitchspeed.git
cd pitchspeed

# 安装依赖
npm install

# 开发模式运行
npm run tauri dev

# 构建生产版本
npm run tauri build
```

### 系统要求

- Node.js 18+
- Rust 1.70+
- macOS 14+ / Windows 10+

## 📄 许可证

[MIT License](LICENSE)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

**音速调** - 让音频处理更简单