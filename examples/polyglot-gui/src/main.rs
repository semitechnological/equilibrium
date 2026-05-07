use std::time::Duration;

use crepuscularity_gpui::prelude::*;
use gpui::{
    div, px, rgb, size, App, Application, AsyncApp, Bounds, Div, MouseButton, MouseDownEvent, Task,
    Timer, WeakEntity, Window, WindowBounds, WindowOptions,
};

include!(concat!(env!("OUT_DIR"), "/polyglot_gui_template.rs"));

mod polyglot;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Constellation,
    Pipeline,
}

impl Mode {
    fn template_name(self) -> &'static str {
        match self {
            Mode::Constellation => "constellation",
            Mode::Pipeline => "dashboard",
        }
    }
}

struct CrepusShellParts {
    mode_switcher: Div,
    pipeline_controls: Div,
}

struct PolyglotTemplate<R, P, C> {
    parts: CrepusShellParts,
    n: i64,
    mode: &'static str,
    is_gui: bool,
    is_tui: bool,
    is_dashboard: bool,
    is_constellation: bool,
    linked_count: i64,
    missing_count: i64,
    gui_rows: R,
    gui_constellation_preview_rows: P,
    gui_constellation_rows: C,
    pipeline_scroll: i64,
}

struct Dashboard {
    _animation: Task<()>,
    n: i64,
    mode: Mode,
    tick: u64,
}

impl Dashboard {
    fn new(cx: &mut Context<Self>) -> Self {
        let animation = cx.spawn(
            async move |this: WeakEntity<Dashboard>, cx: &mut AsyncApp| loop {
                Timer::after(Duration::from_millis(50)).await;
                if this
                    .update(cx, |this, cx| {
                        this.advance(cx);
                    })
                    .is_err()
                {
                    break;
                }
            },
        );

        Self {
            _animation: animation,
            n: 7,
            mode: Mode::Constellation,
            tick: 0,
        }
    }

    fn set_mode(&mut self, mode: Mode, cx: &mut Context<Self>) {
        self.mode = mode;
        cx.notify();
    }

    fn shift(&mut self, amount: i64, cx: &mut Context<Self>) {
        self.n = (self.n + amount).clamp(0, 1_000_000);
        cx.notify();
    }

    fn dec_100(&mut self, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.shift(-100, cx);
    }
    fn dec_10(&mut self, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.shift(-10, cx);
    }
    fn dec_1(&mut self, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.shift(-1, cx);
    }
    fn inc_1(&mut self, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.shift(1, cx);
    }
    fn inc_10(&mut self, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.shift(10, cx);
    }
    fn inc_100(&mut self, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.shift(100, cx);
    }
    fn reset(&mut self, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.n = 7;
        cx.notify();
    }

    fn advance(&mut self, cx: &mut Context<Self>) {
        self.tick = self.tick.wrapping_add(1);
        cx.notify();
    }
}

impl Render for Dashboard {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let n = self.n;
        let snapshot = polyglot::snapshot(n);
        let mode_switcher = mode_switcher(self.mode, cx);
        let pipeline_controls = control_strip(cx);
        let full_constellation_rows =
            polyglot::constellation_rows(self.tick, 118, 44, &snapshot.rows);
        let preview_constellation_rows =
            polyglot::constellation_rows(self.tick, 58, 12, &snapshot.rows);
        let rows = snapshot
            .rows
            .iter()
            .map(|row| {
                (
                    row.lang.to_string(),
                    row.linked,
                    row.result.clone(),
                    if row.linked { "LINKED" } else { "MISSING" },
                    row.accent,
                )
            })
            .collect::<Vec<_>>()
            .into_iter();
        let preview_rows = preview_constellation_rows.into_iter();
        let constellation_rows = full_constellation_rows.into_iter();

        render_crepus_shell(PolyglotTemplate {
            parts: CrepusShellParts {
                mode_switcher,
                pipeline_controls,
            },
            n: snapshot.n,
            mode: self.mode.template_name(),
            is_gui: true,
            is_tui: false,
            is_dashboard: self.mode == Mode::Pipeline,
            is_constellation: self.mode == Mode::Constellation,
            linked_count: snapshot.linked_count,
            missing_count: snapshot.missing_count,
            gui_rows: rows,
            gui_constellation_preview_rows: preview_rows,
            gui_constellation_rows: constellation_rows,
            pipeline_scroll: 0,
        })
    }
}

fn mode_switcher(mode: Mode, cx: &mut Context<Dashboard>) -> Div {
    let constellation = pill(
        "Constellation",
        cx.listener(|this, _, _, cx| this.set_mode(Mode::Constellation, cx)),
    )
    .when(mode == Mode::Constellation, |cx| {
        cx.bg(rgb(0x1d4ed8)).text_color(rgb(0xffffff))
    });
    let pipeline = pill(
        "Pipeline",
        cx.listener(|this, _, _, cx| this.set_mode(Mode::Pipeline, cx)),
    )
    .when(mode == Mode::Pipeline, |cx| {
        cx.bg(rgb(0x7c3aed)).text_color(rgb(0xffffff))
    });

    div()
        .absolute()
        .left(px(28.0))
        .top(px(24.0))
        .flex()
        .gap_2()
        .child(constellation)
        .child(pipeline)
}

fn control_strip(cx: &mut Context<Dashboard>) -> Div {
    div()
        .w_full()
        .max_w(px(430.0))
        .flex()
        .flex_nowrap()
        .gap_1()
        .items_end()
        .justify_end()
        .overflow_hidden()
        .mt(px(12.0))
        .child(control_pill("-100", cx.listener(Dashboard::dec_100)))
        .child(control_pill("-10", cx.listener(Dashboard::dec_10)))
        .child(control_pill("-1", cx.listener(Dashboard::dec_1)))
        .child(control_pill("+1", cx.listener(Dashboard::inc_1)))
        .child(control_pill("+10", cx.listener(Dashboard::inc_10)))
        .child(control_pill("+100", cx.listener(Dashboard::inc_100)))
        .child(control_pill("reset", cx.listener(Dashboard::reset)))
}

fn pill(label: &str, on_click: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static) -> Div {
    div()
        .px_3()
        .py_1()
        .rounded_full()
        .bg(rgb(0x18181b))
        .border_1()
        .border_color(rgb(0x27272a))
        .cursor_pointer()
        .child(label.to_string())
        .on_mouse_down(MouseButton::Left, on_click)
}

fn control_pill(
    label: &str,
    on_click: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> Div {
    div()
        .px_2()
        .py_1()
        .rounded_full()
        .bg(rgb(0x18181b))
        .border_1()
        .border_color(rgb(0x27272a))
        .text_xs()
        .cursor_pointer()
        .child(label.to_string())
        .on_mouse_down(MouseButton::Left, on_click)
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1440.0), px(900.0)), cx);
        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(Dashboard::new),
        );
        cx.activate(true);
    });
}
