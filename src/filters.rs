use globset::{Glob, GlobSetBuilder};
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

// === Шаблоны ===
pub fn get_template(
    name: &str,
) -> Option<(
    Option<Vec<&'static str>>,
    Vec<&'static str>,
    Vec<&'static str>,
)> {
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

pub fn get_default_config() -> (
    Option<Vec<&'static str>>,
    Vec<&'static str>,
    Vec<&'static str>,
) {
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
    forbidden_dirs: &[String], // Было &[&str] — стало &[String]
    exclude_patterns: &[String],
) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for path in paths {
        let mut builder = WalkBuilder::new(path);

        // Теперь forbidden_dirs — Vec<String>, можно передать напрямую
        let forbidden_dirs: Vec<String> = forbidden_dirs.to_vec();

        builder.filter_entry(move |entry| {
            !forbidden_dirs.contains(&entry.file_name().to_string_lossy().to_string())
        });

        for result in builder.build() {
            if let Ok(entry) = result {
                if entry.file_type().map_or(false, |t| t.is_file()) {
                    if is_text_file(entry.path())
                        && !matches_exclude(entry.path(), exclude_patterns)
                    {
                        if allowed_exts.map_or(true, |exts| {
                            exts.iter().any(|e| {
                                entry.path().extension().map_or(false, |ext| {
                                    ext.eq_ignore_ascii_case(e.trim_start_matches('.'))
                                })
                            }) || entry
                                .path()
                                .file_name()
                                .map_or(false, |name| name == "Gemfile")
                        }) {
                            files.push(entry.path().to_path_buf());
                        }
                    }
                }
            }
        }
    }

    files.sort();
    files
}

fn is_text_file(path: &Path) -> bool {
    use std::fs::File;
    use std::io::Read;

    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0u8; 1024];
        if file.read(&mut buffer).is_ok() {
            if buffer.contains(&0) {
                return false;
            }
        }
    }
    true
}

fn matches_exclude(path: &Path, patterns: &[String]) -> bool {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        if let Ok(glob) = Glob::new(pattern) {
            builder.add(glob);
        }
    }
    if let Ok(set) = builder.build() {
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            return set.is_match(file_name);
        }
    }
    false
}
