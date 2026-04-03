use std::collections::HashMap;

use colored::Colorize;

use crate::rules::{Category, JunkItem, Risk};
use crate::util::path::shorten_path;
use crate::util::size::format_size;

/// Print scan results as a colored table to stdout, grouping multi-item rules.
pub fn print_table(items: &[JunkItem]) {
    if items.is_empty() {
        println!("{}", "No junk found!".green().bold());
        return;
    }

    // Group by category, then by rule_id.
    let order = [
        Category::DevCache,
        Category::ProjectArtifact,
        Category::SystemJunk,
        Category::AppCache,
        Category::External,
    ];

    let mut by_category: HashMap<Category, Vec<&JunkItem>> = HashMap::new();
    for item in items {
        by_category.entry(item.category).or_default().push(item);
    }

    // Header.
    println!(
        "{:<22} {:<26} {:>10}  {:<40} {}",
        "Category".bold(),
        "Name".bold(),
        "Size".bold(),
        "Path".bold(),
        "Action".bold(),
    );
    println!("{}", "-".repeat(112));

    let mut total_size: u64 = 0;
    let mut total_safe_size: u64 = 0;
    let mut first_category = true;

    for cat in &order {
        let Some(cat_items) = by_category.get(cat) else {
            continue;
        };

        if !first_category {
            println!();
        }
        first_category = false;

        // Group items by rule_id.
        let mut rule_groups: HashMap<&str, Vec<&&JunkItem>> = HashMap::new();
        for item in cat_items {
            rule_groups.entry(&item.rule_id).or_default().push(item);
        }

        // Sort groups by total size descending.
        let mut groups: Vec<(&str, Vec<&&JunkItem>)> = rule_groups.into_iter().collect();
        groups.sort_by(|a, b| {
            let size_a: u64 = a.1.iter().map(|i| i.size).sum();
            let size_b: u64 = b.1.iter().map(|i| i.size).sum();
            size_b.cmp(&size_a)
        });

        for (_rule_id, mut group_items) in groups {
            group_items.sort_by(|a, b| b.size.cmp(&a.size));

            if group_items.len() == 1 {
                // Single item — print directly.
                let item = group_items[0];
                print_item_line(item, cat);
                total_size += item.size;
                if item.risk == Risk::Safe {
                    total_safe_size += item.size;
                }
            } else {
                // Multi-item group — print summary line + top items.
                let group_size: u64 = group_items.iter().map(|i| i.size).sum();
                let count = group_items.len();
                let rule_name = &group_items[0].rule_name;
                let risk_marker = match group_items[0].risk {
                    Risk::Safe => " ".normal(),
                    Risk::Caution => "!".yellow(),
                    Risk::Dangerous => "X".red(),
                };

                println!(
                    "{}{:<22} {:<26} {:>10}  {}",
                    risk_marker,
                    format!("{cat}").dimmed(),
                    format!("{rule_name} ({count} items)").bold(),
                    format_size(group_size).cyan(),
                    "(grouped)".dimmed(),
                );

                // Show top 3 largest items.
                let show_count = 3.min(group_items.len());
                for item in &group_items[..show_count] {
                    println!(
                        " {:<22} {:<26} {:>10}  {}",
                        "",
                        format!("  {}", item.rule_name).dimmed(),
                        format_size(item.size).dimmed(),
                        shorten_path(&item.path).dimmed(),
                    );
                }
                if count > show_count {
                    println!(
                        " {:<22} {}",
                        "",
                        format!("  ... and {} more", count - show_count).dimmed(),
                    );
                }

                for item in &group_items {
                    total_size += item.size;
                    if item.risk == Risk::Safe {
                        total_safe_size += item.size;
                    }
                }
            }
        }
    }

    println!("{}", "-".repeat(112));
    println!(
        "{}  ({} safely reclaimable)",
        format!("Total: {}", format_size(total_size)).bold(),
        format_size(total_safe_size).green(),
    );
    println!(
        "{}",
        "\nActions: 'delete' = remove files/dirs  |  '$ ...' = run native cleanup command"
            .dimmed()
    );
}

fn print_item_line(item: &JunkItem, cat: &Category) {
    let risk_marker = match item.risk {
        Risk::Safe => " ".normal(),
        Risk::Caution => "!".yellow(),
        Risk::Dangerous => "X".red(),
    };
    let action = if let Some(cmd) = &item.clean_command {
        format!("$ {}", cmd.join(" "))
    } else {
        "delete".into()
    };

    println!(
        "{}{:<22} {:<26} {:>10}  {:<40} {}",
        risk_marker,
        format!("{cat}").dimmed(),
        item.rule_name,
        format_size(item.size).cyan(),
        shorten_path(&item.path).dimmed(),
        action.dimmed(),
    );
}
