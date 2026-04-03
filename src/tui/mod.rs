pub mod clean_screen;
pub mod confirm_screen;
pub mod done_screen;
pub mod results_screen;
pub mod scan_screen;
pub mod theme;
pub mod widgets;

use std::io;
use std::sync::mpsc;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use ratatui::Terminal;

use crate::rules::{Category, CleanEvent, CleanStats, JunkItem, Risk, ScanEvent, ScanStats};

/// Which screen the TUI is showing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Scanning,
    Results,
    Confirm,
    Cleaning,
    Done,
}

/// A category group in the results tree (top level).
pub struct CategoryGroup {
    pub category: Category,
    pub expanded: bool,
    pub selected: bool,
    pub groups: Vec<RuleGroup>,
}

/// A group of items sharing the same rule_id within a category (middle level).
pub struct RuleGroup {
    pub rule_id: String,
    pub rule_name: String,
    pub expanded: bool,
    pub selected: bool,
    pub items: Vec<ItemEntry>,
}

impl RuleGroup {
    /// Whether this group has multiple items and should render as a collapsible group header.
    pub fn is_multi(&self) -> bool {
        self.items.len() > 1
    }

    /// Total size of all items in this group.
    pub fn total_size(&self) -> u64 {
        self.items.iter().map(|i| i.item.size).sum()
    }

    /// Total size of selected items in this group.
    pub fn selected_size(&self) -> u64 {
        self.items.iter().filter(|i| i.selected).map(|i| i.item.size).sum()
    }

    /// Number of items in this group.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

/// An individual item in the results tree (leaf level).
pub struct ItemEntry {
    pub item: JunkItem,
    pub selected: bool,
}

/// Describes what the cursor is pointing at: category, group header, or individual item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorTarget {
    /// Cursor is on a category header.
    Category { cat_idx: usize },
    /// Cursor is on a group header (only for multi-item groups).
    Group { cat_idx: usize, group_idx: usize },
    /// Cursor is on an individual item.
    /// For single-item groups, this is the only row for that group.
    /// For multi-item groups, this is a child row within the expanded group.
    Item {
        cat_idx: usize,
        group_idx: usize,
        item_idx: usize,
    },
}

/// The main TUI application state.
pub struct App {
    pub screen: Screen,
    pub items: Vec<JunkItem>,
    pub categories: Vec<CategoryGroup>,
    pub cursor: usize,
    pub scroll_offset: usize,
    pub scan_progress: String,
    pub scan_stats: Option<ScanStats>,
    pub clean_stats: Option<CleanStats>,
    pub clean_current: Option<String>,
    pub clean_progress: (usize, usize),
    pub spinner_frame: usize,
    pub should_quit: bool,
    pub scan_rx: Option<mpsc::Receiver<ScanEvent>>,
    pub clean_rx: Option<mpsc::Receiver<CleanEvent>>,
}

impl App {
    pub fn new(scan_rx: mpsc::Receiver<ScanEvent>) -> Self {
        Self {
            screen: Screen::Scanning,
            items: Vec::new(),
            categories: Vec::new(),
            cursor: 0,
            scroll_offset: 0,
            scan_progress: "Starting scan...".into(),
            scan_stats: None,
            clean_stats: None,
            clean_current: None,
            clean_progress: (0, 0),
            spinner_frame: 0,
            should_quit: false,
            scan_rx: Some(scan_rx),
            clean_rx: None,
        }
    }

