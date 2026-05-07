//! Equilibrium Polyglot Calculator — interactive TUI
//!
//! Press ← / → (or h/l) to change n, q to quit.
//! Every keystroke triggers live FFI calls to C, C++, Zig, Nim, V, D, Odin, and Rust.

use crepuscularity_tui::{render_template, TemplateContext, TemplateValue};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color as CrosstermColor, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use ratatui::{
    style::{Color, Style},
    DefaultTerminal, Frame,
};
use std::io;

// ── C FFI (always linked) ────────────────────────────────────────────────────
// ── C++ FFI (always linked) ──────────────────────────────────────────────────
// ── Zig FFI (linked when zig was found at build time) ────────────────────────
// ── Nim FFI ──────────────────────────────────────────────────────────────────
// ── V FFI ─────────────────────────────────────────────────────────────────────
// ── D FFI ─────────────────────────────────────────────────────────────────────
// ── Odin FFI ──────────────────────────────────────────────────────────────────
// ── Rust native ───────────────────────────────────────────────────────────────
mod polyglot;

// ── App state ─────────────────────────────────────────────────────────────────
struct App {
    n: i64,
    mode: Mode,
    tick: u64,
    pipeline_scroll: usize,
}

impl App {
    fn new() -> Self {
        Self {
            n: 7,
            mode: Mode::Dashboard,
            tick: 0,
            pipeline_scroll: 0,
        }
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
    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Dashboard => Mode::Constellation,
            Mode::Constellation => Mode::Dashboard,
        };
    }
    fn set_dashboard(&mut self) {
        self.mode = Mode::Dashboard;
    }
    fn set_constellation(&mut self) {
        self.mode = Mode::Constellation;
    }
    fn scroll_pipeline(&mut self, amount: isize) {
        self.pipeline_scroll = self.pipeline_scroll.saturating_add_signed(amount).min(7);
    }
    fn advance(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Dashboard,
    Constellation,
}

// ── Rendering ─────────────────────────────────────────────────────────────────

const TEMPLATE: &str = include_str!("../templates/polyglot.crepus");

fn ui(frame: &mut Frame, app: &App) {
    let area = frame.area();
    frame
        .buffer_mut()
        .set_style(area, Style::default().fg(Color::White).bg(Color::Black));

    let snapshot = polyglot::snapshot(app.n);
    let mut ctx = TemplateContext::new();
    ctx.set("n", snapshot.n);
    ctx.set("tui_rows", TemplateValue::List(result_rows(&snapshot.rows)));
    ctx.set("mode", app.mode.name());
    ctx.set("is_dashboard", app.mode == Mode::Dashboard);
    ctx.set("is_constellation", app.mode == Mode::Constellation);
    ctx.set("linked_count", snapshot.linked_count);
    ctx.set("missing_count", snapshot.missing_count);
    ctx.set("pipeline_scroll", app.pipeline_scroll as i64);
    ctx.set(
        "tui_constellation_rows",
        TemplateValue::List(text_rows(polyglot::constellation_rows(
            app.tick,
            area.width.saturating_sub(4).max(20) as usize,
            area.height.saturating_sub(2).max(6) as usize,
            &snapshot.rows,
        ))),
    );
    ctx.set(
        "tui_constellation_preview_rows",
        TemplateValue::List(text_rows(polyglot::constellation_rows(
            app.tick,
            58,
            12,
            &snapshot.rows,
        ))),
    );
    let _ = render_template(TEMPLATE, &ctx, frame, area);
}

fn result_rows(rows: &[polyglot::ResultRow]) -> Vec<TemplateContext> {
    rows.iter().map(result_row).collect()
}

fn result_row(row: &polyglot::ResultRow) -> TemplateContext {
    let mut ctx = TemplateContext::new();
    ctx.set("lang", row.lang);
    ctx.set("linked", row.linked);
    ctx.set("result", row.result.clone());
    ctx.set("status", if row.linked { "LINKED" } else { "MISSING" });
    ctx.set("accent", row.accent);
    ctx
}

impl Mode {
    fn name(self) -> &'static str {
        match self {
            Mode::Dashboard => "dashboard",
            Mode::Constellation => "constellation",
        }
    }
}

fn text_rows(lines: Vec<String>) -> Vec<TemplateContext> {
    lines
        .into_iter()
        .map(|line| {
            let mut row = TemplateContext::new();
            row.set("line", line);
            row
        })
        .collect()
}

// ── Entry point ───────────────────────────────────────────────────────────────
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    execute!(
        io::stdout(),
        SetBackgroundColor(CrosstermColor::Black),
        SetForegroundColor(CrosstermColor::White),
        Clear(ClearType::All)
    )?;
    terminal.clear()?;
    let result = run(terminal);
    ratatui::restore();
    execute!(io::stdout(), ResetColor)?;
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
                    KeyCode::Up | KeyCode::Char('k') => app.scroll_pipeline(-1),
                    KeyCode::Down | KeyCode::Char('j') => app.scroll_pipeline(1),
                    KeyCode::Tab | KeyCode::Char('m') => app.toggle_mode(),
                    KeyCode::Char('1') => app.set_dashboard(),
                    KeyCode::Char('2') => app.set_constellation(),
                    _ => {}
                }
            }
        }
        app.advance();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn renders_target_background_and_every_language() {
        let backend = TestBackend::new(96, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new();

        terminal.draw(|frame| ui(frame, &app)).unwrap();

        let buffer = terminal.backend().buffer();
        assert!(buffer.content.iter().all(|cell| cell.style().bg.is_some()));

        let width = buffer.area.width as usize;
        let text = buffer
            .content
            .chunks(width)
            .map(|row| row.iter().map(|cell| cell.symbol()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        for lang in ["C", "C++", "Zig", "Nim", "V", "D", "Odin", "Rust"] {
            assert!(text.contains(lang), "missing language row: {lang}");
        }
        for result in [
            "add=",
            "factorial=",
            "square",
            "LINKED",
            "Session",
            "prime=",
        ] {
            assert!(text.contains(result), "missing result text: {result}");
        }
    }
}
