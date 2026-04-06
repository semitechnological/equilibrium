//! Equilibrium Polyglot Calculator — GPUI desktop app (multi-mode)
//!
//! Three modes: Calculator, Sequence, Languages
//!
//! ## Building
//! Windows (native, recommended):
//!   cargo build --release
//!
//! Linux with Vulkan:
//!   RUSTFLAGS="-L /tmp/gpui-libs" cargo build --bin polyglot-gui
//!
//! macOS:
//!   cargo build --release

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use gpui::*;
use std::os::raw::c_int;

// ── C FFI (always linked) ────────────────────────────────────────────────────
#[cfg(has_c)]
mod c_ffi {
    include!(concat!(env!("OUT_DIR"), "/c_bindings.rs"));
}

// ── C++ FFI (always linked) ──────────────────────────────────────────────────
#[cfg(has_cpp)]
mod cpp_ffi {
    include!(concat!(env!("OUT_DIR"), "/cpp_bindings.rs"));
}

// ── Zig FFI (linked when zig was found at build time) ────────────────────────
#[cfg(has_zig)]
extern "C" {
    fn zig_square(n: c_int) -> c_int;
    fn zig_sum_1_to_n(n: i64) -> i64;
    fn zig_is_power_of_two(n: u64) -> bool;
}

// ── Nim FFI (linked when nim was found at build time) ────────────────────────
#[cfg(has_nim)]
extern "C" {
    fn nim_popcount(n: u32) -> i32;
    fn nim_reverse_bits(n: u32) -> u32;
}

// ── V FFI (linked when v was found at build time) ────────────────────────────
#[cfg(has_v)]
extern "C" {
    fn v_celsius_to_fahrenheit(c: f64) -> f64;
    fn v_km_to_miles(km: f64) -> f64;
}

// ── D FFI (linked when ldc2 was found at build time) ─────────────────────────
#[cfg(has_d)]
extern "C" {
    fn d_abs(n: i32) -> i32;
    fn d_triangular(n: i32) -> i64;
}

// ── Odin FFI (linked when odin was found at build time) ──────────────────────
#[cfg(has_odin)]
extern "C" {
    fn odin_abs(n: i32) -> i32;
    fn odin_min(a: i32, b: i32) -> i32;
    fn odin_max(a: i32, b: i32) -> i32;
}

// ── Rust native ───────────────────────────────────────────────────────────────
fn rust_is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn rust_next_prime(after: u64) -> u64 {
    let mut n = after + 1;
    while !rust_is_prime(n) {
        n += 1;
    }
    n
}

#[allow(dead_code)]
fn rust_fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 2..=n {
        let c = a.saturating_add(b);
        a = b;
        b = c;
    }
    b
}

#[allow(dead_code)]
fn rust_factorial(n: u64) -> u64 {
    (1..=n).fold(1u64, |acc, x| acc.saturating_mul(x))
}

// ── App state ─────────────────────────────────────────────────────────────────
#[derive(Clone, PartialEq)]
enum Mode {
    Calculator,
    Sequence,
    Languages,
}

struct PolyglotCalc {
    n: i32,
    mode: Mode,
}

impl PolyglotCalc {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            n: 7,
            mode: Mode::Calculator,
        }
    }
}

// ── UI helpers ────────────────────────────────────────────────────────────────

fn result_row(lang: &'static str, linked: bool, text: String) -> impl IntoElement {
    let tag_col = if linked {
        rgb(0x4ade80u32)
    } else {
        rgb(0x52525bu32)
    };
    let txt_col = if linked {
        rgb(0xa1a1aau32)
    } else {
        rgb(0x3f3f46u32)
    };

    div()
        .flex()
        .gap(px(12.))
        .py(px(8.))
        .child(
            div()
                .w(px(56.))
                .flex_shrink_0()
                .text_color(tag_col)
                .text_size(rems(0.75))
                .font_weight(FontWeight::BOLD)
                .child(lang),
        )
        .child(
            div()
                .flex_1()
                .text_color(txt_col)
                .text_size(rems(0.8))
                .child(SharedString::from(text)),
        )
}

fn tab_button(
    id: impl Into<ElementId>,
    label: &'static str,
    active: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .cursor_pointer()
        .px(px(16.))
        .py(px(8.))
        .rounded(px(6.))
        .bg(if active {
            rgb(0x3f3f46u32)
        } else {
            rgb(0x27272au32)
        })
        .text_color(if active {
            rgb(0xfafafau32)
        } else {
            rgb(0x71717au32)
        })
        .text_size(rems(0.875))
        .font_weight(if active {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        })
        .on_click(on_click)
        .child(label)
}

