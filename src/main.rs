use clap::Parser;
use std::path::PathBuf;

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

    // Преобразуем Vec<&str> в Vec<String>
    let forbidden_dirs: Vec<String> = forbidden_dirs.iter().map(|s| s.to_string()).collect();
    let mut exclude_files: Vec<String> = exclude_files.iter().map(|s| s.to_string()).collect();
    exclude_files.extend(cli.exclude);

    let files = pj_rs::filters::collect_and_filter(
        &args,
        allowed_exts.as_deref(),
        &forbidden_dirs,
        &exclude_files,
    );

    if cli.tree {
        pj_rs::output::print_tree(&files);
    } else if cli.list {
        pj_rs::output::print_list(&files);
    } else {
        pj_rs::output::print_content(&files);
    }
}
