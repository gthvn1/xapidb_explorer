use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListItem, ListState},
};

enum Focus {
    Tables,
    Rows,
}

struct Table {
    name: String,
    rows: Vec<String>,
}
pub struct App {
    should_exit: bool,
    tables: Vec<Table>,
    tables_state: ListState,
    rows_state: ListState,
    focus: Focus,
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
                    "Tab2 Row 4".to_string(),
                    "Tab2 Row 5".to_string(),
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
            focus: Focus::Tables,
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read().unwrap() {
                self.handle_key(key)?;
            };
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
        match key.code {
            KeyCode::Tab => self.toggle_focus(),
            KeyCode::Up => self.select_above(),
            KeyCode::Down => self.select_below(),
            KeyCode::Esc | KeyCode::Char('q') => self.should_exit = true,
            _ => todo!("handle key"),
        }

        Ok(())
    }

    fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Tables => Focus::Rows,
            Focus::Rows => Focus::Tables,
        };
    }

    fn select_above(&mut self) {
        match self.focus {
            Focus::Tables => {
                let i = self.tables_state.selected().unwrap();
                if i > 0 {
                    self.tables_state.select(Some(i - 1));
                    self.rows_state.select(Some(0));
                }
            }
            Focus::Rows => {
                let i = self.rows_state.selected().unwrap();
                if i > 0 {
                    self.rows_state.select(Some(i - 1));
                }
            }
        }
    }

    fn select_below(&mut self) {
        match self.focus {
            Focus::Tables => {
                let i = self.tables_state.selected().unwrap();
                if i < self.tables.len() {
                    self.tables_state.select(Some(i + 1));
                    self.rows_state.select(Some(0));
                }
            }
            Focus::Rows => {
                let i = self.rows_state.selected().unwrap();
                let table = self
                    .tables
                    .get(self.tables_state.selected().unwrap())
                    .unwrap();
                if i < table.rows.len() {
                    self.rows_state.select(Some(i + 1));
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        self.draw_tables(frame, chunks[0]);
        self.draw_rows(frame, chunks[1]);
    }

    fn draw_tables(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .tables
            .iter()
            .map(|t| ListItem::new(t.name.as_str()))
            .collect();

        let block = Block::bordered()
            .title("Tables")
            .border_style(match self.focus {
                Focus::Tables => Style::default().fg(Color::Yellow),
                Focus::Rows => Style::default(),
            });

        let list = List::new(items).block(block).highlight_symbol(">> ");

        frame.render_stateful_widget(list, area, &mut self.tables_state);
    }

    fn draw_rows(&mut self, frame: &mut Frame, area: Rect) {
        // We get the row according to the selected table
        let rows = self
            .tables
            .get(self.tables_state.selected().unwrap())
            .map(|t| &t.rows)
            .unwrap();

        let items: Vec<ListItem> = rows.iter().map(|r| ListItem::new(r.as_str())).collect();

        let block = Block::bordered()
            .title("Rows")
            .border_style(match self.focus {
                Focus::Rows => Style::default().fg(Color::Yellow),
                Focus::Tables => Style::default(),
            });

        let list = List::new(items).block(block).highlight_symbol(">> ");

        frame.render_stateful_widget(list, area, &mut self.rows_state);
    }
}
