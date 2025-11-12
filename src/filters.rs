use globset::{Glob, GlobSetBuilder};
use ignore::WalkBuilder;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FilterStats {
    pub included_files: Vec<PathBuf>,
    pub included_extensions: HashSet<String>,
    pub filtered_out_extensions: HashSet<String>,
    pub total_processed_files: usize, // Общее количество файлов, которые были рассмотрены
}

pub type TemplateConfig = (
    Option<Vec<&'static str>>,
    Vec<&'static str>,
    Vec<&'static str>,
);

pub fn get_template(name: &str) -> Option<TemplateConfig> {
    match name {
        "cs" => Some((
            Some(vec![".cs", ".xaml", ".csproj", ".props", ".targets"]),
            vec!["bin", "obj", "packages", ".vs"],
            vec![
                "*.user",
                "*.suo",
                "AssemblyInfo.cs",
                "*.g.cs",
                "*.i.cs",
                "TemporaryGeneratedFile_*.cs",
            ],
        )),
        "wpf" => Some((
            Some(vec![
                ".cs", ".xaml", ".csproj", ".props", ".targets", ".config",
            ]),
            vec!["bin", "obj", ".vs"],
            vec!["*.user", "*.suo"],
        )),
        "rb" => Some((
            Some(vec![".rb", ".gemfile", "Gemfile", "Rakefile", ".rake"]),
            vec![".bundle", "tmp", "log"],
            vec!["*.gem"],
        )),
        "rails" => Some((
            Some(vec![
                ".rb", ".rake", ".erb", ".haml", ".slim", ".yml", ".yaml", ".js", ".ts", ".css",
                ".scss", ".json",
            ]),
            vec!["tmp", "log", "public/assets", ".bundle"],
            vec!["*.sqlite3", "*.log"],
        )),
        _ => None,
    }
}

pub fn get_default_config() -> TemplateConfig {
    (
        None,
        vec![
            "bin",
            "obj",
            ".git",
            ".vs",
            ".idea",
            "node_modules",
            "tmp",
            "log",
            "packages",
            ".bundle",
        ],
        vec![
            "*.tmp",
            "*.cache",
            "*.suo",
            "*.user",
            "*.xlsx",
            "*.xls",
            "*.pdf",
            "*.jpg",
            "*.jpeg",
            "*.png",
            "*.gif",
            "*.ico",
            "*.dll",
            "*.exe",
            "*.so",
            "*.dylib",
            "*.bin",
            ".DS_Store",
            "*.gem",
            "*.log",
        ],
    )
}

pub fn collect_and_filter(
    paths: &[PathBuf],
    allowed_exts: Option<&[&str]>,
    forbidden_dirs: &[String],
    exclude_patterns: &[String],
) -> FilterStats {
    let mut included_files = Vec::new();
    let mut included_extensions = HashSet::new();
    let mut filtered_out_extensions = HashSet::new();
    let mut total_processed_files = 0; // Счётчик всех файлов

    for path in paths {
        let mut builder = WalkBuilder::new(path);

        let forbidden_dirs: Vec<String> = forbidden_dirs.iter().map(|s| s.to_string()).collect();

        builder.filter_entry(move |entry| {
            !forbidden_dirs.contains(&entry.file_name().to_string_lossy().to_string())
        });

        for entry in builder.build().flatten() {
            if entry.file_type().is_some_and(|t| t.is_file()) {
                total_processed_files += 1; // Увеличиваем счётчик

                let path = entry.path();
                if !is_text_file(path) {
                    // Файл не текстовый, добавляем его расширение в отфильтрованные (или можно игнорировать)
                    // Для простоты добавим, как если бы оно не прошло фильтр
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        filtered_out_extensions.insert(ext.to_lowercase());
                    }
                    continue; // Переходим к следующему файлу
                }

                if matches_exclude(path, exclude_patterns) {
                    // Файл соответствует паттерну исключения
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        filtered_out_extensions.insert(ext.to_lowercase());
                    }
                    continue;
                }

                let allowed_by_extension = allowed_exts.is_none_or(|exts| {
                    exts.iter().any(|e| {
                        path.extension()
                            .is_some_and(|ext| ext.eq_ignore_ascii_case(e.trim_start_matches('.')))
                    }) || path.file_name().is_some_and(|name| name == "Gemfile") // Явно включаем Gemfile
                });

                if allowed_by_extension {
                    // Файл прошёл все проверки
                    included_files.push(path.to_path_buf());
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        included_extensions.insert(ext.to_lowercase());
                    }
                } else {
                    // Файл не прошёл проверку по расширению
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        filtered_out_extensions.insert(ext.to_lowercase());
                    }
                }
            }
        }
    }

    included_files.sort();
    FilterStats {
        included_files,
        included_extensions,
        filtered_out_extensions,
        total_processed_files,
    }
}

fn is_text_file(path: &Path) -> bool {
    use std::fs::File;
    use std::io::Read;

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return true, // если не удалось открыть — считаем текстовым
    };
    let mut buffer = [0u8; 1024];
    file.read(&mut buffer).is_ok() && !buffer.contains(&0)
}

fn matches_exclude(path: &Path, patterns: &[String]) -> bool {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        if let Ok(glob) = Glob::new(pattern) {
            builder.add(glob);
        }
    }
    if let Ok(set) = builder.build()
        && let Some(file_name) = path.file_name().and_then(|s| s.to_str())
    {
        return set.is_match(file_name);
    }
    false
}
