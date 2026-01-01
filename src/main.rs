use xapidb_explorer::xapidb::parser::DbNode;

use crossterm::event::{self, Event};

use ratatui::{
    Frame,
    layout::Constraint,
    style::Style,
    widgets::{Block, Row, Table, TableState},
};

fn main() {
    let root = DbNode::read_xml();
    root.print_tree();

    // ---- https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html
    let mut table_state = TableState::default();
    table_state.select(Some(0));

    let rows = [
        Row::new(vec!["Cell11", "Cell12", "Cell13"]),
        Row::new(vec!["Cell21", "Cell22", "Cell23"]),
        Row::new(vec!["Cell31", "Cell32", "Cell33"]),
    ];

    let widths = [
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(10),
    ];

    let table = Table::new(rows, widths)
        .block(Block::new().title("Table"))
        .row_highlight_style(Style::new().reversed())
        .highlight_symbol(">>");

    let mut terminal = ratatui::init();

    loop {
        terminal
            .draw(|frame| draw(frame, &table, &mut table_state))
            .expect("Failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }

    ratatui::restore();
}

fn draw(frame: &mut Frame, table: &Table, state: &mut TableState) {
    let area = frame.area();
    frame.render_stateful_widget(table, area, state);
}
