//! 班级结业典礼程序
//!
//! 流程：
//! 1. 多线程并发弹出若干个 "System Error" 消息框（MessageBoxA），营造蓝屏告警效果；
//! 2. 等待所有消息框被关闭后，用 ShellExecuteW 全屏播放 "再见文山.mov" 视频；
//! 3. 视频播放约 2 分半后，像"小病毒"一样批量打开几十个网页（GitHub 主页 + 个人站点），
//!    紧接着打开 Windows 相机，让全班同学看到自己。
//!
//! 目标平台：Windows（希沃白板）。在 macOS 上交叉编译为 x86_64-pc-windows-gnu。

#[cfg(windows)]
mod app {
    use std::thread;
    use std::time::Duration;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::Win32::UI::WindowsAndMessaging::{
        MB_ICONERROR, MB_OK, MB_SETFOREGROUND, MB_TOPMOST, MessageBoxA, SW_SHOWMAXIMIZED,
        SW_SHOWNORMAL,
    };
    use windows::core::{PCSTR, PCWSTR};

    /// 与 exe 同目录的视频文件名。
    /// 可在编译时通过环境变量 OVERTHECLASS_VIDEO 覆盖，例如：
    ///   OVERTHECLASS_VIDEO=再见文山.mov cargo build ...
    const VIDEO_FILE: &str = match option_env!("OVERTHECLASS_VIDEO") {
        Some(v) => v,
        None => "再见文山.mov",
    };

    /// 视频开始播放后，等待多少秒再触发"小病毒"环节。
    const VIRUS_DELAY_SECS: u64 = 150; // 约 2 分半

    /// "小病毒"阶段总共要打开多少个网页窗口。
    const VIRUS_WINDOW_COUNT: usize = 30;

    /// 循环打开的网址列表。两个站点交替开，让屏幕更热闹。
    const VIRUS_URLS: &[&str] = &[
        "https://github.com/YangZhenxun",
        "https://yangzhenxun.github.io",
        "https://www.luogu.com.cn/user/1654430",
        "https://github.com/QiuyuzeDERECK",
    ];

    /// 并发弹出的"系统错误"消息（纯 ASCII，避免 MessageBoxA 在非中文代码页下乱码）。
    const ERROR_MESSAGES: &[(&str, &str)] = &[
        (
            "System Error",
            "0xC0000021 : STATUS_SYSTEM_PROCESS_TERMINATED",
        ),
        ("System Error", "0x80004005 : Unspecified error"),
        ("System Error", "0x000000ED : UNMOUNTABLE_BOOT_VOLUME"),
        ("System Error", "A critical system process has failed."),
        ("System Error", "0xDEADDEAD : MANUALLY_INITIATED_CRASH"),
        ("System Error", "0x0000007B : INACCESSIBLE_BOOT_DEVICE"),
    ];

    /// 弹出一个 MessageBoxA（系统错误图标，置顶）。
    fn show_error(title: &str, body: &str) {
        let title_c = format!("{title}\0");
        let body_c = format!("{body}\0");
        unsafe {
            MessageBoxA(
                Some(HWND::default()),
                PCSTR(body_c.as_ptr()),
                PCSTR(title_c.as_ptr()),
                MB_ICONERROR | MB_OK | MB_TOPMOST | MB_SETFOREGROUND,
            );
        }
    }

    /// 用 ShellExecuteW 打开一个 URL / 协议 / 文件（系统默认处理方式）。
    /// `maximized` 为 true 时最大化打开（视频、相机），否则普通窗口（网页）。
    ///
    /// 对 URI 协议（含 ":"，如 `microsoft.windows.camera:`、`https://...`），
    /// verb 传 NULL 更标准——系统走协议的默认处理程序；
    /// 对文件（如视频），verb 用 "open" 调用关联程序打开。
    fn shell_open(target: &str, maximized: bool) {
        let wide: Vec<u16> = target.encode_utf16().chain(std::iter::once(0u16)).collect();
        // 协议（带冒号）用 NULL verb；文件用 "open" verb。
        let is_protocol = target.contains(':');
        let open_verb: Vec<u16> = "open\0".encode_utf16().collect();
        let verb = if is_protocol {
            PCWSTR::null()
        } else {
            PCWSTR(open_verb.as_ptr())
        };
        let show = if maximized {
            SW_SHOWMAXIMIZED
        } else {
            SW_SHOWNORMAL
        };
        unsafe {
            let _ = ShellExecuteW(
                Some(HWND::default()),
                verb,
                PCWSTR(wide.as_ptr()),
                PCWSTR::null(),
                PCWSTR::null(),
                show,
            );
        }
    }

    /// 全屏播放视频文件（使用系统默认播放器打开）。
    fn play_video() {
        shell_open(VIDEO_FILE, true);
    }

    /// 像小病毒一样批量打开网页。
    /// 直接重复打开同一 URL：Edge/Chrome 默认每次 ShellExecuteW 都开新标签，
    /// 地址栏保持干净的原始 URL，无需任何锚点或查询参数。
    fn open_websites_like_virus() {
        for i in 0..VIRUS_WINDOW_COUNT {
            let url = VIRUS_URLS[i % VIRUS_URLS.len()];
            shell_open(url, false);
            // 120ms 一个，快速连开，呈现"病毒爆发"的瞬间冲击感。
            thread::sleep(Duration::from_millis(120));
        }
    }

    /// 打开 Windows UWP 相机，让全班同学看到自己。
    fn open_camera() {
        // microsoft.windows.camera: 是 UWP 相机的协议，ShellExecuteW 可直接拉起。
        shell_open("microsoft.windows.camera:", true);
    }

    pub fn run() {
        // 阶段一：间隔 1 秒逐个弹出 System Error 消息框，营造"错误一个接一个冒出来"的效果，
        // 而不是 6 个同时砸到屏幕上。每个线程阻塞在自己的 MessageBoxA 上，直到用户点击"确定"。
        let handles: Vec<_> = ERROR_MESSAGES
            .iter()
            .map(|(title, body)| {
                let title = (*title).to_string();
                let body = (*body).to_string();
                let h = thread::spawn(move || show_error(&title, &body));
                // 弹出一个后等 1 秒再弹下一个。
                thread::sleep(Duration::from_secs(1));
                h
            })
            .collect();

        // 等待全部消息框被关闭后再继续。
        for h in handles {
            let _ = h.join();
        }

        // 短暂停顿，让屏幕"恢复"一下再放视频。
        thread::sleep(Duration::from_millis(600));

        // 阶段二：播放"再见，文山"视频。
        play_video();

        // 阶段三：视频异步播放（ShellExecuteW 立即返回），主线程在此计时等待。
        // 约播放 2 分半后，启动"小病毒"环节：批量开网页 + 打开相机。
        let virus = thread::spawn(|| {
            thread::sleep(Duration::from_secs(VIRUS_DELAY_SECS));
            open_websites_like_virus();
            // 给浏览器一点时间把窗口都弹出来，再开相机。
            thread::sleep(Duration::from_millis(800));
            open_camera();
        });
        let _ = virus.join();
    }
}

#[cfg(not(windows))]
mod app {
    pub fn run() {
        eprintln!(
            "此程序为 Windows 设计。请交叉编译：\n  \
             cargo build --release --target x86_64-pc-windows-gnu"
        );
    }
}

fn main() {
    app::run();
}
