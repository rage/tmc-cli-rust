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

/// Struct to control the state of the interactive prompt
///
/// Example:
///
/// ```
/// let items = vec!["Eka".to_string(), "Toka".to_string(), "Kolmas".to_string()];
/// let mut app = AppState::new(items.clone());
///
/// // no item is selected at first
/// assert_eq!(None, app.get_selected());
/// app.items.next();
/// assert_eq!(items[0], app.get_selected().unwrap());
///
/// //filters out every item that doesn't contain 's'
/// app.push_filter('s');
/// assert_eq!(items[2], app.get_selected().unwrap());
/// ```
pub struct AppState {
    pub items: StatefulList<String>,
    filter: String,
}


impl AppState {
    pub fn new(items: Vec<String>) -> AppState {
        let mut items = StatefulList::with_items(items);
        items.next();
        AppState {
            items,
            filter: String::from(""),
        }
    }

    /// pushes an ASCII character to the filter
    /// refreshes the displayed items afterwards
    pub fn push_filter(&mut self, c: char) {
        self.filter.push(c);
        self.refresh_filtered();
    }

    /// removes the last character in the filter string,
    /// if it exists.
    /// refreshes the displayed items afterwards
    pub fn pop_filter(&mut self) {
        self.filter.pop();
        self.refresh_filtered();
    }

    /// returns the currently selected item wrapped in Some,
    /// or None if no item is selected
    pub fn get_selected(&self) -> Option<String> {
        if let Some(selected) = self.items.state.selected() {
            Some(self.items.displayed[selected].clone())
        } else {
            None
        }
    }

    fn refresh_filtered(&mut self) {
        self.items.displayed = self
            .items
            .items
            .iter()
            .filter(|item| item.to_lowercase().contains(&self.filter.to_lowercase()))
            .cloned()
            .collect();

        self.items.state = ListState::default();
        self.items.next();
    }
}

#[cfg(test)]
mod tests {

    use super::AppState;

    fn get_item_list() -> Vec<String> {
        vec!["eka", "toka", "kolmas"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }

    #[test]
    fn app_state_new() {
        let items = get_item_list();

        let app = AppState::new(items.clone());

        assert_eq!(app.items.items, items);
        assert_eq!(app.items.displayed, items);
        assert_eq!(app.items.state.selected(), None);
    }

    #[test]
    fn app_select_next() {
        let items = get_item_list();

        let mut app = AppState::new(items.clone());

        assert_eq!(None, app.items.get_current());
        assert_eq!(None, app.get_selected());
        app.items.next();
        assert_eq!(items[0], app.get_selected().unwrap());
        app.items.next();
        assert_eq!(items[1], app.get_selected().unwrap());
    }

    #[test]
    fn app_test_filter_push_pop() {
        let items = get_item_list();

        let mut app = AppState::new(items.clone());

        app.push_filter('s');
        assert_eq!(items[2], app.get_selected().unwrap());
        app.pop_filter();
        assert_eq!(items[0], app.get_selected().unwrap());
    }
}
