use include_dir::{Dir, include_dir};
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        eprintln!("Invalid arguments.\nUsage: nexus-code-block");
        std::process::exit(1);
    }

    let script = ASSETS
        .get_file("set-html-clipboard.swift")
        .expect("Script not found")
        .contents_utf8()
        .expect("Invalid UTF-8");

    // Write to a temp file
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

    let code_block = format!(
        "<u>{file}</u><br>[language-rust]<br>[data-start=\"{line}\"]<br>[data-line=\"\"]<pre><code>{code}</code></pre>"
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
