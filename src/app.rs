use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    style::Style,
    widgets::{Block, List, ListItem, ListState},
};

pub struct App {
    should_exit: bool,
    items: Vec<String>,
    state: ListState,
}

impl Default for App {
    fn default() -> Self {
        // https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html
        let items = vec![
            "Item 1".to_string(),
            "Item 2".to_string(),
            "Item 3".to_string(),
        ];
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            should_exit: false,
            items,
            state,
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| draw(frame, &self.items, &mut self.state))?;
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
        let i = self.state.selected().unwrap_or(0);
        if i > 0 {
            self.state.select(Some(i - 1));
        }
    }

    fn select_below(&mut self) {
        let i = self.state.selected().unwrap_or(0);
        if i + 1 < self.items.len() {
            self.state.select(Some(i + 1));
        }
    }
}

fn draw(frame: &mut Frame, items: &[String], state: &mut ListState) {
    let list_items: Vec<ListItem> = items.iter().map(|s| ListItem::new(s.as_str())).collect();
    let list = List::new(list_items)
        .block(Block::bordered().title("List"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    let area = frame.area();
    frame.render_stateful_widget(list, area, state);
}
