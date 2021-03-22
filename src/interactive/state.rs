use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> Default for StatefulList<T> {
    fn default() -> Self {
        StatefulList::new()
    }
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
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
                    self.items.len() - 1
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
}

impl AppState {
    pub fn new(items: Vec<(String, usize)>) -> AppState {
        AppState {
            items: StatefulList::with_items(items),
        }
    }

    /// Rotate through the event list.
    /// This only exists to simulate some kind of "progress"
    pub fn advance(&mut self) {}
}
