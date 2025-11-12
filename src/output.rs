// src/output.rs
use crate::filters::FilterStats;
use console::Style; // <-- Убираем неиспользуемый Color
// use std::collections::HashSet; // <-- Убираем неиспользуемый импорт
use std::fs;
use std::path::PathBuf; // <-- Импортируем FilterStats

// Определим стили для удобства
const INCLUDED_STYLE: Style = Style::new().green();
const FILTERED_STYLE: Style = Style::new().red();
const COUNT_STYLE: Style = Style::new().yellow();

pub fn print_content(files: &[PathBuf]) {
    if files.is_empty() {
        eprintln!("No relevant source files found.");
        return;
    }

    for file in files {
        println!("{}", "=".repeat(20));
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

pub fn print_tree(stats: &FilterStats) {
    // <-- Теперь принимает &FilterStats
    let files = &stats.included_files; // Извлекаем файлы из статистики

    println!("Source files included in dump (tree view with metadata):");
    println!("{}", "=".repeat(56));

    if files.is_empty() {
        println!("├── <no source files>");
        print_filter_summary(stats); // Выводим summary даже если файлов нет
        return;
    }

    let max_width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(120)
        .min(120);

    for file in files {
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

        // Используем стандартный стиль для имени файла
        println!("{}{} {}", prefix, dots, meta);
    }
    // Заменяем последний ├── на └──
    if let Some(last_file) = files.last() {
        let rel_path = last_file
            .strip_prefix(std::env::current_dir().unwrap())
            .unwrap_or(last_file);
        let lines = fs::read_to_string(last_file)
            .map(|s| s.lines().count())
            .unwrap_or(0);
        let size = format_size(last_file.metadata().map(|m| m.len()).unwrap_or(0));
        let meta = format!("({} lines, {})", lines, size);

        let prefix = format!("└── {}", rel_path.display());
        let available = max_width.saturating_sub(prefix.len() + meta.len() + 1);
        let dots = if available > 0 {
            ".".repeat(available)
        } else {
            ".".to_string()
        };
        println!("{}{} {}", prefix, dots, meta);
    }

    print_filter_summary(stats); // Выводим summary после дерева
}

pub fn print_list(stats: &FilterStats) {
    // <-- Теперь принимает &FilterStats
    let files = &stats.included_files; // Извлекаем файлы из статистики

    if files.is_empty() {
        println!("<no source files>");
        print_filter_summary(stats); // Выводим summary даже если файлов нет
        return;
    }

    let mut sorted_files = files.clone();
    sorted_files.sort_by_key(|f| std::cmp::Reverse(f.metadata().map(|m| m.len()).unwrap_or(0)));

    for file in sorted_files {
        let size_str = format_size(file.metadata().map(|m| m.len()).unwrap_or(0));

        let rel_path = file
            .strip_prefix(std::env::current_dir().unwrap())
            .unwrap_or(&file);
        println!(
            "{:>10}    {}",
            COUNT_STYLE.apply_to(&size_str),
            rel_path.display()
        );
    }

    print_filter_summary(stats); // Выводим summary после списка
}

fn format_size(bytes: u64) -> String {
    format!("{:.1} KB", bytes as f64 / 1024.0)
}

// --- Новая функция для вывода статистики ---
fn print_filter_summary(stats: &FilterStats) {
    println!("\n--- Filter Summary ---");
    // Используем cloned() или owned() чтобы получить Vec<String>, а не Vec<&String>
    let mut included_sorted: Vec<String> = stats
        .included_extensions
        .iter().cloned()
        .collect();
    let mut filtered_sorted: Vec<String> = stats
        .filtered_out_extensions
        .iter().cloned()
        .collect();

    // Сортируем для более предсказуемого вывода
    included_sorted.sort();
    filtered_sorted.sort();

    // Выводим включённые расширения
    if !included_sorted.is_empty() {
        let included_str = included_sorted.join(", "); // Теперь join работает
        println!(
            "Extensions included ({}): {}",
            INCLUDED_STYLE.apply_to(included_sorted.len()),
            INCLUDED_STYLE.apply_to(&included_str)
        );
    } else {
        println!("Extensions included (0): <none>");
    }

    // Выводим отфильтрованные расширения
    if !filtered_sorted.is_empty() {
        let filtered_str = filtered_sorted.join(", "); // Теперь join работает
        println!(
            "Extensions filtered out ({}): {}",
            FILTERED_STYLE.apply_to(filtered_sorted.len()),
            FILTERED_STYLE.apply_to(&filtered_str)
        );
    } else {
        println!("Extensions filtered out (0): <none>");
    }

    // Выводим общий счётчик
    let included_count = stats.included_files.len();
    let total_count = stats.total_processed_files;
    let excluded_count = total_count - included_count;
    println!(
        "Total files processed: {} ({} included, {} excluded)",
        COUNT_STYLE.apply_to(total_count),
        INCLUDED_STYLE.apply_to(included_count),
        FILTERED_STYLE.apply_to(excluded_count)
    );
}
