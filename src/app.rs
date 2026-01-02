use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, List, ListItem, ListState},
};

struct Table {
    name: String,
    rows: Vec<String>,
}
pub struct App {
    should_exit: bool,
    tables: Vec<Table>,
    tables_state: ListState,
    rows_state: ListState,
}

impl Default for App {
    fn default() -> Self {
        // https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html
        let tables = vec![
            Table {
                name: "Table 1".to_string(),
                rows: vec![
                    "Tab1 Row 1".to_string(),
                    "Tab1 Row 2".to_string(),
                    "Tab1 Row 3".to_string(),
                ],
            },
            Table {
                name: "Table 2".to_string(),
                rows: vec![
                    "Tab2 Row 1".to_string(),
                    "Tab2 Row 2".to_string(),
                    "Tab2 Row 3".to_string(),
                ],
            },
        ];
        let mut tables_state = ListState::default();
        tables_state.select(Some(0));

        let mut rows_state = ListState::default();
        rows_state.select(Some(0));

        Self {
            should_exit: false,
            tables,
            tables_state,
            rows_state,
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| draw(frame, &mut self))?;
            if let Event::Key(key) = event::read().unwrap() {
                self.handle_key(key)?;
            };
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
        match key.code {
            KeyCode::Up => self.select_above(),
            KeyCode::Down => self.select_below(),
            KeyCode::Esc | KeyCode::Char('q') => self.should_exit = true,
            _ => todo!("handle key"),
        }

        Ok(())
    }

    fn select_above(&mut self) {
        let i = self.tables_state.selected().unwrap();
        if i > 0 {
            self.tables_state.select(Some(i - 1));
            self.rows_state.select(Some(0));
        }
    }

    fn select_below(&mut self) {
        let i = self.tables_state.selected().unwrap();
        if i < self.tables.len() {
            self.tables_state.select(Some(i + 1));
            self.rows_state.select(Some(0));
        }
    }
}

fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    draw_tables(frame, app, chunks[0]);
    draw_rows(frame, app, chunks[1]);
}

fn draw_tables(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .tables
        .iter()
        .map(|t| ListItem::new(t.name.as_str()))
        .collect();
    let list = List::new(items)
        .block(Block::bordered().title("Tables"))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut app.tables_state);
}

fn draw_rows(frame: &mut Frame, app: &mut App, area: Rect) {
    let rows = app
        .tables
        .get(app.tables_state.selected().unwrap())
        .map(|t| &t.rows)
        .unwrap();

    let items: Vec<ListItem> = rows.iter().map(|r| ListItem::new(r.as_str())).collect();

    let list = List::new(items)
        .block(Block::bordered().title("Rows"))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut app.rows_state);
}
