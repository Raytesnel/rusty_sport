use crate::models::exercise::Exercise;

#[derive(Clone, Debug, PartialEq)]
pub enum InterludeStatus {
    Idle,
    Preparing,
    Active,
    WaitingForConfirmation,
}

#[derive(Clone)]
pub struct InterludeState {
    pub exercises: Vec<Exercise>,
    pub current_index: usize,
    pub status: InterludeStatus,
    pub exercise_finished: bool,
}

impl InterludeState {
    pub fn current(&self) -> Option<&Exercise> {
        self.exercises.get(self.current_index)
    }

    pub fn start(&mut self) {
        self.status = InterludeStatus::Preparing;
    }

    pub fn activate(&mut self) {
        self.status = InterludeStatus::Active;
        self.exercise_finished = false;
    }

    pub fn complete(&mut self) {
        self.status = InterludeStatus::WaitingForConfirmation;
    }

    pub fn finish_timer(&mut self) {
        self.exercise_finished = true;
    }

    pub fn next(&mut self) {
        if self.exercises.is_empty() {
            return;
        }

        self.current_index = (self.current_index + 1) % self.exercises.len();

        self.status = InterludeStatus::Idle;
        self.exercise_finished = false;
    }
}
