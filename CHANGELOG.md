# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.1.1] - 2025-01-28

### Added

- **Safety limits**: maximum 100 files and 10 MB total size to prevent overwhelming output.
- New flag `--force` (`-F`) to bypass safety limits when needed.

### Changed

- Default behavior now prevents accidental dump of huge projects.
- Error messages now suggest using `--force` when limits are exceeded.

---

## [v0.1.0] - 2025-01-28

### Added

- CLI with flags: `--tree`, `--list`, `--template`, `--exclude`.
- Predefined templates: `rails`, `rb`, `cs`, `wpf`.
- File filtering by extension, forbidden directories, and exclude patterns.
- Output formats: tree view, sorted list, full content dump.
- Library `pj-rs` — can be used as a dependency in other projects.
- Basic `README.md` and `CHANGELOG.md`.

### Changed

- Migrated from Ruby to Rust for performance and portability.
- Introduced modular architecture: `filters`, `output`, `cli`.

### Fixed

- Memory usage for large projects (Rust vs Ruby).
- Text file detection (binary vs text).
- Proper handling of symlinks and hidden files.

### Removed

- Dependency on `file` utility for MIME type detection (now uses Rust-only logic).

---

## [Unreleased]

### Planned

- Support for more templates: `.1C`, `.ex`, `.heex`.
- GitHub Actions for automatic releases.
- Cross-platform builds (Windows, macOS, Linux).
- Tests and benchmarks.