    /// Total number of visible rows in the results tree.
    pub fn visible_row_count(&self) -> usize {
        let mut count = 0;
        for cat in &self.categories {
            count += 1; // Category header row.
            if cat.expanded {
                for group in &cat.groups {
                    if group.is_multi() {
                        count += 1; // Group header row.
                        if group.expanded {
                            count += group.items.len(); // Individual item rows.
                        }
                    } else {
                        // Single-item group: renders directly as the item.
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Returns what the cursor is pointing at.
    pub fn cursor_target(&self) -> Option<CursorTarget> {
        let mut row = 0;
        for (ci, cat) in self.categories.iter().enumerate() {
            if row == self.cursor {
                return Some(CursorTarget::Category { cat_idx: ci });
            }
            row += 1;
            if cat.expanded {
                for (gi, group) in cat.groups.iter().enumerate() {
                    if group.is_multi() {
                        // Group header row.
                        if row == self.cursor {
                            return Some(CursorTarget::Group {
                                cat_idx: ci,
                                group_idx: gi,
                            });
                        }
                        row += 1;
                        if group.expanded {
                            for ii in 0..group.items.len() {
                                if row == self.cursor {
                                    return Some(CursorTarget::Item {
                                        cat_idx: ci,
                                        group_idx: gi,
                                        item_idx: ii,
                                    });
                                }
                                row += 1;
                            }
                        }
                    } else {
                        // Single-item group: renders as a single item row.
                        if row == self.cursor {
                            return Some(CursorTarget::Item {
                                cat_idx: ci,
                                group_idx: gi,
                                item_idx: 0,
                            });
                        }
                        row += 1;
                    }
                }
            }
        }
        None
    }

    /// Toggle selection of the item/group/category at cursor.
    pub fn toggle_selection(&mut self) {
        let Some(target) = self.cursor_target() else {
            return;
        };
        match target {
            CursorTarget::Category { cat_idx } => {
                let new_state = !self.categories[cat_idx].selected;
                self.categories[cat_idx].selected = new_state;
                for group in &mut self.categories[cat_idx].groups {
                    group.selected = new_state;
                    for item in &mut group.items {
                        item.selected = new_state;
                    }
                }
            }
            CursorTarget::Group {
                cat_idx,
                group_idx,
            } => {
                let group = &mut self.categories[cat_idx].groups[group_idx];
                let new_state = !group.selected;
                group.selected = new_state;
                for item in &mut group.items {
                    item.selected = new_state;
                }
                // Propagate upward: update category selected state.
                self.categories[cat_idx].selected = self.categories[cat_idx]
                    .groups
                    .iter()
                    .all(|g| g.selected);
            }
            CursorTarget::Item {
                cat_idx,
                group_idx,
                item_idx,
            } => {
                let item = &mut self.categories[cat_idx].groups[group_idx].items[item_idx];
                item.selected = !item.selected;
                // Propagate upward: update group selected state.
                let group = &self.categories[cat_idx].groups[group_idx];
                let group_all_selected = group.items.iter().all(|i| i.selected);
                self.categories[cat_idx].groups[group_idx].selected = group_all_selected;
                // Propagate upward: update category selected state.
                self.categories[cat_idx].selected = self.categories[cat_idx]
                    .groups
                    .iter()
                    .all(|g| g.selected);
            }
        }
    }

    /// Toggle expand/collapse of the category or multi-item group at cursor.
    pub fn toggle_expand(&mut self) {
        let Some(target) = self.cursor_target() else {
            return;
        };
        match target {
            CursorTarget::Category { cat_idx } => {
                self.categories[cat_idx].expanded = !self.categories[cat_idx].expanded;
            }
            CursorTarget::Group {
                cat_idx,
                group_idx,
            } => {
                let group = &mut self.categories[cat_idx].groups[group_idx];
                if group.is_multi() {
                    group.expanded = !group.expanded;
                }
            }
            CursorTarget::Item { .. } => {
                // No-op: items are leaf nodes.
            }
        }
        // Clamp cursor after collapse to avoid pointing at a non-existent row.
        let max = self.visible_row_count().saturating_sub(1);
        if self.cursor > max {
            self.cursor = max;
        }
    }

    /// Select all items.
    pub fn select_all(&mut self) {
        for cat in &mut self.categories {
            cat.selected = true;
            for group in &mut cat.groups {
                group.selected = true;
                for item in &mut group.items {
                    item.selected = true;
                }
            }
        }
    }

    /// Deselect all items.
    pub fn select_none(&mut self) {
        for cat in &mut self.categories {
            cat.selected = false;
            for group in &mut cat.groups {
                group.selected = false;
                for item in &mut group.items {
                    item.selected = false;
                }
            }
        }
    }

    /// Update scroll_offset to keep cursor visible within the given terminal height.
    pub fn update_scroll_offset(&mut self, terminal_height: usize) {
        // Approximate visible height (subtract borders/status bar).
        let visible = terminal_height.saturating_sub(5);
        if visible == 0 {
            return;
        }
        if self.cursor < self.scroll_offset {
            self.scroll_offset = self.cursor;
        } else if self.cursor >= self.scroll_offset + visible {
            self.scroll_offset = self.cursor - visible + 1;
        }
    }

    /// Get total size of selected items.
    pub fn selected_size(&self) -> u64 {
        self.categories
            .iter()
            .flat_map(|c| &c.groups)
            .flat_map(|g| &g.items)
            .filter(|i| i.selected)
            .map(|i| i.item.size)
            .sum()
    }

    /// Get count of selected items.
    pub fn selected_count(&self) -> usize {
        self.categories
            .iter()
            .flat_map(|c| &c.groups)
            .flat_map(|g| &g.items)
            .filter(|i| i.selected)
            .count()
    }

    /// Get all selected items.
    pub fn selected_items(&self) -> Vec<&JunkItem> {
        self.categories
            .iter()
            .flat_map(|c| &c.groups)
            .flat_map(|g| &g.items)
            .filter(|i| i.selected)
            .map(|i| &i.item)
            .collect()
    }

    /// Build category groups from discovered items, grouping by rule_id within each category.
    pub fn build_categories(&mut self) {
        use std::collections::BTreeMap;
        use std::collections::HashMap;

        let mut cat_map: BTreeMap<Category, Vec<JunkItem>> = BTreeMap::new();
        // Sort order for categories.
        let order = [
            Category::DevCache,
            Category::ProjectArtifact,
            Category::SystemJunk,
            Category::AppCache,
            Category::External,
        ];
        for cat in &order {
            cat_map.entry(*cat).or_default();
        }

        for item in self.items.drain(..) {
            cat_map.entry(item.category).or_default().push(item);
        }

        self.categories.clear();
        for cat in &order {
            if let Some(items) = cat_map.remove(cat) {
                if items.is_empty() {
                    continue;
                }

                // Group items by rule_id.
                let mut rule_map: HashMap<String, Vec<JunkItem>> = HashMap::new();
                for item in items {
                    rule_map
                        .entry(item.rule_id.clone())
                        .or_default()
                        .push(item);
                }

                // Build RuleGroups, sorting items within each group by size descending.
                let mut rule_groups: Vec<RuleGroup> = rule_map
                    .into_iter()
                    .map(|(rule_id, mut group_items)| {
                        group_items.sort_by(|a, b| b.size.cmp(&a.size));
                        let rule_name = group_items[0].rule_name.clone();
                        let entries: Vec<ItemEntry> = group_items
                            .into_iter()
                            .map(|item| {
                                let selected = item.risk == Risk::Safe;
                                ItemEntry { item, selected }
                            })
                            .collect();
                        let all_selected = entries.iter().all(|e| e.selected);
                        let count = entries.len();
                        RuleGroup {
                            rule_id,
                            rule_name,
                            expanded: count <= 5,
                            selected: all_selected,
                            items: entries,
                        }
                    })
                    .collect();

                // Sort groups by total size descending.
                rule_groups.sort_by_key(|g| std::cmp::Reverse(g.total_size()));

                let all_selected = rule_groups.iter().all(|g| g.selected);
                self.categories.push(CategoryGroup {
                    category: *cat,
                    expanded: true,
                    selected: all_selected,
                    groups: rule_groups,
                });
            }
        }
    }

    /// Process scan events from the channel.
    pub fn process_scan_events(&mut self) {
        let Some(rx) = &self.scan_rx else { return };
        loop {
            match rx.try_recv() {
                Ok(ScanEvent::ItemFound(item)) => {
                    self.items.push(item);
                }
                Ok(ScanEvent::Progress(msg)) => {
                    self.scan_progress = msg;
                }
                Ok(ScanEvent::Complete(stats)) => {
                    self.scan_stats = Some(stats);
                    self.build_categories();
                    self.screen = Screen::Results;
                    self.scan_rx = None;
                    return;
                }
                Ok(ScanEvent::Error(err)) => {
                    self.scan_progress = format!("Error: {err}");
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    // Scanner thread finished without sending Complete.
                    self.build_categories();
                    self.screen = Screen::Results;
                    self.scan_rx = None;
                    return;
                }
            }
        }
    }

    /// Process clean events from the channel.
    pub fn process_clean_events(&mut self) {
        let Some(rx) = &self.clean_rx else { return };
        loop {
            match rx.try_recv() {
                Ok(CleanEvent::Deleting(path)) => {
                    self.clean_current =
                        Some(crate::util::path::shorten_path(&path));
                }
                Ok(CleanEvent::Deleted { .. }) => {
                    self.clean_progress.0 += 1;
                }
                Ok(CleanEvent::Failed { path, error }) => {
                    self.clean_progress.0 += 1;
                    self.clean_current = Some(format!(
                        "Failed: {} - {}",
                        crate::util::path::shorten_path(&path),
                        error
                    ));
                }
                Ok(CleanEvent::Complete(stats)) => {
                    self.clean_stats = Some(stats);
                    self.screen = Screen::Done;
                    self.clean_rx = None;
                    return;
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.screen = Screen::Done;
                    self.clean_rx = None;
                    return;
                }
            }
        }
    }
}

/// Run the TUI application.
pub fn run(scan_rx: mpsc::Receiver<ScanEvent>) -> anyhow::Result<()> {
    // Setup terminal.
    terminal::enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new(scan_rx);
    let tick_rate = Duration::from_millis(100);

    loop {
        // Update scroll offset before drawing.
        app.update_scroll_offset(terminal.size()?.height as usize);

        // Draw.
        terminal.draw(|frame| {
            match app.screen {
                Screen::Scanning => scan_screen::draw(frame, &app),
                Screen::Results => results_screen::draw(frame, &app),
                Screen::Confirm => confirm_screen::draw(frame, &app),
                Screen::Cleaning => clean_screen::draw(frame, &app),
                Screen::Done => done_screen::draw(frame, &app),
            }
        })?;

        if app.should_quit {
            break;
        }

        // Process background events.
        match app.screen {
            Screen::Scanning => {
                app.process_scan_events();
                app.spinner_frame = app.spinner_frame.wrapping_add(1);
            }
            Screen::Cleaning => {
                app.process_clean_events();
                app.spinner_frame = app.spinner_frame.wrapping_add(1);
            }
            _ => {}
        }

        // Handle keyboard input.
        if event::poll(tick_rate)?
            && let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match app.screen {
                    Screen::Scanning => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        _ => {}
                    },
                    Screen::Results => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.cursor > 0 {
                                app.cursor -= 1;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            let max = app.visible_row_count().saturating_sub(1);
                            if app.cursor < max {
                                app.cursor += 1;
                            }
                        }
                        KeyCode::Char(' ') => app.toggle_selection(),
                        KeyCode::Tab => app.toggle_expand(),
                        KeyCode::Char('a') => app.select_all(),
                        KeyCode::Char('n') => app.select_none(),
                        KeyCode::Enter => {
                            if app.selected_count() > 0 {
                                app.screen = Screen::Confirm;
                            }
                        }
                        _ => {}
                    },
                    Screen::Confirm => match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            app.screen = Screen::Results;
                        }
                        KeyCode::Enter => {
                            // Start cleaning.
                            let items: Vec<JunkItem> = app
                                .selected_items()
                                .into_iter()
                                .cloned()
                                .collect();
                            let total = items.len();
                            app.clean_progress = (0, total);

                            let (tx, rx) = mpsc::channel();
                            app.clean_rx = Some(rx);
                            app.screen = Screen::Cleaning;

                            std::thread::spawn(move || {
                                crate::cleaner::executor::execute(items, tx);
                            });
                        }
                        _ => {}
                    },
                    Screen::Cleaning => {},
                    Screen::Done => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
                            app.should_quit = true;
                        }
                        _ => {}
                    },
                }
            }
    }

    // Restore terminal.
    terminal::disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    // Print summary after exit.
    if let Some(stats) = &app.clean_stats {
        println!(
            "Freed {} across {} items in {:.1}s",
            crate::util::size::format_size(stats.deleted_size),
            stats.deleted_count,
            stats.duration.as_secs_f64(),
        );
        if stats.failed_count > 0 {
            println!("{} items failed to delete.", stats.failed_count);
        }
    }

    Ok(())
}
