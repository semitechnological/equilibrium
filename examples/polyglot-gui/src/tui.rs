//! Equilibrium Polyglot Calculator — interactive TUI
//!
//! Press ← / → (or h/l) to change n, q to quit.
//! Every keystroke triggers live FFI calls to C, C++, Zig, Nim, V, D, Odin, and Rust.

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use std::{io, os::raw::c_int};

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

// ── Nim FFI ──────────────────────────────────────────────────────────────────
#[cfg(has_nim)]
extern "C" {
    fn nim_popcount(n: u32) -> i32;
    fn nim_reverse_bits(n: u32) -> u32;
}

// ── V FFI ─────────────────────────────────────────────────────────────────────
#[cfg(has_v)]
extern "C" {
    fn v_celsius_to_fahrenheit(c: f64) -> f64;
    fn v_km_to_miles(km: f64) -> f64;
}

// ── D FFI ─────────────────────────────────────────────────────────────────────
#[cfg(has_d)]
extern "C" {
    fn d_abs(n: i32) -> i32;
    fn d_triangular(n: i32) -> i64;
}

// ── Odin FFI ──────────────────────────────────────────────────────────────────
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

// ── App state ─────────────────────────────────────────────────────────────────
struct App {
    n: i64,
}

impl App {
    fn new() -> Self {
        Self { n: 7 }
    }

    fn increment(&mut self) {
        self.n += 1;
    }
    fn decrement(&mut self) {
        self.n = (self.n - 1).max(0);
    }
    fn double(&mut self) {
        self.n = (self.n * 2).min(9_999_999);
    }
    fn halve(&mut self) {
        self.n = (self.n / 2).max(0);
    }
    fn reset(&mut self) {
        self.n = 7;
    }
}

// ── Rendering ─────────────────────────────────────────────────────────────────

