use serde::Serialize;

use crate::rules::JunkItem;
use crate::util::path::shorten_path;

#[derive(Serialize)]
struct JsonItem {
    rule_id: String,
    rule_name: String,
    category: String,
    risk: String,
    path: String,
    size: u64,
}

#[derive(Serialize)]
struct JsonReport {
    items: Vec<JsonItem>,
    total_size: u64,
    total_items: usize,
}

/// Print scan results as JSON to stdout.
pub fn print_json(items: &[JunkItem]) {
    let json_items: Vec<JsonItem> = items
        .iter()
        .map(|item| JsonItem {
            rule_id: item.rule_id.to_string(),
            rule_name: item.rule_name.to_string(),
            category: format!("{}", item.category),
            risk: format!("{}", item.risk),
            path: shorten_path(&item.path),
            size: item.size,
        })
        .collect();

    let total_size: u64 = items.iter().map(|i| i.size).sum();
    let report = JsonReport {
        total_items: json_items.len(),
        total_size,
        items: json_items,
    };

    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
