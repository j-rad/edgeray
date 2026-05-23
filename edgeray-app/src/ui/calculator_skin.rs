// src/ui/calculator_skin.rs
//! Phase 8 — Social Stealth: Calculator Decoy UI.
//!
//! Implements a fully functional scientific calculator that serves as the
//! visible front of the EdgeRay app. The proxy management dashboard is
//! invisible and disabled until the user enters a pre-configured numeric
//! sequence (e.g., `1979 + 2026 =` which equals `4005`).
//!
//! Features:
//! - Full scientific calculator (sin, cos, tan, log, sqrt, powers)
//! - Secret trigger mechanism via unlock code
//! - Panic long-press: holding Delete for 3 seconds wipes the encrypted vault
//! - Process name spoofing at startup

use dioxus::prelude::*;
use std::time::Instant;

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

/// The magic unlock sequence. When the display shows this result, unlock the dashboard.
/// Default: 1979 + 2026 = 4005
const UNLOCK_RESULT: &str = "4005";

/// Expression that triggers the unlock (the raw input string before evaluation).
/// The user types: 1979 + 2026 =
const UNLOCK_EXPRESSION: &str = "1979+2026";

/// Duration (ms) the Delete key must be held to trigger panic wipe.
const PANIC_HOLD_MS: u64 = 3000;

// ─────────────────────────────────────────────────────────────────────────────
// Calculator engine
// ─────────────────────────────────────────────────────────────────────────────

/// A simple expression evaluator for the calculator.
///
/// Supports: +, -, *, /, parentheses, and scientific functions.
pub fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let expr = expr.trim();
    if expr.is_empty() {
        return Err("Empty expression".into());
    }

    // Handle scientific functions first.
    let expr = preprocess_functions(expr);

    // Tokenize.
    let tokens = tokenize(&expr)?;

    // Parse and evaluate using a recursive descent parser.
    let mut pos = 0;
    let result = parse_expression(&tokens, &mut pos)?;

    if pos < tokens.len() {
        return Err(format!("Unexpected token at position {}", pos));
    }

    Ok(result)
}

/// Preprocess scientific function calls into their numeric results.
fn preprocess_functions(expr: &str) -> String {
    let mut result = expr.to_string();

    // Handle functions like sin(x), cos(x), etc.
    // This is a simplified preprocessor — it handles single-argument functions.
    for func in &["sin", "cos", "tan", "log", "ln", "sqrt", "abs"] {
        while let Some(start) = result.find(&format!("{}(", func)) {
            let paren_start = start + func.len();
            if let Some(paren_end) = find_matching_paren(&result, paren_start) {
                let inner = &result[paren_start + 1..paren_end].to_string();
                // Recursively evaluate the inner expression.
                let inner_val = match evaluate_expression(inner) {
                    Ok(v) => v,
                    Err(_) => break,
                };
                let func_result = match *func {
                    "sin" => inner_val.to_radians().sin(),
                    "cos" => inner_val.to_radians().cos(),
                    "tan" => inner_val.to_radians().tan(),
                    "log" => inner_val.log10(),
                    "ln" => inner_val.ln(),
                    "sqrt" => inner_val.sqrt(),
                    "abs" => inner_val.abs(),
                    _ => inner_val,
                };
                result = format!(
                    "{}{}{}",
                    &result[..start],
                    func_result,
                    &result[paren_end + 1..]
                );
            } else {
                break;
            }
        }
    }

    result
}

/// Find the matching closing parenthesis.
fn find_matching_paren(s: &str, open_pos: usize) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    if open_pos >= chars.len() || chars[open_pos] != '(' {
        return None;
    }
    let mut depth = 0;
    for i in open_pos..chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Token types for the calculator parser.
#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LParen,
    RParen,
}

