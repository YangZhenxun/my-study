# overtheclass —— 班级结业典礼程序

用 Rust + [windows-rs](https://github.com/microsoft/windows-rs) 写的小程序，给班级结业典礼助兴用。

## 效果

1. 启动后，**多线程并发弹出若干个「System Error」消息框**（系统错误图标、置顶），看起来像系统出故障了；
2. 当所有消息框都被点掉后，**自动全屏播放 `再见文山.mov`** 视频（调用系统默认播放器打开）；
3. 视频播放约 **2 分半**后，像「小病毒」一样**批量弹出几十个网页**（GitHub 主页 + 个人站点），紧接着**打开 Windows 相机**，让全班同学在屏幕上看到自己。

> 设计在希沃白板（Windows 系统）上运行。

## 目录结构

```
overtheclass/
├── .cargo/
│   └── config.toml    ← 交叉编译链接器配置
├── Cargo.toml
├── src/
│   └── main.rs
├── README.md          ← 本文件
└── 再见文山.mov        ← 你需要自己放进来的视频（与 exe 同目录）
```

## 视频文件

程序会打开与可执行文件**同目录**下的 `再见文山.mov`。

- 也可以在编译时通过环境变量 `OVERTHECLASS_VIDEO` 指定别的文件名，例如：
  ```sh
  OVERTHECLASS_VIDEO=zaijian.mov cargo build --release --target x86_64-pc-windows-gnu
  ```
- 运行时务必把视频和 `overtheclass.exe` 放在同一个文件夹里。
- `.mov` / `.mp4` 等常见格式均可，由 Windows 系统默认播放器打开。

## 交叉编译（macOS → Windows）

因为要在希沃白板（Windows）上运行，而开发机是 macOS，需要交叉编译为 `x86_64-pc-windows-gnu`。
选用 **MinGW-w64 + GNU target** 而非 MSVC target，原因：

- 工具链一条命令装好，不用下载庞大的 Windows SDK / MSVC 头文件；
- `windows-rs` 调用的 `MessageBoxA` / `ShellExecuteW` 等标准 Win32 API，MinGW 自带导入库完全支持；
- 配合 `-static` 静态链接，生成的 exe 不依赖任何 mingw 运行时 DLL，拷到任意 Windows 机器即可运行。

### 一次性环境准备

```sh
# 1. 添加 Windows GNU 目标
rustup target add x86_64-pc-windows-gnu

# 2. 安装 MinGW-w64 交叉工具链（提供 x86_64-w64-mingw32-gcc 链接器）
brew install mingw-w64
```

`.cargo/config.toml` 已配好（本仓库自带）：

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
rustflags = ["-C", "link-args=-static"]
```

### 编译

```sh
cargo build --release --target x86_64-pc-windows-gnu
```

产物路径：

```
target/x86_64-pc-windows-gnu/release/overtheclass.exe
```

### 在希沃白板上运行

1. 把 `overtheclass.exe` 和 `再见文山.mov` 拷到希沃白板的同一个文件夹（例如 U 盘或桌面）；
2. 双击 `overtheclass.exe`；
3. 依次点掉弹出的几个「System Error」消息框；
4. 视频自动全屏播放；
5. 播放约 2 分半后，屏幕开始疯狂弹网页，随后打开相机 🎉

## 自定义

均在 `src/main.rs` 顶部常量区修改：

- **消息框数量 / 文案**：`ERROR_MESSAGES` 数组。
- **视频文件名**：`VIDEO_FILE` 常量，或用 `OVERTHECLASS_VIDEO` 环境变量。
- **「小病毒」触发时机**：`VIRUS_DELAY_SECS`（默认 150 秒 ≈ 2 分半）。
- **弹网页数量**：`VIRUS_WINDOW_COUNT`（默认 30）。
- **弹哪些网址**：`VIRUS_URLS`（默认 GitHub 主页 + 个人站点）。
- **弹框后停顿时长**：`thread::sleep(Duration::from_millis(600))`。

## 说明

- 消息框使用 `MessageBoxA` + 纯 ASCII 文案，避免在不同 ANSI 代码页下出现乱码；
- 消息框带 `MB_TOPMOST | MB_SETFOREGROUND`，保证在希沃白板全屏时也能弹到最前；
- 视频通过 `ShellExecuteW` 以 `SW_SHOWMAXIMIZED` 方式打开，调用系统默认播放器；
- 批量开网页时直接重复打开同一 URL，Edge/Chrome 默认每次 `ShellExecuteW` 都开新标签，地址栏保持干净的原始 URL；
- 相机通过 UWP 协议 `microsoft.windows.camera:` 拉起，需 Windows 10 及以上自带相机应用。
