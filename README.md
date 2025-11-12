# pj-rs

**pj** (project journal) is a command-line utility for **dumping project context**.  
It scans files, filters them by extensions, excludes irrelevant ones (e.g., `bin/`, `node_modules/`), and outputs results in different formats.

---

## 📁 Example Project Structure

Let's assume we have the following project structure:

```
example_project/
├── Gemfile
├── Rakefile
├── app/
│   ├── controllers/
│   │   └── application_controller.rb
│   ├── models/
│   │   └── user.rb
│   └── views/
│       └── index.erb
├── config/
│   ├── application.rb
│   └── routes.rb
├── public/
│   └── favicon.ico
├── tmp/
│   └── cache.log
├── vendor/
│   └── bundle/
└── README.md
```

---

## 📌 Examples

### Show project tree with Rails template

```bash
pj --tree --template rails
```

**Output:**

```
Source files included in dump (tree view with metadata):
========================================================
├── Gemfile.............................................. (1 lines, 0.1 KB)
├── Rakefile............................................. (10 lines, 0.4 KB)
├── app/controllers/application_controller.rb............ (15 lines, 0.8 KB)
├── app/models/user.rb................................... (8 lines, 0.3 KB)
├── app/views/index.erb.................................. (5 lines, 0.2 KB)
├── config/application.rb................................ (20 lines, 1.2 KB)
└── config/routes.rb..................................... (12 lines, 0.6 KB)
```

### List files by size (largest first)

```bash
pj --list --template rails
```

**Output:**

```
      1.2 KB    config/application.rb
      0.8 KB    app/controllers/application_controller.rb
      0.6 KB    config/routes.rb
      0.4 KB    Rakefile
      0.3 KB    app/models/user.rb
      0.2 KB    app/views/index.erb
      0.1 KB    Gemfile
```

### Exclude temporary files

```bash
pj --tree --template rails --exclude "*.log"
```

### Use default filters (no template)

```bash
pj --tree
```

---

## 📄 Default Behavior

When called without flags, `pj` prints the **full content** of all filtered source files (after applying default filters like ignoring `node_modules`, `tmp`, etc.):

```bash
pj --template rails
```

This would print the content of `Gemfile`, `Rakefile`, `application_controller.rb`, etc.

---

## 🚀 Installation

```bash
git clone https://github.com/ilmir/pj-rs
cd pj-rs
cargo build --release
./target/release/pj --help
```

---

## 🧱 Architecture

```mermaid
graph TD
    subgraph "pj-rs library"
        A[pj_rs::filters]
        B[pj_rs::output]
    end

    subgraph "pj binary"
        C[main.rs]
    end

    C --> A
    C --> B

    A --> D[ignore::WalkBuilder]
    A --> E[globset::GlobSetBuilder]
    B --> F[std::fs]

    style subgraph fill:#eef
```

