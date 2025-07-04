use include_dir::{Dir, include_dir};
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Language name constants
pub const MARKUP: &str = "markup";
pub const CSS: &str = "css";
pub const CLIKE: &str = "clike";
pub const JAVASCRIPT: &str = "javascript";
pub const RUST: &str = "rust";
pub const GO: &str = "go";
pub const SOLIDITY: &str = "solidity";
pub const HASKELL: &str = "haskell";

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        eprintln!("Invalid arguments.\nUsage: nexus-code-block");
        std::process::exit(1);
    }

    // pbcopy does not support text/html so we must use a swift script as workarround
    // load the script from assets
    let script = ASSETS
        .get_file("set-html-clipboard.swift")
        .expect("Script not found")
        .contents_utf8()
        .expect("Invalid UTF-8");

    // Write the swift script to a temp file
    let tmp_path = std::env::temp_dir().join("set-html-clipboard.swift");
    std::fs::write(&tmp_path, script).expect("Failed to write Swift script");

    let key = "ZED_RELATIVE_FILE";
    let file = match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Couldn't read {}: {}", key, e);
            std::process::exit(1);
        }
    };

    let key = "ZED_ROW";
    let line = match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Couldn't read {}: {}", key, e);
            std::process::exit(1);
        }
    };

    let key = "ZED_SELECTED_TEXT";
    let code = match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Couldn't read {}: {}", key, e);
            std::process::exit(1);
        }
    };
    let code = code.replace("\r\n", "\n").replace('\r', "\n");

    let language = match detect_language(&file) {
        Some(lang) => format!("[language-{}]", lang),
        None => format!("[language-clike]"),
    };

    let code_block = format!(
        "<u>{file}</u><br>{language}<br>[data-start=\"{line}\"]<br>[data-line=\"\"]<pre><code>{code}</code></pre>"
    );

    let mut child = Command::new("swift")
        .arg(&tmp_path)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run Swift clipboard script");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(code_block.as_bytes())
            .expect("Failed to write HTML to Swift script");
    }

    let status = child.wait().expect("Failed to wait on Swift script");
    if !status.success() {
        eprintln!("Clipboard script failed");
    }

    println!("Copied to clipboard!");
}

/// Detects the programming language from a file path or extension.
/// Returns `Some(language)` if known, `None` otherwise.
pub fn detect_language<P: AsRef<Path>>(path: P) -> Option<&'static str> {
    let ext_map: HashMap<&str, &str> = HashMap::from([
        // Markup
        ("html", MARKUP),
        ("htm", MARKUP),
        ("xml", MARKUP),
        ("xhtml", MARKUP),
        ("svg", MARKUP),
        ("mathml", MARKUP),
        ("ssml", MARKUP),
        ("atom", MARKUP),
        ("rss", MARKUP),
        // CSS
        ("css", CSS),
        // C-like
        ("c", CLIKE),
        ("h", CLIKE),
        ("cpp", CLIKE),
        ("hpp", CLIKE),
        ("cc", CLIKE),
        ("cxx", CLIKE),
        ("cs", CLIKE),
        ("java", CLIKE),
        ("m", CLIKE),
        ("mm", CLIKE),
        // JavaScript
        ("js", JAVASCRIPT),
        ("mjs", JAVASCRIPT),
        ("cjs", JAVASCRIPT),
        // Rust
        ("rs", RUST),
        // Go
        ("go", GO),
        // Solidity
        ("sol", SOLIDITY),
        // Haskell
        ("hs", HASKELL),
        ("lhs", HASKELL),
    ]);

    let ext = path
        .as_ref()
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())?;

    ext_map.get(ext.as_str()).copied()
}
