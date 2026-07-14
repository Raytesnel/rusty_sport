mod components;
mod models;
mod services;
mod utils;

use dioxus::prelude::*;

use components::exercise_card::ExerciseCard;
use models::app_state::AppState;
use services::exercise_loader::load_exercises;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let exercises = load_exercises().expect("Could not load exercises");

    let mut state = use_signal(|| AppState {
        exercises,
        current_index: 0,
    });

    let current = state().current().cloned();

    rsx! {

        div {

            style:"
                width:100vw;
                height:100vh;
                background:#111;
                display:flex;
                justify-content:center;
                align-items:center;
            ",


            if let Some(exercise) = current {

                ExerciseCard {
                    exercise,

                    on_done: move |_| {
                        state.write().next();
                    }
                }

            } else {

                "No exercises"

            }
        }
    }
}
