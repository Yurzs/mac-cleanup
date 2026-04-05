# mac-cleanup

A fast, interactive macOS disk cleanup tool for developers. Finds and removes build artifacts, package caches, and system junk that tools like CleanMyMac miss.

Built in Rust with a TUI interface.

## What it finds

**Developer Caches** — pip, uv, Go, Cargo, npm, Yarn, pnpm, CocoaPods, Gradle, Maven, Homebrew

**Project Artifacts** — `node_modules`, Rust `target/`, Python `.venv/`, `__pycache__`, `.tox/`, Gradle/Maven `build/`, CMake build dirs, Ruby `.bundle/`

**System Junk** — Xcode DerivedData, archives, user logs, Trash

**App Caches** — Chrome, Firefox, Safari, Spotify, Slack, Discord

**External Tools** — Docker system prune, unused iOS Simulators

## Installation

### Homebrew (recommended)

```sh
brew tap Yurzs/tap
brew install mac-cleanup
```

### From crates.io

```sh
cargo install mac-cleanup
```

### From source

```sh
git clone https://github.com/Yurzs/mac-cleanup.git
cd mac-cleanup
cargo install --path .
```

Building from source requires Rust 2024 edition (1.85+).

## Usage

### Interactive TUI (default)

```sh
mac-cleanup
```

Launches a full-screen terminal UI. Scan results appear in real-time, grouped by category and rule. Select items with checkboxes, review the cleanup plan, then confirm.

**TUI Controls:**

| Key | Action |
|-----|--------|
| `j`/`k` or `Up`/`Down` | Navigate |
| `Space` | Toggle item/group/category selection |
| `Tab` | Expand/collapse group or category |
| `a` | Select all |
| `n` | Select none |
| `Enter` | Proceed to cleanup |
| `q` / `Esc` | Quit or go back |

Items are grouped by rule — 1,800+ `__pycache__` dirs collapse into a single line. Safe items are pre-selected; caution items (like Xcode Archives) are not.

### CLI mode

```sh
# Scan and print a table
mac-cleanup --no-tui

# JSON output for scripting
mac-cleanup --no-tui --json

# Filter by category
mac-cleanup --no-tui --category dev-cache
mac-cleanup --no-tui --category project-artifact,system-junk

# Scan and delete safe items
mac-cleanup --no-tui --execute

# Skip confirmation prompt
mac-cleanup --no-tui --execute --yes

# Exclude paths
mac-cleanup --no-tui --exclude cargo,gradle
```

### Profiles

Rules are organized into profiles that auto-detect based on your installed tools.

```sh
# See which profiles are detected on your system
mac-cleanup profiles list

# Use only specific profiles
mac-cleanup --profile developer,ios

# Auto-detect — only activate profiles for tools you have installed
mac-cleanup --auto-detect
```

**Built-in profiles:**

| Profile | Auto-detects | Includes |
|---------|-------------|----------|
| `general` | Always active | System logs, Trash, browser/app caches |
| `developer` | node, cargo, go, python3, ruby | Package caches, build artifacts |
| `ios` | xcrun, Xcode directory | Xcode DerivedData, CocoaPods, simulators |
| `android` | gradle, .gradle directory | Gradle/Maven caches and build output |
| `devops` | docker | Docker system prune |

Without `--profile` or `--auto-detect`, all rules are active (backwards compatible).

### Custom profiles

Create TOML files in `~/.config/mac-cleanup/profiles/`:

```toml
# ~/.config/mac-cleanup/profiles/data-science.toml
[profile]
name = "Data Science"
description = "Conda and Jupyter caches"

[[profile.detect]]
command_on_path = "conda"

[[profile.detect]]
path_exists = "~/miniconda3"

[[rules]]
id = "conda-pkgs"
name = "Conda packages"
category = "DevCache"
risk = "Safe"
description = "Conda package cache"
clean_command = ["conda", "clean", "--all", "--yes"]

[rules.kind.known_path]
paths = ["~/miniconda3/pkgs", "~/anaconda3/pkgs"]
```

External profiles are validated at load time — paths must be under `~/`, commands must be on an allowlist of known package managers, and shell metacharacters are blocked.

## Configuration

Optional config file at `~/.config/mac-cleanup/config.toml`:

```toml
scan_roots = ["~/Projects", "~/work"]
exclude = ["~/Projects/important-project", "**/vendor"]
```

CLI flags override config file values. `--exclude` patterns support `*` and `**` wildcards.

## Safety

- **Dry-run by default** — the TUI shows what will be deleted before you confirm. CLI mode requires `--execute` to delete anything.
- **Native cleanup commands** — rules for Go, npm, Homebrew, etc. use the tool's own cleanup command (`go clean -cache`, `brew cleanup`) instead of raw deletion.
- **Sibling-file confirmation** — a `target/` directory is only flagged as Rust build artifacts if `Cargo.toml` exists alongside it. Same for `node_modules` + `package.json`, etc.
- **Risk levels** — items are tagged Safe, Caution, or Dangerous. Caution items are not pre-selected. Dangerous items show a red warning on the confirm screen.
- **Audit log** — every deletion is logged to `~/.local/state/mac-cleanup/cleanup.log`.
- **Profile guardrails** — external TOML profiles can't target system paths, protected directories (`~/Documents`, `~/.ssh`), or run arbitrary commands.

## How it works

1. **Known-path scan** — checks fixed locations (`~/Library/Caches/pip`, `~/.gradle/caches`, etc.) and computes disk usage
2. **Parallel filesystem walk** — single-pass walk of your home directory using [jwalk](https://crates.io/crates/jwalk), matching all project-scan rules simultaneously. Skips descent into matched targets (`node_modules`, `target/`) for speed.
3. **External tool detection** — runs `docker system df`, `xcrun simctl list`, etc. with graceful handling when services aren't running
4. **Size calculation** — uses actual disk blocks (`st_blocks * 512`), matching `du` output. Accounts for APFS compression and clones.

## License

MIT
