use std::fs;
use std::path::PathBuf;

pub fn print_content(files: &[PathBuf]) {
    if files.is_empty() {
        eprintln!("No relevant source files found.");
        return;
    }

    for file in files {
        println!("{}", "=".repeat(20));
        // 🔥 Убираем лишний &
        if let Ok(rel_path) = file.strip_prefix(std::env::current_dir().unwrap()) {
            println!("File: {}", rel_path.display());
        } else {
            println!("File: {}", file.display());
        }
        println!("{}", "-".repeat(20));
        if let Ok(content) = fs::read_to_string(file) {
            println!("{}", content);
        }
        println!();
    }
}

pub fn print_tree(files: &[PathBuf]) {
    println!("Source files included in dump (tree view with metadata):");
    println!("{}", "=".repeat(56));

    if files.is_empty() {
        println!("├── <no source files>");
        return;
    }

    let max_width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(120)
        .min(120);

    for file in files {
        // 🔥 Убираем лишний &
        let rel_path = file
            .strip_prefix(std::env::current_dir().unwrap())
            .unwrap_or(file);
        let lines = fs::read_to_string(file)
            .map(|s| s.lines().count())
            .unwrap_or(0);
        let size = format_size(file.metadata().map(|m| m.len()).unwrap_or(0));
        let meta = format!("({} lines, {})", lines, size);

        let prefix = format!("├── {}", rel_path.display());
        let available = max_width.saturating_sub(prefix.len() + meta.len() + 1);
        let dots = if available > 0 {
            ".".repeat(available)
        } else {
            ".".to_string()
        };

        println!("{}{} {}", prefix, dots, meta);
    }
}

pub fn print_list(files: &[PathBuf]) {
    if files.is_empty() {
        println!("<no source files>");
        return;
    }

    let mut files = files.to_vec();
    files.sort_by_key(|f| std::cmp::Reverse(f.metadata().map(|m| m.len()).unwrap_or(0)));

    for file in files {
        let size_str = format_size(file.metadata().map(|m| m.len()).unwrap_or(0));

        let rel_path = file
            .strip_prefix(std::env::current_dir().unwrap())
            .unwrap_or(&file);
        println!("{:>10}    {}", size_str, rel_path.display());
    }
}

fn format_size(bytes: u64) -> String {
    format!("{:.1} KB", bytes as f64 / 1024.0)
}
