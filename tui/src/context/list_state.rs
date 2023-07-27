use ratatui::widgets::ListState;

#[derive(Clone, Debug)]
pub struct StatefulList<T> {
    items: Vec<T>,
    state: ListState,
}

impl<T> StatefulList<T>
where
    T: Clone,
{
    pub fn new(items: Vec<T>) -> Self {
        let mut list_state = Self {
            items,
            state: ListState::default(),
        };

        list_state.next();

        list_state
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

    pub fn get_selected_entry(&self) -> Option<&T> {
        if let Some(index) = self.state.selected() {
            return self.items.get(index);
        }

        None
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn items(&self) -> &Vec<T> {
        &self.items
    }
}
