use xapidb_explorer::xapidb::parser::DbNode;

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
    Attributes,
}

pub struct App {
    should_exit: bool,
    root: DbNode,
    tables_state: ListState,
    rows_state: ListState,
    attrs_state: ListState,
    focus: Focus,
}

impl App {
    pub fn new(root: DbNode) -> Self {
        // https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html
        let mut tables_state = ListState::default();
        tables_state.select(Some(0));

        let mut rows_state = ListState::default();
        rows_state.select(Some(0));

        let mut attrs_state = ListState::default();
        attrs_state.select(Some(0));

        Self {
            should_exit: false,
            root,
            tables_state,
            rows_state,
            attrs_state,
            focus: Focus::Tables,
        }
    }

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
            Focus::Rows => Focus::Attributes,
            Focus::Attributes => Focus::Tables,
        };
    }

    fn select_above(&mut self) {
        match self.focus {
            Focus::Tables => {
                let i = self.tables_state.selected().unwrap();
                if i == 0 {
                    let last = self.root.children.len();
                    self.tables_state.select(Some(last - 1));
                } else {
                    self.tables_state.select(Some(i - 1));
                }
                self.rows_state.select(Some(0));
            }
            Focus::Rows => todo!("Select row above"),
            Focus::Attributes => todo!("Select attribute above"),
        }
    }

    fn select_below(&mut self) {
        match self.focus {
            Focus::Tables => {
                let i = self.tables_state.selected().unwrap();
                if i == self.root.children.len() - 1 {
                    self.tables_state.select(Some(0));
                } else {
                    self.tables_state.select(Some(i + 1));
                }
                self.rows_state.select(Some(0));
            }
            Focus::Rows => todo!("Select row below"),
            Focus::Attributes => todo!("Select attribute below"),
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20), // table
                Constraint::Percentage(40), // row
                Constraint::Percentage(40), // attributes
            ])
            .split(area);

        self.draw_tables(frame, chunks[0]);
        self.draw_rows(frame, chunks[1]);
        self.draw_attrs(frame, chunks[2]);
    }

    fn draw_tables(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .root
            .children
            .iter()
            .map(|c| {
                let name = c.attributes.get("name").unwrap();
                ListItem::new(name.as_str())
            })
            .collect();

        let block = Block::bordered()
            .title("Tables")
            .border_style(match self.focus {
                Focus::Tables => Style::default().fg(Color::Yellow),
                Focus::Rows | Focus::Attributes => Style::default(),
            });

        let list = List::new(items).block(block).highlight_symbol(">> ");

        frame.render_stateful_widget(list, area, &mut self.tables_state);
    }

    fn draw_rows(&mut self, frame: &mut Frame, area: Rect) {
        // We get rows according to the selected table
        let rows = self
            .root
            .children
            .get(self.tables_state.selected().unwrap())
            .map(|c| &c.children)
            .unwrap();

        let items: Vec<ListItem> = rows.iter().map(|r| ListItem::new(r.get_name())).collect();

        let block = Block::bordered()
            .title("Rows")
            .border_style(match self.focus {
                Focus::Rows => Style::default().fg(Color::Yellow),
                Focus::Tables | Focus::Attributes => Style::default(),
            });

        let list = List::new(items).block(block).highlight_symbol(">> ");

        frame.render_stateful_widget(list, area, &mut self.rows_state);
    }

    fn draw_attrs(&mut self, frame: &mut Frame, area: Rect) {
        // We get rows according to the selected table
        let table_index = match self.tables_state.selected() {
            Some(i) => i,
            None => return,
        };

        let rows = match self.root.children.get(table_index) {
            Some(t) => &t.children,
            None => return,
        };

        let row_index = match self.rows_state.selected() {
            Some(i) => i,
            None => return,
        };

        let row = match rows.get(row_index) {
            Some(r) => r,
            None => return,
        };

        let items: Vec<ListItem> = row
            .attributes
            .iter()
            .map(|(key, value)| ListItem::new(format!("{key}:{value}")))
            .collect();

        let block = Block::bordered()
            .title("Attributes")
            .border_style(match self.focus {
                Focus::Attributes => Style::default().fg(Color::Yellow),
                Focus::Tables | Focus::Rows => Style::default(),
            });

        let list = List::new(items).block(block).highlight_symbol(">> ");
        frame.render_stateful_widget(list, area, &mut self.attrs_state);
    }
}
