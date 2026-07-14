use crate::models::exercise::Exercise;

#[derive(Clone)]
pub struct AppState {
    pub exercises: Vec<Exercise>,
    pub current_index: usize,
}

impl AppState {
    pub fn current(&self) -> Option<&Exercise> {
        self.exercises.get(self.current_index)
    }

    pub fn next(&mut self) {
        if self.exercises.is_empty() {
            return;
        }

        self.current_index = (self.current_index + 1) % self.exercises.len();
    }
}
