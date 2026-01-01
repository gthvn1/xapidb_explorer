use xapidb_explorer::xapidb::parser::DbNode;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::collections::HashSet;
use std::io;

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, ListState},
};

// --- UI state ---
struct UiState<'a> {
    expanded: HashSet<*const DbNode>,
    selected: usize,
    visible: Vec<(usize, &'a DbNode)>,
}

impl<'a> UiState<'a> {
    fn new() -> Self {
        Self {
            expanded: HashSet::new(),
            selected: 0,
            visible: Vec::new(),
        }
    }
}

// --- Flatten tree according to expanded nodes ---
fn flatten<'a>(
    node: &'a DbNode,
    depth: usize,
    expanded: &HashSet<*const DbNode>,
    out: &mut Vec<(usize, &'a DbNode)>,
) {
    out.push((depth, node));
    if expanded.contains(&(node as *const _)) {
        for child in &node.children {
            flatten(child, depth + 1, expanded, out);
        }
    }
}

// --- Render the tree ---
fn render_tree<'a>(area: Rect, f: &mut ratatui::Frame, state: &UiState<'a>) {
    let mut items: Vec<ListItem> = Vec::new();

    for (depth, node) in &state.visible {
        let indent = "  ".repeat(*depth);

        // Determine marker for collapsible nodes
        let marker = if !node.children.is_empty() {
            "▶ "
        } else {
            "  "
        };

        if node.name == "row" {
            // First line: row with marker
            items.push(ListItem::new(format!("{}{}row", indent, marker)));

            // Then all attributes (extra indent, no marker)
            for (k, v) in &node.attributes {
                items.push(ListItem::new(format!("{}  {}: {}", indent, k, v)));
            }
        } else {
            // Table line: marker + table name
            let display_name = node
                .attributes
                .get("name")
                .cloned()
                .unwrap_or_else(|| node.name.clone());
            items.push(ListItem::new(format!(
                "{}{}{}",
                indent, marker, display_name
            )));
        }
    }

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("XAPI DB"))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut list_state);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = DbNode::read_xml();
    root.print_tree();

    let mut state = UiState::new();

    // ----- terminal setup -----
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        // Recompute visible nodes
        state.visible.clear();
        flatten(&root, 0, &state.expanded, &mut state.visible);

        terminal.draw(|f| {
            let area = f.area();
            render_tree(area, f, &state);
        })?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    if state.selected > 0 {
                        state.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if state.selected + 1 < state.visible.len() {
                        state.selected += 1;
                    }
                }
                KeyCode::Right | KeyCode::Enter => {
                    let node = state.visible[state.selected].1;
                    state.expanded.insert(node as *const _);
                }
                KeyCode::Left => {
                    let node = state.visible[state.selected].1;
                    state.expanded.remove(&(node as *const _));
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    // ----- restore terminal -----
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