fn seq_cell(text: String, header: bool) -> impl IntoElement {
    div()
        .w(px(100.))
        .flex_shrink_0()
        .px(px(8.))
        .py(px(6.))
        .text_color(if header {
            rgb(0x4ade80u32)
        } else {
            rgb(0xa1a1aau32)
        })
        .text_size(rems(0.8))
        .font_weight(if header {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        })
        .child(SharedString::from(text))
}

fn lang_card(
    lang: &'static str,
    compiler: &'static str,
    linked: bool,
    compiler_found: bool,
) -> impl IntoElement {
    let dot_color = if linked {
        rgb(0x4ade80u32) // green
    } else if compiler_found {
        rgb(0xfbbf24u32) // yellow
    } else {
        rgb(0x52525bu32) // grey
    };
    let status = if linked {
        "linked"
    } else if compiler_found {
        "found"
    } else {
        "absent"
    };

    div()
        .flex()
        .items_center()
        .gap(px(10.))
        .px(px(14.))
        .py(px(10.))
        .rounded(px(6.))
        .bg(rgb(0x18181bu32))
        // dot
        .child(
            div()
                .w(px(8.))
                .h(px(8.))
                .rounded_full()
                .flex_shrink_0()
                .bg(dot_color),
        )
        // lang name
        .child(
            div()
                .w(px(64.))
                .flex_shrink_0()
                .text_color(rgb(0xfafafau32))
                .text_size(rems(0.875))
                .font_weight(FontWeight::BOLD)
                .child(lang),
        )
        // compiler
        .child(
            div()
                .w(px(80.))
                .flex_shrink_0()
                .text_color(rgb(0x71717au32))
                .text_size(rems(0.75))
                .child(compiler),
        )
        // status
        .child(
            div()
                .text_color(dot_color)
                .text_size(rems(0.75))
                .child(status),
        )
}

// ── Render ────────────────────────────────────────────────────────────────────
impl Render for PolyglotCalc {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let n = self.n.max(0);

        // ── Tab bar ──────────────────────────────────────────────────────────
        let tab_bar = div()
            .flex()
            .gap(px(8.))
            .child(tab_button(
                "tab-calc",
                "Calculator",
                self.mode == Mode::Calculator,
                cx.listener(|this, _, _, cx| {
                    this.mode = Mode::Calculator;
                    cx.notify();
                }),
            ))
            .child(tab_button(
                "tab-seq",
                "Sequence",
                self.mode == Mode::Sequence,
                cx.listener(|this, _, _, cx| {
                    this.mode = Mode::Sequence;
                    cx.notify();
                }),
            ))
            .child(tab_button(
                "tab-lang",
                "Languages",
                self.mode == Mode::Languages,
                cx.listener(|this, _, _, cx| {
                    this.mode = Mode::Languages;
                    cx.notify();
                }),
            ));

        // ── Mode content ─────────────────────────────────────────────────────
        let content: AnyElement = match self.mode {
            Mode::Calculator => self.render_calculator(n, cx).into_any_element(),
            Mode::Sequence => self.render_sequence().into_any_element(),
            Mode::Languages => self.render_languages().into_any_element(),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x09090bu32))
            .p(px(24.))
            .gap(px(16.))
            // ── Title
            .child(
                div()
                    .text_color(rgb(0xfafafau32))
                    .text_size(rems(1.2))
                    .font_weight(FontWeight::BOLD)
                    .child("Equilibrium · Polyglot Dashboard"),
            )
            // ── Tabs
            .child(tab_bar)
            // ── Content
            .child(content)
            // ── Footer
            .child(
                div()
                    .text_color(rgb(0x3f3f46u32))
                    .text_size(rems(0.7))
                    .child("Built with equilibrium (auto-generated FFI) + GPUI"),
            )
    }
}

