//! Equilibrium Polyglot Dashboard
//!
//! A GUI application (rendered via crepuscularity-web) that shows live results
//! from foreign-language modules compiled by equilibrium. C, C++, and Zig
//! are always linked; other language modules are shown with their compiler status.

use crepuscularity_core::context::{TemplateContext, TemplateValue};
use crepuscularity_web::render_template_to_html;
use equilibrium::{find_compiler, Language};
use std::os::raw::c_int;
use std::path::Path;

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

// ── Zig FFI (linked when zig compiler was found at build time) ───────────────
#[cfg(has_zig)]
extern "C" {
    fn zig_square(n: c_int) -> c_int;
    fn zig_sum_1_to_n(n: i64) -> i64;
    fn zig_is_power_of_two(n: u64) -> bool;
}

// ── Rust native (always available — host language) ───────────────────────────
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

// ── Language card data ────────────────────────────────────────────────────────

struct LangCard {
    name: &'static str,
    extension: &'static str,
    source_file: &'static str,
    fn_sig: &'static str,
    #[allow(dead_code)] // kept for introspection / future use
    equilibrium_lang: Language,
    linked: bool,
    compiler_found: bool,
    compiler_info: String,
    result: String,
    reason: &'static str,
}

fn lang_card(lang: Language) -> (bool, String) {
    match find_compiler(lang) {
        Some(info) => {
            let name = info.compiler.as_deref().unwrap_or("?");
            let ver = info.version.as_deref().unwrap_or("");
            (true, format!("{name} · {ver}"))
        }
        None => (false, "compiler not found".to_string()),
    }
}

