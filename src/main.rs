// src/main.rs
use clap::Parser;
use std::path::PathBuf;

const MAX_FILES: usize = 100;
const MAX_TOTAL_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

#[derive(Parser)]
#[command(name = "pj")]
#[command(about = "Dump project context with filters")]
struct Cli {
    /// Show tree view with file stats
    #[arg(long, short = 'T')]
    tree: bool,

    /// List files sorted by size (desc)
    #[arg(long, short = 'L')]
    list: bool,

    /// Use predefined template (cs, wpf, rb, rails)
    #[arg(long, short = 't', value_parser = ["cs", "wpf", "rb", "rails"])]
    template: Option<String>,

    /// Exclude files matching pattern
    #[arg(long, short = 'x')]
    exclude: Vec<String>,

    /// Force output even if files/size limits are exceeded
    #[arg(long, short = 'F')]
    force: bool,

    /// Paths to scan (defaults to current directory)
    paths: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let args = if cli.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        cli.paths
    };

    let (allowed_exts, forbidden_dirs, exclude_files) = match &cli.template {
        Some(name) => pj_rs::filters::get_template(name).unwrap_or_else(|| {
            eprintln!("Unknown template: {}", name);
            std::process::exit(1);
        }),
        None => pj_rs::filters::get_default_config(),
    };

    let forbidden_dirs: Vec<String> = forbidden_dirs.iter().map(|s| s.to_string()).collect();
    let mut exclude_files: Vec<String> = exclude_files.iter().map(|s| s.to_string()).collect();
    exclude_files.extend(cli.exclude);

    // --- Теперь получаем FilterStats ---
    let stats = pj_rs::filters::collect_and_filter(
        &args,
        allowed_exts.as_deref(),
        &forbidden_dirs,
        &exclude_files,
    );

    // --- Проверка лимитов по статистике ---
    if !cli.force {
        let total_size: u64 = stats
            .included_files // Используем файлы из статистики
            .iter()
            .map(|f| f.metadata().map(|m| m.len()).unwrap_or(0))
            .sum();

        if stats.included_files.len() > MAX_FILES {
            // Используем файлы из статистики
            eprintln!(
                "⚠️  Warning: Found {} files (limit: {}).",
                stats.included_files.len(), // Используем файлы из статистики
                MAX_FILES
            );
            eprintln!("Use --force to proceed anyway.");
            std::process::exit(1);
        }
        if total_size > MAX_TOTAL_SIZE {
            eprintln!(
                "⚠️  Warning: Total size is {:.2} MB (limit: 10 MB).",
                total_size as f64 / 1024.0 / 1024.0
            );
            eprintln!("Use --force to proceed anyway.");
            std::process::exit(1);
        }
    }

    if cli.tree {
        pj_rs::output::print_tree(&stats); // Передаём &stats
    } else if cli.list {
        pj_rs::output::print_list(&stats); // Передаём &stats
    } else {
        pj_rs::output::print_content(&stats.included_files); // Передаём только список файлов
    }
}
