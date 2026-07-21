//! 独立滚动条组件（自绘，≈ LibreOffice `ScrollBar` `aHScroll`/`aVScroll`）。
//!
//! LibreOffice 的 `ScrollBar` 是 VCL 控件（`sc/source/ui/view/tabview.cxx` 的
//! `aHScroll`/`aVScroll`，底层 `vcl/source/control/scrbar.cxx`），其几何由
//! `SetRange` / `SetThumbPos` / `SetVisibleSize` 描述。EWP 为守住「单 canvas 红线」
//! （禁止 `ScrollHandle`/`overflow_*_scroll`/`track_scroll`/`_b.origin`），不引入任何
//! GPUI 原生滚动容器，而是用纯 `div`/`canvas` 自绘：thumb 的「长度 / 位置」完全由
//! `viewport / total`、`scroll / total` 的比例算法推出（与 LibreOffice 等价）。
//!
//! 拖拽用 GPUI 0.2.2 的「全局鼠标事件」机制（**无 `capture_mouse`**）：
//! 在组件自身的 `canvas` paint 闭包（Paint 阶段）里 `window.on_mouse_event(...)`
//! 注册 `MouseDown`/`MouseMove`/`MouseUp` 监听器——这些监听器对全窗口的鼠标事件
//! 生效（无论指针是否在组件内），等价于「捕获鼠标」。本模式与 `gpui-component`
//! 0.5.1 自带 scrollbar 的实现完全一致。所有写入都经 `on_drag` 回调回到 `SheetView`
//! → `SheetViewState.set_scroll_x/y` + `cx.notify()`，绝不移动任何窗口几何（红线 4）。

use gpui::{
    App, Bounds, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Window, canvas,
    div, fill, point, px, size,
};
use gpui::prelude::*;
use std::rc::Rc;

use crate::sheet::view_state::SheetViewState;
use crate::styles::ThemeColors;

/// 滚动条 thumb 几何（track 局部坐标，像素）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ScrollbarThumb {
    /// thumb 起点（沿 track，距 track 左/上缘）。
    pub start: f32,
    /// thumb 长度。
    pub size: f32,
}

// mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::ScrollHdl → aHScroll.SetRange/SetThumbPos/SetVisibleSize
//   C++（vcl/source/control/scrbar.cxx 逐字核心，等价比例）：
//     nThumbSize = nVisibleSize;                  // 可见尺寸 → thumb 长度
//     nThumbPos  = nTop;                          // 顶部位置 → thumb 起点
//     nRange     = nMax - nMin;                   // 文档总长
//   Rust 逐行对应（自绘，避免引入 VCL ScrollBar）：
//     let ratio = (viewport / total).clamp(0,1);  // ≈ nVisibleSize / nRange
//     let size  = ratio * track_len;              // ≈ SetVisibleSize → thumb 长度
//     let start = (scroll / total).clamp(0,1) * track_len;  // ≈ SetThumbPos → thumb 起点
//   等价：thumb 占 track 的比例 = viewport/total；thumb 起点比例 = scroll/total（与 LibreOffice 一致）。
/// 比例算法：由 `scroll / viewport / total / track_len` 推出 thumb 几何。
/// `size = viewport/total * track_len`；`start = scroll/total * track_len`（再夹到 `[0, track_len-size]`）。
pub fn thumb_metrics(
    scroll: f32,
    viewport: f32,
    total: f32,
    track_len: f32,
) -> ScrollbarThumb {
    let total = total.max(f32::EPSILON);
    let ratio = (viewport / total).clamp(0.0, 1.0);
    let size = ratio * track_len;
    let max_start = (track_len - size).max(0.0);
    let start = ((scroll / total).clamp(0.0, 1.0) * track_len).clamp(0.0, max_start);
    ScrollbarThumb { start, size }
}

// mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScrollHdl 反解：从 thumb 起点反算 nPosX
//   C++：nPosX = nThumbPos * (nMax-nMin) / nRange;     // thumb 比例 → 文档位置
//   Rust 逐行对应：
//     let raw = start / track_len * total;            // ≈ nPosX 反解
//     raw.clamp(0, max(0, total-viewport))            // ≈ 合法范围（与 set_scroll_x/y 一致）
/// 反解：拖拽到 `thumb_start` → 对应 scroll 像素，再 clamp 到 `[0, total-viewport]`。
/// 与 `thumb_metrics` 严格互逆（退化 `total<=viewport` 时退 0）。
pub fn scroll_from_thumb(start: f32, track_len: f32, viewport: f32, total: f32) -> f32 {
    let track_len = track_len.max(f32::EPSILON);
    let raw = start / track_len * total;
    raw.clamp(0.0, (total - viewport).max(0.0))
}