impl PolyglotCalc {
    fn render_calculator(&self, n: i32, cx: &mut Context<Self>) -> impl IntoElement {
        // ── Live FFI calls ────────────────────────────────────────────────────
        #[cfg(has_c)]
        let c_text = unsafe {
            format!(
                "c_add({n},{n}) = {}   c_gcd({n},{}) = {}   c_fibonacci({n}) = {}",
                c_ffi::c_add(n, n),
                n + 1,
                c_ffi::c_gcd(n, n + 1),
                c_ffi::c_fibonacci(n),
            )
        };
        #[cfg(not(has_c))]
        let c_text = String::from("not linked — C compiler absent at build time");

        #[cfg(has_cpp)]
        let cpp_text = unsafe {
            let safe = n.min(20);
            format!(
                "cpp_factorial({safe}) = {}   cpp_is_prime({n}) = {}",
                cpp_ffi::cpp_factorial(safe),
                cpp_ffi::cpp_is_prime(n) != 0,
            )
        };
        #[cfg(not(has_cpp))]
        let cpp_text = String::from("not linked — C++ compiler absent at build time");

        #[cfg(has_zig)]
        let zig_text = unsafe {
            format!(
                "zig_square({n}) = {}   zig_sum_1_to_{n} = {}   zig_is_power_of_two({n}) = {}",
                zig_square(n),
                zig_sum_1_to_n(n as i64),
                zig_is_power_of_two(n as u64),
            )
        };
        #[cfg(not(has_zig))]
        let zig_text = String::from("not linked — zig absent at build time");

        #[cfg(has_nim)]
        let nim_text = unsafe {
            format!(
                "nim_popcount({n}) = {}   nim_reverse_bits({:#010x}) = {:#010x}",
                nim_popcount(n as u32),
                n as u32,
                nim_reverse_bits(n as u32),
            )
        };
        #[cfg(not(has_nim))]
        let nim_text = String::from("not linked — nim absent at build time");

        #[cfg(has_v)]
        let v_text = unsafe {
            format!(
                "v_celsius_to_fahrenheit({n}°C) = {:.1}°F   v_km_to_miles({n}km) = {:.2}mi",
                v_celsius_to_fahrenheit(n as f64),
                v_km_to_miles(n as f64),
            )
        };
        #[cfg(not(has_v))]
        let v_text = String::from("not linked — v absent at build time");

        #[cfg(has_d)]
        let d_text = unsafe {
            format!(
                "d_abs(-{n}) = {}   d_triangular({n}) = {}",
                d_abs(-n),
                d_triangular(n),
            )
        };
        #[cfg(not(has_d))]
        let d_text = String::from("not linked — ldc2 absent at build time");

        #[cfg(has_odin)]
        let odin_text = unsafe {
            format!(
                "odin_abs(-{n}) = {}   odin_min({n},{}) = {}   odin_max({n},{}) = {}",
                odin_abs(-n),
                n + 3,
                odin_min(n, n + 3),
                n + 3,
                odin_max(n, n + 3),
            )
        };
        #[cfg(not(has_odin))]
        let odin_text = String::from("not linked — odin absent at build time");

        let rs_text = format!(
            "rust_is_prime({n}) = {}   rust_next_prime({n}) = {}",
            rust_is_prime(n as u64),
            rust_next_prime(n as u64),
        );

        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            // ── n control
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.))
                    .child(
                        div()
                            .id("dec")
                            .cursor_pointer()
                            .px(px(20.))
                            .py(px(10.))
                            .rounded(px(6.))
                            .bg(rgb(0x27272au32))
                            .text_color(rgb(0xfafafau32))
                            .text_size(rems(1.2))
                            .font_weight(FontWeight::BOLD)
                            .on_click(cx.listener(|this, _ev, _window, cx| {
                                this.n = this.n.saturating_sub(1);
                                cx.notify();
                            }))
                            .child("−"),
                    )
                    .child(
                        div()
                            .w(px(80.))
                            .flex()
                            .justify_center()
                            .text_color(rgb(0xfafafau32))
                            .text_size(rems(2.5))
                            .font_weight(FontWeight::BOLD)
                            .child(SharedString::from(format!("{n}"))),
                    )
                    .child(
                        div()
                            .id("inc")
                            .cursor_pointer()
                            .px(px(20.))
                            .py(px(10.))
                            .rounded(px(6.))
                            .bg(rgb(0x27272au32))
                            .text_color(rgb(0xfafafau32))
                            .text_size(rems(1.2))
                            .font_weight(FontWeight::BOLD)
                            .on_click(cx.listener(|this, _ev, _window, cx| {
                                this.n += 1;
                                cx.notify();
                            }))
                            .child("+"),
                    )
                    .child(
                        div()
                            .id("double")
                            .cursor_pointer()
                            .px(px(20.))
                            .py(px(10.))
                            .rounded(px(6.))
                            .bg(rgb(0x27272au32))
                            .text_color(rgb(0xa78bfau32))
                            .text_size(rems(1.0))
                            .on_click(cx.listener(|this, _ev, _window, cx| {
                                this.n = (this.n * 2).min(10000);
                                cx.notify();
                            }))
                            .child("×2"),
                    )
                    .child(
                        div()
                            .id("reset")
                            .cursor_pointer()
                            .px(px(20.))
                            .py(px(10.))
                            .rounded(px(6.))
                            .bg(rgb(0x27272au32))
                            .text_color(rgb(0x71717au32))
                            .text_size(rems(1.0))
                            .on_click(cx.listener(|this, _ev, _window, cx| {
                                this.n = 7;
                                cx.notify();
                            }))
                            .child("reset"),
                    ),
            )
            // ── Results panel
            .child(
                div()
                    .flex()
                    .flex_col()
                    .rounded(px(8.))
                    .bg(rgb(0x18181bu32))
                    .px(px(20.))
                    .py(px(4.))
                    .child(result_row("C", cfg!(has_c), c_text))
                    .child(result_row("C++", cfg!(has_cpp), cpp_text))
                    .child(result_row("Zig", cfg!(has_zig), zig_text))
                    .child(result_row("Nim", cfg!(has_nim), nim_text))
                    .child(result_row("V", cfg!(has_v), v_text))
                    .child(result_row("D", cfg!(has_d), d_text))
                    .child(result_row("Odin", cfg!(has_odin), odin_text))
                    .child(result_row("Rust", true, rs_text)),
            )
    }

    fn render_sequence(&self) -> impl IntoElement {
        // Header row
        let header = div()
            .flex()
            .gap(px(0.))
            .border_b_1()
            .border_color(rgb(0x27272au32))
            .child(seq_cell("n".to_string(), true))
            .child(seq_cell("fib(n) [C]".to_string(), true))
            .child(seq_cell("fact(n) [C++]".to_string(), true))
            .child(seq_cell("n² [Zig]".to_string(), true))
            .child(seq_cell("tri(n) [D]".to_string(), true))
            .child(seq_cell("prime? [Rs]".to_string(), true));

        let mut rows = div().flex().flex_col().gap(px(2.));
        for i in 1i32..=8 {
            #[cfg(has_c)]
            let fib = unsafe { format!("{}", c_ffi::c_fibonacci(i)) };
            #[cfg(not(has_c))]
            let fib = "—".to_string();

            #[cfg(has_cpp)]
            let fact = unsafe { format!("{}", cpp_ffi::cpp_factorial(i.min(20))) };
            #[cfg(not(has_cpp))]
            let fact = "—".to_string();

            #[cfg(has_zig)]
            let sq = unsafe { format!("{}", zig_square(i)) };
            #[cfg(not(has_zig))]
            let sq = format!("{}", i * i);

            #[cfg(has_d)]
            let tri = unsafe { format!("{}", d_triangular(i)) };
            #[cfg(not(has_d))]
            let tri = format!("{}", (i as i64) * (i as i64 + 1) / 2);

            let prime = if rust_is_prime(i as u64) { "yes" } else { "no" };

            let row = div()
                .flex()
                .gap(px(0.))
                .rounded(px(4.))
                .bg(if i % 2 == 0 {
                    rgb(0x18181bu32)
                } else {
                    rgb(0x09090bu32)
                })
                .child(seq_cell(format!("{i}"), false))
                .child(seq_cell(fib, false))
                .child(seq_cell(fact, false))
                .child(seq_cell(sq, false))
                .child(seq_cell(tri, false))
                .child(seq_cell(prime.to_string(), false));
            rows = rows.child(row);
        }

        div()
            .flex()
            .flex_col()
            .gap(px(4.))
            .child(
                div()
                    .text_color(rgb(0x71717au32))
                    .text_size(rems(0.8))
                    .child("n = 1..8: fibonacci (C), factorial (C++), square (Zig), triangular (D), prime (Rust)"),
            )
            .child(header)
            .child(rows)
    }

    fn render_languages(&self) -> impl IntoElement {
        // Check which compilers are available at runtime for display
        let zig_found = which::which("zig").is_ok();
        let nim_found = which::which("nim").is_ok();
        let v_found = which::which("v").is_ok();
        let d_found = which::which("ldc2").is_ok();
        let odin_found =
            which::which("odin").is_ok() || std::path::Path::new("/usr/local/odin/odin").exists();
        let cs_found = which::which("dotnet").is_ok();
        let hare_found = which::which("hare").is_ok();

        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .child(
                div()
                    .text_color(rgb(0x71717au32))
                    .text_size(rems(0.8))
                    .child("Green = linked via FFI   Yellow = compiler found   Grey = absent"),
            )
            .child(lang_card("C", "gcc", cfg!(has_c), true))
            .child(lang_card("C++", "g++", cfg!(has_cpp), true))
            .child(lang_card("Zig", "zig", cfg!(has_zig), zig_found))
            .child(lang_card("Nim", "nim", cfg!(has_nim), nim_found))
            .child(lang_card("V", "v", cfg!(has_v), v_found))
            .child(lang_card("D", "ldc2", cfg!(has_d), d_found))
            .child(lang_card("Odin", "odin", cfg!(has_odin), odin_found))
            .child(lang_card("C#", "dotnet", false, cs_found))
            .child(lang_card("Hare", "hare", false, hare_found))
            .child(lang_card("Rust", "rustc", true, true))
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────
fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(200.), px(150.)),
                    size: size(px(860.), px(600.)),
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Equilibrium · Polyglot Dashboard")),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(PolyglotCalc::new),
        )
        .unwrap();
    });
}