fn build_cards() -> Vec<LangCard> {
    // ── C ───────────────────────────────────────────────────────────────────
    let (c_found, c_info) = lang_card(Language::C);
    #[cfg(has_c)]
    let c_result = unsafe {
        format!(
            "c_add(21, 21) = {}   |   c_gcd(48, 18) = {}   |   c_fibonacci(10) = {}",
            c_ffi::c_add(21, 21),
            c_ffi::c_gcd(48, 18),
            c_ffi::c_fibonacci(10)
        )
    };
    #[cfg(not(has_c))]
    let c_result = String::from("(not linked)");

    // ── C++ ─────────────────────────────────────────────────────────────────
    let (cpp_found, cpp_info) = lang_card(Language::Cpp);
    #[cfg(has_cpp)]
    let cpp_result = unsafe {
        format!(
            "cpp_factorial(10) = {}   |   cpp_is_prime(97) = {}   |   cpp_strlen(\"equilibrium\") = {}",
            cpp_ffi::cpp_factorial(10),
            cpp_ffi::cpp_is_prime(97),
            cpp_ffi::cpp_strlen(b"equilibrium\0".as_ptr() as *const std::os::raw::c_char)
        )
    };
    #[cfg(not(has_cpp))]
    let cpp_result = String::from("(not linked)");

    // ── Zig ─────────────────────────────────────────────────────────────────
    let (zig_found, zig_info) = lang_card(Language::Zig);
    #[cfg(has_zig)]
    let zig_result = unsafe {
        format!(
            "zig_square(12) = {}   |   zig_sum_1_to_n(100) = {}   |   zig_is_power_of_two(64) = {}",
            zig_square(12),
            zig_sum_1_to_n(100),
            zig_is_power_of_two(64),
        )
    };
    #[cfg(not(has_zig))]
    let zig_result = String::from("(not linked)");

    // ── V ───────────────────────────────────────────────────────────────────
    let (v_found, v_info) = lang_card(Language::V);

    // ── Rust (native) ────────────────────────────────────────────────────────
    let rust_result = format!(
        "rust_is_prime(97) = {}   |   rust_next_prime(100) = {}",
        rust_is_prime(97),
        rust_next_prime(100),
    );

    // ── D ───────────────────────────────────────────────────────────────────
    let (d_found, d_info) = lang_card(Language::D);

    // ── Nim ─────────────────────────────────────────────────────────────────
    let (nim_found, nim_info) = lang_card(Language::Nim);

    // ── Odin ────────────────────────────────────────────────────────────────
    let (odin_found, odin_info) = lang_card(Language::Odin);

    // ── Hare ────────────────────────────────────────────────────────────────
    let (hare_found, hare_info) = lang_card(Language::Hare);

    // ── C# ──────────────────────────────────────────────────────────────────
    let (cs_found, cs_info) = lang_card(Language::CSharp);

    vec![
        LangCard {
            name: "C",
            extension: ".c / .h",
            source_file: "foreign-code/c_module.c",
            fn_sig: "int c_add(int, int)  |  int c_gcd(int, int)  |  long c_fibonacci(int)",
            equilibrium_lang: Language::C,
            linked: cfg!(has_c),
            compiler_found: c_found,
            compiler_info: c_info,
            result: c_result,
            reason: "C compiler not found — install clang or gcc",
        },
        LangCard {
            name: "C++",
            extension: ".cpp / .hpp",
            source_file: "foreign-code/cpp_module.cpp",
            fn_sig:
                "long long cpp_factorial(int)  |  int cpp_is_prime(int)  |  int cpp_strlen(char*)",
            equilibrium_lang: Language::Cpp,
            linked: cfg!(has_cpp),
            compiler_found: cpp_found,
            compiler_info: cpp_info,
            result: cpp_result,
            reason: "C++ compiler not found — install clang++ or g++",
        },
        LangCard {
            name: "Zig",
            extension: ".zig",
            source_file: "foreign-code/zig_module.zig",
            fn_sig:
                "zig_square(i32) i32  |  zig_sum_1_to_n(i64) i64  |  zig_is_power_of_two(u64) bool",
            equilibrium_lang: Language::Zig,
            linked: cfg!(has_zig),
            compiler_found: zig_found,
            compiler_info: zig_info,
            result: zig_result,
            reason: "zig not found — install from ziglang.org",
        },
        LangCard {
            name: "V (Vlang)",
            extension: ".v",
            source_file: "foreign-code/v_module.v",
            fn_sig: "celsius_to_fahrenheit(f64) f64  |  km_to_miles(f64) f64",
            equilibrium_lang: Language::V,
            linked: false,
            compiler_found: v_found,
            compiler_info: v_info,
            result: String::new(),
            reason: "v compiler not found — install from vlang.io",
        },
        LangCard {
            name: "Rust",
            extension: ".rs",
            source_file: "foreign-code/rust_module.rs",
            fn_sig: "rust_is_prime(u64) bool  |  rust_next_prime(u64) u64",
            equilibrium_lang: Language::Rust,
            linked: true,
            compiler_found: true,
            compiler_info: format!(
                "rustc · {}",
                find_compiler(Language::Rust)
                    .and_then(|i| i.version)
                    .unwrap_or_default()
            ),
            result: rust_result,
            reason: "",
        },
        LangCard {
            name: "D",
            extension: ".d / .di",
            source_file: "foreign-code/d_module.d",
            fn_sig: "d_abs(int) int  |  d_clamp(int, int, int) int  |  d_triangular(int) long",
            equilibrium_lang: Language::D,
            linked: false,
            compiler_found: d_found,
            compiler_info: d_info,
            result: String::new(),
            reason: "D compiler not found — install ldc2, dmd, or gdc",
        },
        LangCard {
            name: "Nim",
            extension: ".nim",
            source_file: "foreign-code/nim_module.nim",
            fn_sig: "nim_popcount(uint32) int32  |  nim_reverse_bits(uint32) uint32",
            equilibrium_lang: Language::Nim,
            linked: false,
            compiler_found: nim_found,
            compiler_info: nim_info,
            result: String::new(),
            reason: "nim not found — install from nim-lang.org",
        },
        LangCard {
            name: "Odin",
            extension: ".odin",
            source_file: "foreign-code/odin_module.odin",
            fn_sig: "odin_max(i32, i32) i32  |  odin_min(i32, i32) i32  |  odin_abs(i32) i32",
            equilibrium_lang: Language::Odin,
            linked: false,
            compiler_found: odin_found,
            compiler_info: odin_info,
            result: String::new(),
            reason: "odin not found — install from odin-lang.org",
        },
        LangCard {
            name: "Hare",
            extension: ".ha",
            source_file: "foreign-code/hare_module.ha",
            fn_sig: "hare_sign(i32) i32  |  hare_div_safe(i32, i32) i32",
            equilibrium_lang: Language::Hare,
            linked: false,
            compiler_found: hare_found,
            compiler_info: hare_info,
            result: String::new(),
            reason: "hare not found — install from harelang.org",
        },
        LangCard {
            name: "C#",
            extension: ".cs",
            source_file: "foreign-code/csharp_module.cs",
            fn_sig: "cs_circle_area_x100(int) int  |  cs_hypotenuse_x100(int, int) int",
            equilibrium_lang: Language::CSharp,
            linked: false,
            compiler_found: cs_found,
            compiler_info: cs_info,
            result: String::new(),
            reason: "C# compiler not found — install dotnet SDK",
        },
    ]
}