/// 在组件 paint 闭包内绘制 track + thumb，并注册全局鼠标事件完成拖拽。
/// `drag_grab` 为渲染时 `SheetView.hscroll_drag/vscroll_drag` 的快照（抓取偏移）；
/// 仅当其为 `Some` 时才注册 `MouseMove`/`MouseUp`（拖拽进行中）。
/// `on_drag(target, window, cx)` 把目标 scroll 写回 `SheetView`；
/// `on_drag_begin(grab, window, cx)` 在开始拖拽时记录抓取偏移；
/// `on_drag_end(window, cx)` 在松手时清除拖拽态。
fn paint_scrollbar(
    is_vertical: bool,
    bounds: Bounds<Pixels>,
    window: &mut Window,
    state: SheetViewState,
    total: f32,
    viewport: f32,
    drag_grab: Option<f32>,
    on_drag: Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>,
    on_drag_begin: Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>,
    on_drag_end: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
) {
    let c = ThemeColors::current();
    let track_len = if is_vertical {
        f32::from(bounds.size.height)
    } else {
        f32::from(bounds.size.width)
    };

    // track 背景（≈ ScrollBar 槽）。
    window.paint_quad(fill(bounds, c.sidebar_bg));

    // 不可滚（total <= viewport）：不画 thumb、不注册拖拽。
    if total <= viewport {
        return;
    }

    let thumb = thumb_metrics(
        if is_vertical { state.scroll_y } else { state.scroll_x },
        viewport,
        total,
        track_len,
    );
    let (tx, ty, tw, th) = if is_vertical {
        (
            f32::from(bounds.origin.x) + 2.0,
            f32::from(bounds.origin.y) + thumb.start,
            f32::from(bounds.size.width) - 4.0,
            thumb.size,
        )
    } else {
        (
            f32::from(bounds.origin.x) + thumb.start,
            f32::from(bounds.origin.y) + 2.0,
            thumb.size,
            f32::from(bounds.size.height) - 4.0,
        )
    };
    window.paint_quad(fill(
        Bounds::new(point(px(tx), px(ty)), size(px(tw), px(th))),
        c.text_muted,
    ));

    // ── 全局 MouseDown：开始拖拽（命中 track/thumb 即启动） ──
    // mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::ScrollHdl（点击 thumb 抓取 / 点击 track 跳转）
    let state_d = state;
    let total_d = total;
    let viewport_d = viewport;
    let on_drag_d = on_drag.clone();
    let on_drag_begin_d = on_drag_begin.clone();
    window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
        if !phase.bubble() || !bounds.contains(&event.position) {
            return;
        }
        // 局部坐标（沿 track 轴）。
        let local = if is_vertical {
            f32::from(event.position.y - bounds.origin.y)
        } else {
            f32::from(event.position.x - bounds.origin.x)
        };
        let t = thumb_metrics(
            if is_vertical {
                state_d.scroll_y
            } else {
                state_d.scroll_x
            },
            viewport_d,
            total_d,
            track_len,
        );
        let grab = if local >= t.start && local <= t.start + t.size {
            // 命中 thumb：记录指针在 thumb 内的偏移，拖拽时 thumb 不跳变。
            local - t.start
        } else {
            // 命中 track：跳转使 thumb 中心落到点击处，抓取偏移取 thumb 半长。
            let target = scroll_from_thumb(local - t.size / 2.0, track_len, viewport_d, total_d);
            on_drag_d(target, window, cx);
            t.size / 2.0
        };
        on_drag_begin_d(grab, window, cx);
    });

    // ── 仅拖拽进行中：全局 MouseMove（跟随指针）+ MouseUp（结束） ──
    if let Some(grab) = drag_grab {
        let total_m = total;
        let viewport_m = viewport;
        let on_drag_m = on_drag.clone();
        window.on_mouse_event(move |event: &MouseMoveEvent, _phase, window, cx| {
            // 仅左键按下时视为拖拽（全局监听，等价于「捕获鼠标」）。
            if event.pressed_button != Some(MouseButton::Left) {
                return;
            }
            let local = if is_vertical {
                f32::from(event.position.y - bounds.origin.y)
            } else {
                f32::from(event.position.x - bounds.origin.x)
            };
            let target = scroll_from_thumb(local - grab, track_len, viewport_m, total_m);
            on_drag_m(target, window, cx);
        });
        let on_drag_end_m = on_drag_end.clone();
        window.on_mouse_event(move |_event: &MouseUpEvent, _phase, window, cx| {
            on_drag_end_m(window, cx);
        });
    }
}

// mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::ScrollHdl → pViewData->Scroll(...) → SetPosX + Invalidate
//   C++（设计还原）：用户拖横滚动条 → ScrollHdl → pViewData->SetPosX(newX) + Invalidate(pane)
//   Rust 逐行对应：on_drag(target) → SheetView::on_scrollbar_drag(H, target) → state.set_scroll_x(target, viewport_w, total_w) + cx.notify()
/// 横向滚动条组件（canvas 自绘 + 全局鼠标拖拽）。`drag_grab` 为 `SheetView.hscroll_drag` 快照。
pub fn render_h_scrollbar(
    state: SheetViewState,
    total_w: f32,
    viewport_w: f32,
    drag_grab: Option<f32>,
    on_drag: Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>,
    on_drag_begin: Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>,
    on_drag_end: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
) -> impl IntoElement {
    div()
        .id("h-scrollbar")
        .h(px(12.))
        .flex_1()
        .child(canvas(
            |_bounds, _window, _cx| (),
            move |bounds, _prepaint, window, cx| {
                paint_scrollbar(
                    false, bounds, window, state, total_w, viewport_w, drag_grab, on_drag,
                    on_drag_begin, on_drag_end,
                );
                let _ = (window, cx);
            },
        ))
}

// mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::ScrollVHdl → pViewData->Scroll(...) → SetPosY + Invalidate
//   C++（设计还原）：用户拖纵滚动条 → ScrollVHdl → pViewData->SetPosY(newY) + Invalidate(pane)
//   Rust 逐行对应：on_drag(target) → SheetView::on_scrollbar_drag(V, target) → state.set_scroll_y(target, viewport_h, total_h) + cx.notify()
/// 纵向滚动条组件（canvas 自绘 + 全局鼠标拖拽）。`drag_grab` 为 `SheetView.vscroll_drag` 快照。
pub fn render_v_scrollbar(
    state: SheetViewState,
    total_h: f32,
    viewport_h: f32,
    drag_grab: Option<f32>,
    on_drag: Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>,
    on_drag_begin: Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>,
    on_drag_end: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
) -> impl IntoElement {
    div()
        .id("v-scrollbar")
        .w(px(12.))
        .child(canvas(
            |_bounds, _window, _cx| (),
            move |bounds, _prepaint, window, cx| {
                paint_scrollbar(
                    true, bounds, window, state, total_h, viewport_h, drag_grab, on_drag,
                    on_drag_begin, on_drag_end,
                );
                let _ = (window, cx);
            },
        ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── thumb 比例算法（与 LibreOffice SetVisibleSize/SetThumbPos 等价） ──

    #[test]
    fn thumb_metrics_proportional() {
        // total=1000, viewport=200 → ratio 0.2；track=100 → size=20。
        let t = thumb_metrics(0.0, 200.0, 1000.0, 100.0);
        assert_eq!(t.size, 20.0);
        // 滚动到 500/1000 → start = 0.5 * 100 = 50。
        let t2 = thumb_metrics(500.0, 200.0, 1000.0, 100.0);
        assert_eq!(t2.start, 50.0);
        assert_eq!(t2.size, 20.0);
        // 滚动到 max=800 → start = 80（= track - size）。
        let t3 = thumb_metrics(800.0, 200.0, 1000.0, 100.0);
        assert_eq!(t3.start, 80.0);
        assert_eq!(t3.size, 20.0);
    }

    #[test]
    fn thumb_metrics_full_when_not_scrollable() {
        // total <= viewport → ratio 夹到 1 → size = track（满槽），start=0。
        let t = thumb_metrics(0.0, 1000.0, 500.0, 100.0);
        assert_eq!(t.size, 100.0);
        assert_eq!(t.start, 0.0);
        // 非零 scroll 也退化为满槽、起点 0。
        let t2 = thumb_metrics(300.0, 1000.0, 500.0, 100.0);
        assert_eq!(t2.size, 100.0);
        assert_eq!(t2.start, 0.0);
    }

    #[test]
    fn scroll_from_thumb_inverse_of_thumb_metrics() {
        // 往返：thumb.start → scroll_from_thumb → 原 scroll（退化区间内严格互逆）。
        for &s in [0.0_f32, 100.0, 400.0, 800.0].iter() {
            let tm = thumb_metrics(s, 200.0, 1000.0, 100.0);
            let back = scroll_from_thumb(tm.start, 100.0, 200.0, 1000.0);
            assert!((back - s).abs() < 1e-3, "roundtrip 失败 s={s} back={back}");
        }
    }

    #[test]
    fn scroll_from_thumb_clamps() {
        // 远超上下界 → 夹到 [0, total-viewport] = [0, 800]。
        assert_eq!(scroll_from_thumb(9999.0, 100.0, 200.0, 1000.0), 800.0);
        assert_eq!(scroll_from_thumb(-50.0, 100.0, 200.0, 1000.0), 0.0);
        // total <= viewport 退化 → 永远 0。
        assert_eq!(scroll_from_thumb(50.0, 100.0, 1000.0, 500.0), 0.0);
    }

    #[test]
    fn thumb_and_scroll_consistent_across_track_lengths() {
        // 同一 scroll 比例在不同 track 长度下，thumb 位置比例一致（与 LibreOffice 同构）。
        for &track in [80.0_f32, 200.0, 640.0].iter() {
            let s = 400.0;
            let tm = thumb_metrics(s, 200.0, 1000.0, track);
            // start/track 应等于 scroll/total = 0.4。
            assert!((tm.start / track - 0.4).abs() < 1e-3, "track={track}");
            let back = scroll_from_thumb(tm.start, track, 200.0, 1000.0);
            assert!((back - s).abs() < 1e-3, "roundtrip track={track}");
        }
    }
}
