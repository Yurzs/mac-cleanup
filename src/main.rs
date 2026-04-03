use clap::Parser;
use colored::Colorize;

use mac_cleanup::cli::{Cli, Command, ProfileAction};
use mac_cleanup::profiles;
use mac_cleanup::rules;
use mac_cleanup::scanner;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    // Handle subcommands first.
    if let Some(Command::Profiles { action }) = &cli.command {
        return match action {
            ProfileAction::List => {
                print_profiles_list();
                Ok(())
            }
        };
    }

    // Resolve rules based on profile selection.
    let mut active_rules = resolve_rules(&cli);

    // Apply category filter on top.
    if let Some(filter) = cli.category_filter() {
        active_rules.retain(|r| filter.contains(&r.category));
    }

    if active_rules.is_empty() {
        eprintln!("No rules match the specified filters.");
        std::process::exit(1);
    }

    // Load config file (if it exists) and merge with CLI flags.
    let config = mac_cleanup::config::Config::load();

    let scan_roots = if cli.scan_roots.is_some() {
        cli.scan_roots()
    } else if !config.scan_roots.is_empty() {
        config
            .scan_roots
            .iter()
            .map(|r| mac_cleanup::util::path::expand_tilde(&r.to_string_lossy()))
            .collect()
    } else {
        cli.scan_roots()
    };

    let mut exclude_patterns = cli.exclude.clone().unwrap_or_default();
    exclude_patterns.extend(config.exclude.clone());

    if cli.no_tui || cli.json {
        run_plain(active_rules, scan_roots, &exclude_patterns, &cli)
    } else {
        let rx = scanner::start_scan(active_rules, scan_roots, exclude_patterns);
        mac_cleanup::tui::run(rx)
    }
}

/// Resolve which rules to use based on --profile / --auto-detect flags.
fn resolve_rules(cli: &Cli) -> Vec<rules::Rule> {
    if let Some(profile_names) = &cli.profile {
        // Explicit profile selection.
        let all_profiles = profiles::load_all_profiles();
        profiles::resolve_rules(&all_profiles, profile_names)
    } else if cli.auto_detect {
        // Auto-detect mode.
        let all_profiles = profiles::load_all_profiles();
        let statuses = profiles::detect_profiles(&all_profiles);
        let detected_ids: Vec<String> = statuses
            .iter()
            .filter(|s| s.detected)
            .map(|s| s.id.clone())
            .collect();
        eprintln!("Auto-detected profiles: {}", detected_ids.join(", "));
        profiles::resolve_rules(&all_profiles, &detected_ids)
    } else {
        // Default: all rules (backwards compatible).
        rules::all_rules()
    }
}

/// Print the profiles list with detection status.
fn print_profiles_list() {
    let all_profiles = profiles::load_all_profiles();
    let statuses = profiles::detect_profiles(&all_profiles);

    println!("{}", "Available profiles:\n".bold());

    for status in &statuses {
        let badge = if status.detected {
            "[DETECTED]".green().bold().to_string()
        } else {
            "          ".to_string()
        };

        let source = if status.builtin { "" } else { " (external)" };

        println!(
            "  {:<14} {badge}  {}{source}",
            status.id.bold(),
            status.description,
        );

        if !status.matched_reasons.is_empty() {
            println!(
                "  {:<14}            Matched: {}",
                "",
                status.matched_reasons.join(", "),
            );
        }
    }

    println!();
    println!("Usage:");
    println!("  mac-cleanup --profile developer,ios    Activate specific profiles");
    println!("  mac-cleanup --auto-detect              Activate only detected profiles");
}

fn run_plain(
    rules: Vec<rules::Rule>,
    scan_roots: Vec<std::path::PathBuf>,
    exclude_patterns: &[String],
    cli: &Cli,
) -> anyhow::Result<()> {
    use mac_cleanup::rules::ScanEvent;
    use std::sync::mpsc;

    eprintln!("Scanning...");

    let rx = scanner::start_scan(rules, scan_roots, exclude_patterns.to_vec());
    let mut items = Vec::new();

    for event in rx {
        match event {
            ScanEvent::ItemFound(item) => items.push(item),
            ScanEvent::Progress(_) => {}
            ScanEvent::Complete(stats) => {
                eprintln!(
                    "Scan complete: {} items found in {:.1}s",
                    stats.total_items,
                    stats.duration.as_secs_f64()
                );
            }
            ScanEvent::Error(e) => eprintln!("Warning: {e}"),
        }
    }

    items.sort_by(|a, b| b.size.cmp(&a.size));

    if cli.json {
        mac_cleanup::reporter::json::print_json(&items);
    } else {
        mac_cleanup::reporter::table::print_table(&items);
    }

    if cli.execute && !items.is_empty() {
        let safe_items: Vec<_> = items
            .into_iter()
            .filter(|i| i.risk == mac_cleanup::rules::Risk::Safe)
            .collect();

        if safe_items.is_empty() {
            eprintln!("No safe items to delete.");
            return Ok(());
        }

        let total_size: u64 = safe_items.iter().map(|i| i.size).sum();
        eprintln!(
            "\nWill delete {} safe items ({})",
            safe_items.len(),
            mac_cleanup::util::size::format_size(total_size),
        );

        if !cli.yes {
            eprint!("Proceed? [y/N] ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                eprintln!("Aborted.");
                return Ok(());
            }
        }

        let (tx, rx) = mpsc::channel();
        mac_cleanup::cleaner::executor::execute(safe_items, tx);

        for event in rx {
            match event {
                mac_cleanup::rules::CleanEvent::Deleted { path, size } => {
                    eprintln!(
                        "  Deleted {} ({})",
                        mac_cleanup::util::path::shorten_path(&path),
                        mac_cleanup::util::size::format_size(size),
                    );
                }
                mac_cleanup::rules::CleanEvent::Failed { path, error } => {
                    eprintln!(
                        "  Failed: {} - {}",
                        mac_cleanup::util::path::shorten_path(&path),
                        error,
                    );
                }
                mac_cleanup::rules::CleanEvent::Complete(stats) => {
                    eprintln!(
                        "\nFreed {} across {} items in {:.1}s",
                        mac_cleanup::util::size::format_size(stats.deleted_size),
                        stats.deleted_count,
                        stats.duration.as_secs_f64(),
                    );
                }
                _ => {}
            }
        }
    }

    Ok(())
}
