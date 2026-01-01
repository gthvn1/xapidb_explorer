use xapidb_explorer::xapidb::parser::DbNode;

use crossterm::event::{self, Event, KeyCode};

use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, List, ListItem, ListState},
};

fn main() {
    let root = DbNode::read_xml();
    root.print_tree();

    // https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html
    let items = ["Item 1", "Item 2", "Item 3"];
    let mut state = ListState::default();
    state.select(Some(0));

    let mut terminal = ratatui::init();

    loop {
        terminal
            .draw(|frame| draw(frame, &items, &mut state))
            .expect("Failed to draw frame");

        match event::read().expect("failed to read event") {
            Event::Key(key) => match key.code {
                KeyCode::Up => {
                    let i = state.selected().unwrap_or(0);
                    if i > 0 {
                        state.select(Some(i - 1));
                    }
                }
                KeyCode::Down => {
                    let i = state.selected().unwrap_or(0);
                    if i + 1 < items.len() {
                        state.select(Some(i + 1));
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => todo!("handle key"),
            },
            _ => {}
        }
    }

    ratatui::restore();
}

fn draw(frame: &mut Frame, items: &[&str], state: &mut ListState) {
    let list_items: Vec<ListItem> = items.iter().map(|&i| ListItem::new(i)).collect();
    let list = List::new(list_items)
        .block(Block::bordered().title("List"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    let area = frame.area();
    frame.render_stateful_widget(list, area, state);
}