fn ui(frame: &mut Frame, app: &App) {
    let n = app.n;
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Length(3), // controls / n display
            Constraint::Length(3), // key hints
            Constraint::Min(16),   // results (8 langs × 2 lines)
            Constraint::Length(1), // footer
        ])
        .split(area);

    // ── Title ─────────────────────────────────────────────────────────────────
    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            "Equilibrium",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  ·  Polyglot Calculator  ·  "),
        Span::styled(
            "live FFI: C  C++  Zig  Rust",
            Style::default().fg(Color::DarkGray),
        ),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, chunks[0]);

    // ── n display ─────────────────────────────────────────────────────────────
    let n_display = Paragraph::new(Line::from(vec![
        Span::styled("  n = ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{n}"),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(Block::default().borders(Borders::NONE));
    frame.render_widget(n_display, chunks[1]);

    // ── Key hints ─────────────────────────────────────────────────────────────
    let hints = Paragraph::new(Line::from(vec![
        key_hint("← / h", "−1"),
        Span::raw("  "),
        key_hint("→ / l", "+1"),
        Span::raw("  "),
        key_hint("d", "×2"),
        Span::raw("  "),
        key_hint("s", "÷2"),
        Span::raw("  "),
        key_hint("r", "reset"),
        Span::raw("  "),
        key_hint("q", "quit"),
    ]))
    .alignment(Alignment::Left);
    frame.render_widget(hints, chunks[2]);

    // ── Results ───────────────────────────────────────────────────────────────
    let results_area = chunks[3];
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // C
            Constraint::Length(2), // C++
            Constraint::Length(2), // Zig
            Constraint::Length(2), // Nim
            Constraint::Length(2), // V
            Constraint::Length(2), // D
            Constraint::Length(2), // Odin
            Constraint::Length(2), // Rust
        ])
        .split(results_area);

    // C
    #[cfg(has_c)]
    let c_result = unsafe {
        format!(
            "c_add({n},{n}) = {}   c_gcd({n},{}) = {}   c_fibonacci({n}) = {}",
            c_ffi::c_add(n as _, n as _),
            n + 1,
            c_ffi::c_gcd(n as _, (n + 1) as _),
            c_ffi::c_fibonacci(n as _),
        )
    };
    #[cfg(not(has_c))]
    let c_result = String::from("not linked — C compiler was absent at build time");

    render_result(frame, rows[0], "C", cfg!(has_c), &c_result, Color::Green);

    // C++
    #[cfg(has_cpp)]
    let cpp_result = unsafe {
        let safe = n.min(20) as _;
        format!(
            "cpp_factorial({safe}) = {}   cpp_is_prime({n}) = {}",
            cpp_ffi::cpp_factorial(safe),
            cpp_ffi::cpp_is_prime(n as _) != 0,
        )
    };
    #[cfg(not(has_cpp))]
    let cpp_result = String::from("not linked — C++ compiler was absent at build time");

    render_result(
        frame,
        rows[1],
        "C++",
        cfg!(has_cpp),
        &cpp_result,
        Color::Blue,
    );

    // Zig
    #[cfg(has_zig)]
    let zig_result = unsafe {
        format!(
            "zig_square({n}) = {}   zig_sum_1_to_{n} = {}   zig_is_power_of_two({n}) = {}",
            zig_square(n as _),
            zig_sum_1_to_n(n),
            zig_is_power_of_two(n as _),
        )
    };
    #[cfg(not(has_zig))]
    let zig_result = String::from("not linked — zig was absent at build time");

    render_result(
        frame,
        rows[2],
        "Zig",
        cfg!(has_zig),
        &zig_result,
        Color::Yellow,
    );

    // Nim
    #[cfg(has_nim)]
    let nim_result = unsafe {
        format!(
            "nim_popcount({n}) = {}   nim_reverse_bits({n:#010x}) = {:#010x}",
            nim_popcount(n as u32),
            nim_reverse_bits(n as u32),
        )
    };
    #[cfg(not(has_nim))]
    let nim_result = String::from("not linked — nim absent at build time");
    render_result(
        frame,
        rows[3],
        "Nim",
        cfg!(has_nim),
        &nim_result,
        Color::Cyan,
    );

    // V
    #[cfg(has_v)]
    let v_result = unsafe {
        format!(
            "celsius_to_fahrenheit({n}) = {:.1}°F   km_to_miles({n}) = {:.2}mi",
            v_celsius_to_fahrenheit(n as f64),
            v_km_to_miles(n as f64),
        )
    };
    #[cfg(not(has_v))]
    let v_result = String::from("not linked — v absent at build time");
    render_result(frame, rows[4], "V", cfg!(has_v), &v_result, Color::Green);

    // D
    #[cfg(has_d)]
    let d_result = unsafe {
        format!(
            "d_abs(-{n}) = {}   d_triangular({n}) = {}",
            d_abs(-(n as i32)),
            d_triangular(n as i32),
        )
    };
    #[cfg(not(has_d))]
    let d_result = String::from("not linked — ldc2 absent at build time");
    render_result(frame, rows[5], "D", cfg!(has_d), &d_result, Color::Blue);

    // Odin
    #[cfg(has_odin)]
    let odin_result = unsafe {
        format!(
            "odin_abs(-{n}) = {}   odin_min({n},{}) = {}   odin_max({n},{}) = {}",
            odin_abs(-(n as i32)),
            n + 3,
            odin_min(n as i32, (n + 3) as i32),
            n + 3,
            odin_max(n as i32, (n + 3) as i32),
        )
    };
    #[cfg(not(has_odin))]
    let odin_result = String::from("not linked — odin absent at build time");
    render_result(
        frame,
        rows[6],
        "Odin",
        cfg!(has_odin),
        &odin_result,
        Color::LightRed,
    );

    // Rust
    let rs_result = format!(
        "rust_is_prime({n}) = {}   rust_next_prime({n}) = {}",
        rust_is_prime(n as _),
        rust_next_prime(n as _),
    );
    render_result(frame, rows[7], "Rust", true, &rs_result, Color::Magenta);

    // ── Footer ────────────────────────────────────────────────────────────────
    let footer = Paragraph::new(Span::styled(
        "Built with equilibrium (auto-generated FFI bindings)",
        Style::default().fg(Color::DarkGray),
    ))
    .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[4]);
}

fn key_hint<'a>(key: &'a str, label: &'a str) -> Span<'a> {
    Span::styled(
        format!("[{key}] {label}"),
        Style::default().fg(Color::DarkGray),
    )
}

fn render_result(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    lang: &str,
    linked: bool,
    result: &str,
    color: Color,
) {
    let tag_style = if linked {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let result_style = if linked {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let line = Line::from(vec![
        Span::styled(format!("{lang:<5}"), tag_style),
        Span::raw("  "),
        Span::styled(result.to_string(), result_style),
    ]);

    frame.render_widget(Paragraph::new(line).wrap(Wrap { trim: false }), area);
}

// ── Entry point ───────────────────────────────────────────────────────────────
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui(frame, &app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Right | KeyCode::Char('l') => app.increment(),
                    KeyCode::Left | KeyCode::Char('h') => app.decrement(),
                    KeyCode::Char('d') => app.double(),
                    KeyCode::Char('s') => app.halve(),
                    KeyCode::Char('r') => app.reset(),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
