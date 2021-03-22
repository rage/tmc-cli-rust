use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub displayed: Vec<T>,
}

impl<T: Clone> Default for StatefulList<T> {
    fn default() -> Self {
        StatefulList::new()
    }
}

impl<T: Clone> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList::with_items(Vec::new())
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            displayed: items.clone(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.displayed.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.displayed.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn get_current(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
pub struct AppState {
    pub items: StatefulList<(String, usize)>,
    filter: String,
}

impl AppState {
    pub fn new(items: Vec<(String, usize)>) -> AppState {
        AppState {
            items: StatefulList::with_items(items),
            filter: String::from(""),
        }
    }

    pub fn push_filter(&mut self, c: char) {
        self.filter.push(c);
        self.refresh_filtered();
    }

    pub fn pop_filter(&mut self) {
        self.filter.pop();
        self.refresh_filtered();
    }

    pub fn get_selected(&self) -> Option<String> {
        if let Some(selected) = self.items.state.selected() {
            Some(self.items.displayed[selected].0.clone())
        } else {
            None
        }
    }

    fn refresh_filtered(&mut self) {
        self.items.displayed = self
            .items
            .items
            .iter()
            .filter(|item| item.0.to_lowercase().contains(&self.filter.to_lowercase()))
            .cloned()
            .collect();

        self.items.state = ListState::default();
        self.items.next();
    }

    /// Rotate through the event list.
    /// This only exists to simulate some kind of "progress"
    pub fn advance(&mut self) {}
}