fn main() {
    let cards = build_cards();

    let linked_count = cards.iter().filter(|c| c.linked).count();
    let found_count = cards.iter().filter(|c| c.compiler_found).count();

    // ── Build crepuscularity template context ─────────────────────────────
    let mut ctx = TemplateContext::new();
    ctx.set("total_languages", TemplateValue::Int(cards.len() as i64));
    ctx.set("compilers_found", TemplateValue::Int(found_count as i64));
    ctx.set("modules_linked", TemplateValue::Int(linked_count as i64));

    let lang_list: Vec<TemplateContext> = cards
        .iter()
        .map(|card| {
            let mut lctx = TemplateContext::new();
            lctx.set("name", TemplateValue::Str(card.name.to_string()));
            lctx.set("extension", TemplateValue::Str(card.extension.to_string()));
            lctx.set(
                "source_file",
                TemplateValue::Str(card.source_file.to_string()),
            );
            lctx.set("fn_sig", TemplateValue::Str(card.fn_sig.to_string()));
            lctx.set("linked", TemplateValue::Bool(card.linked));
            lctx.set("compiler_found", TemplateValue::Bool(card.compiler_found));
            lctx.set(
                "compiler_info",
                TemplateValue::Str(card.compiler_info.clone()),
            );
            lctx.set("result", TemplateValue::Str(card.result.clone()));
            lctx.set("reason", TemplateValue::Str(card.reason.to_string()));
            lctx
        })
        .collect();

    ctx.set("languages", TemplateValue::List(lang_list));

    // ── Render template ───────────────────────────────────────────────────
    let template_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/dashboard.crepus");
    let template_src = std::fs::read_to_string(&template_path)
        .unwrap_or_else(|e| panic!("failed to read template: {e}"));

    let html_body = render_template_to_html(&template_src, &ctx)
        .unwrap_or_else(|e| format!("<pre>template error: {e}</pre>"));

    let full_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Equilibrium · Polyglot Dashboard</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <style>
    body {{ background: #09090b; color: #fafafa; font-family: 'JetBrains Mono', 'Fira Code', monospace; }}
  </style>
</head>
<body>
{html_body}
</body>
</html>"#
    );

    // ── Write output ───────────────────────────────────────────────────────
    let out_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("dashboard.html");
    std::fs::write(&out_path, &full_html).expect("failed to write dashboard.html");

    println!("Dashboard written to: {}", out_path.display());
    println!(
        "Languages: {} total  |  {} compilers found  |  {} modules linked",
        cards.len(),
        found_count,
        linked_count
    );

    // Print live call results for linked modules
    println!("\n── Live FFI results ──────────────────────────────────────────");
    for card in &cards {
        if card.linked && !card.result.is_empty() {
            println!("[{}]  {}", card.name, card.result);
        } else if card.compiler_found {
            println!(
                "[{}]  compiler found but not linked in this build",
                card.name
            );
        } else {
            println!("[{}]  {}", card.name, card.reason);
        }
    }

    println!("\nOpen dashboard.html in your browser to view the GUI.");

    // Try to open in browser automatically (best-effort)
    let _ = std::process::Command::new("xdg-open")
        .arg(out_path.to_str().unwrap())
        .spawn();
}