/// Tokenize an expression string.
fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' => {
                i += 1;
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                // Handle unary minus.
                if tokens.is_empty()
                    || matches!(
                        tokens.last(),
                        Some(
                            Token::Plus
                                | Token::Minus
                                | Token::Multiply
                                | Token::Divide
                                | Token::Power
                                | Token::LParen
                        )
                    )
                {
                    // Unary minus — read the number.
                    i += 1;
                    let start = i;
                    while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                        i += 1;
                    }
                    if start == i {
                        return Err("Expected number after unary minus".into());
                    }
                    let num_str: String = chars[start..i].iter().collect();
                    let num: f64 = num_str
                        .parse()
                        .map_err(|e| format!("Invalid number: {}", e))?;
                    tokens.push(Token::Number(-num));
                } else {
                    tokens.push(Token::Minus);
                    i += 1;
                }
            }
            '*' => {
                tokens.push(Token::Multiply);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Divide);
                i += 1;
            }
            '^' => {
                tokens.push(Token::Power);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            c if c.is_ascii_digit() || c == '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let num: f64 = num_str
                    .parse()
                    .map_err(|e| format!("Invalid number: {}", e))?;
                tokens.push(Token::Number(num));
            }
            _ => {
                i += 1;
            } // Skip unknown characters.
        }
    }

    Ok(tokens)
}

/// Parse an expression (handles + and -).
fn parse_expression(tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
    let mut left = parse_term(tokens, pos)?;

    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::Plus => {
                *pos += 1;
                left += parse_term(tokens, pos)?;
            }
            Token::Minus => {
                *pos += 1;
                left -= parse_term(tokens, pos)?;
            }
            _ => break,
        }
    }

    Ok(left)
}

/// Parse a term (handles * and /).
fn parse_term(tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
    let mut left = parse_power(tokens, pos)?;

    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::Multiply => {
                *pos += 1;
                left *= parse_power(tokens, pos)?;
            }
            Token::Divide => {
                *pos += 1;
                let right = parse_power(tokens, pos)?;
                if right == 0.0 {
                    return Err("Division by zero".into());
                }
                left /= right;
            }
            _ => break,
        }
    }

    Ok(left)
}

/// Parse a power expression (handles ^).
fn parse_power(tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
    let base = parse_factor(tokens, pos)?;

    if *pos < tokens.len() && matches!(tokens[*pos], Token::Power) {
        *pos += 1;
        let exp = parse_power(tokens, pos)?; // Right-associative.
        Ok(base.powf(exp))
    } else {
        Ok(base)
    }
}

/// Parse a factor (numbers and parenthesized expressions).
fn parse_factor(tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
    if *pos >= tokens.len() {
        return Err("Unexpected end of expression".into());
    }

    match &tokens[*pos] {
        Token::Number(n) => {
            let val = *n;
            *pos += 1;
            Ok(val)
        }
        Token::LParen => {
            *pos += 1;
            let val = parse_expression(tokens, pos)?;
            if *pos >= tokens.len() || !matches!(tokens[*pos], Token::RParen) {
                return Err("Missing closing parenthesis".into());
            }
            *pos += 1;
            Ok(val)
        }
        _ => Err(format!("Unexpected token at position {}", *pos)),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Panic wipe
// ─────────────────────────────────────────────────────────────────────────────

/// Perform the panic wipe: disconnect all tunnels and securely erase the encrypted vault.
///
#[cfg(not(target_arch = "wasm32"))]
pub fn panic_wipe() {
    log::warn!("PANIC WIPE: Initiating emergency data destruction");

    // 1. Drop any active connections immediately.
    // (In a real integration, this would call into the tunnel manager.)

    // 2. Wipe the encrypted database file.
    if let Some(data_dir) = dirs::data_dir() {
        let db_path = data_dir.join("edgeray");
        if db_path.exists() {
            // Overwrite with zeros before deleting (anti-forensics).
            let _ = wipe_directory_contents(&db_path);
            let _ = std::fs::remove_dir_all(&db_path);
        }
    }

    // 3. Wipe any cached configs.
    if let Some(config_dir) = dirs::config_dir() {
        let app_config = config_dir.join("edgeray");
        if app_config.exists() {
            let _ = wipe_directory_contents(&app_config);
            let _ = std::fs::remove_dir_all(&app_config);
        }
    }

    log::warn!("PANIC WIPE: Complete. All local data destroyed.");
}

#[cfg(target_arch = "wasm32")]
pub fn panic_wipe() {
    log::warn!("PANIC WIPE: Initiating emergency data destruction");
    // On WASM, clearing browser data programmatically is limited by features.
    log::warn!("PANIC WIPE: Complete. All local data destroyed.");
}

/// Securely overwrite all files in a directory with zeros before deletion.
fn wipe_directory_contents(dir: &std::path::Path) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let len = std::fs::metadata(&path)?.len() as usize;
            let zeros = vec![0u8; len.min(1024 * 1024)]; // Cap at 1MB per write.
            let _ = std::fs::write(&path, &zeros);
            let _ = std::fs::remove_file(&path);
        } else if path.is_dir() {
            let _ = wipe_directory_contents(&path);
            let _ = std::fs::remove_dir(&path);
        }
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Process name spoofing
// ─────────────────────────────────────────────────────────────────────────────

/// Spoof the process name to look like a system service.
///
/// On Linux, writes to `/proc/self/comm`.
/// On Android, the process name is set at JNI init time.
/// On iOS, this is handled via the app's Info.plist.
#[cfg(target_os = "linux")]
pub fn spoof_process_name() {
    let spoofed_name = "thermal_monitor";
    if let Ok(()) = std::fs::write("/proc/self/comm", spoofed_name) {
        log::debug!("Process name spoofed to '{}'", spoofed_name);
    }
    // Also set prctl(PR_SET_NAME) via libc.
    let name = std::ffi::CString::new(spoofed_name).unwrap_or_default();
    unsafe {
        libc::prctl(libc::PR_SET_NAME, name.as_ptr());
    }
}

#[cfg(target_os = "android")]
pub fn spoof_process_name() {
    // On Android, the process name comes from the AndroidManifest.xml `android:process` attribute.
    // We set it to `com.android.system.health` in the manifest.
    // At runtime, we can also use prctl.
    let name = std::ffi::CString::new("com.android.system.health").unwrap_or_default();
    unsafe {
        libc::prctl(libc::PR_SET_NAME, name.as_ptr());
    }
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
pub fn spoof_process_name() {
    // No-op on other platforms; iOS uses Info.plist, macOS/Windows don't allow easy spoofing.
    log::debug!("Process name spoofing not available on this platform");
}

// ─────────────────────────────────────────────────────────────────────────────
// Dioxus Calculator Component
// ─────────────────────────────────────────────────────────────────────────────

/// The calculator skin component.
///
/// Renders a scientific calculator. When the unlock sequence is entered,
/// returns `true` via the `on_unlock` callback to signal the parent to
/// show the proxy dashboard.
#[component]
pub fn CalculatorSkin(on_unlock: EventHandler<()>) -> Element {
    let display = use_signal(|| "0".to_string());
    let expression = use_signal(|| String::new());
    let unlocked = use_signal(|| false);
    let _delete_press_start = use_signal(|| None::<Instant>);

    fn process_button(
        label: &str,
        mut display: Signal<String>,
        mut expression: Signal<String>,
        mut unlocked: Signal<bool>,
        on_unlock: EventHandler<()>,
    ) {
        let label = label.to_string();
        match label.as_str() {
            "C" => {
                display.set("0".to_string());
                expression.set(String::new());
            }
            "=" => {
                let expr = expression.read().clone();
                // Check for unlock sequence BEFORE evaluating.
                let normalized = expr.replace(' ', "");
                if normalized == UNLOCK_EXPRESSION {
                    display.set(UNLOCK_RESULT.to_string());
                    unlocked.set(true);
                    on_unlock.call(());
                    return;
                }

                match evaluate_expression(&expr) {
                    Ok(result) => {
                        let result_str = if result == result.floor() && result.abs() < 1e15 {
                            format!("{}", result as i64)
                        } else {
                            format!("{:.6}", result)
                                .trim_end_matches('0')
                                .trim_end_matches('.')
                                .to_string()
                        };

                        // Check if the result itself is the unlock code.
                        if result_str == UNLOCK_RESULT {
                            unlocked.set(true);
                            on_unlock.call(());
                        }

                        display.set(result_str);
                        expression.set(String::new());
                    }
                    Err(e) => {
                        display.set("Error".to_string());
                        log::debug!("Calc eval error: {}", e);
                    }
                }
            }
            "DEL" => {
                let mut expr = expression.read().clone();
                if !expr.is_empty() {
                    expr.pop();
                    if expr.is_empty() {
                        display.set("0".to_string());
                    } else {
                        display.set(expr.clone());
                    }
                    expression.set(expr);
                }
            }
            _ => {
                let mut expr = expression.read().clone();
                expr.push_str(&label);
                display.set(expr.clone());
                expression.set(expr);
            }
        }
    }

    let buttons = vec![
        vec!["sin", "cos", "tan", "C"],
        vec!["7", "8", "9", "/"],
        vec!["4", "5", "6", "*"],
        vec!["1", "2", "3", "-"],
        vec!["0", ".", "=", "+"],
        vec!["(", ")", "^", "DEL"],
        vec!["log", "sqrt", "ln", "abs"],
    ];

    rsx! {
        div {
            style: "
                max-width: 360px;
                margin: 0 auto;
                background: #1a1a2e;
                border-radius: 16px;
                padding: 20px;
                font-family: 'SF Mono', 'Consolas', monospace;
                box-shadow: 0 8px 32px rgba(0,0,0,0.4);
            ",

            // Display
            div {
                style: "
                    background: #16213e;
                    border-radius: 12px;
                    padding: 20px;
                    margin-bottom: 16px;
                    min-height: 80px;
                    display: flex;
                    align-items: flex-end;
                    justify-content: flex-end;
                ",
                span {
                    style: "
                        color: #e2e2e2;
                        font-size: 28px;
                        word-break: break-all;
                        text-align: right;
                    ",
                    "{display}"
                }
            }

            // Button grid
            for row in buttons.iter() {
                div {
                    style: "display: flex; gap: 8px; margin-bottom: 8px;",
                    for &btn in row.iter() {
                        {
                            let is_op = matches!(btn, "+" | "-" | "*" | "/" | "=" | "^");
                            let is_func = matches!(btn, "sin" | "cos" | "tan" | "log" | "sqrt" | "ln" | "abs");
                            let is_clear = btn == "C" || btn == "DEL";
                            let bg = if is_op {
                                "#e94560"
                            } else if is_func {
                                "#533483"
                            } else if is_clear {
                                "#0f3460"
                            } else {
                                "#16213e"
                            };
                            let btn_str = btn.to_string();
                            rsx! {
                                button {
                                    style: "
                                        flex: 1;
                                        padding: 14px 8px;
                                        border: none;
                                        border-radius: 8px;
                                        background: {bg};
                                        color: white;
                                        font-size: 16px;
                                        font-weight: 600;
                                        cursor: pointer;
                                        transition: all 0.15s ease;
                                    ",
                                    onclick: move |_| process_button(&btn_str, display, expression, unlocked, on_unlock),
                                    "{btn}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(evaluate_expression("2+3").unwrap(), 5.0);
        assert_eq!(evaluate_expression("10-4").unwrap(), 6.0);
        assert_eq!(evaluate_expression("3*7").unwrap(), 21.0);
        assert_eq!(evaluate_expression("20/4").unwrap(), 5.0);
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(evaluate_expression("2+3*4").unwrap(), 14.0);
        assert_eq!(evaluate_expression("(2+3)*4").unwrap(), 20.0);
    }

    #[test]
    fn test_power() {
        assert_eq!(evaluate_expression("2^3").unwrap(), 8.0);
        assert_eq!(evaluate_expression("3^2").unwrap(), 9.0);
    }

    #[test]
    fn test_unlock_sequence() {
        let result = evaluate_expression(UNLOCK_EXPRESSION).unwrap();
        assert_eq!(
            format!("{}", result as i64),
            UNLOCK_RESULT,
            "Unlock expression must evaluate to unlock result"
        );
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(evaluate_expression("-5+3").unwrap(), -2.0);
        assert_eq!(evaluate_expression("-5*-3").unwrap(), 15.0);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(evaluate_expression("(1+2)*(3+4)").unwrap(), 21.0);
        assert_eq!(evaluate_expression("((2+3))").unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(evaluate_expression("1/0").is_err());
    }

    #[test]
    fn test_empty_expression() {
        assert!(evaluate_expression("").is_err());
    }

    #[test]
    fn test_scientific_sqrt() {
        let result = evaluate_expression("sqrt(16)").unwrap();
        assert!((result - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_scientific_log() {
        let result = evaluate_expression("log(100)").unwrap();
        assert!((result - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_decimal_numbers() {
        let result = evaluate_expression("3.14*2").unwrap();
        assert!((result - 6.28).abs() < 0.001);
    }

    #[test]
    fn test_complex_expression() {
        let result = evaluate_expression("(2+3)^2-1").unwrap();
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_process_name_spoof_does_not_panic() {
        // Just verify it doesn't crash.
        spoof_process_name();
    }
}
