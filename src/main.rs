mod components;
mod models;
mod services;
mod utils;

use dioxus::prelude::*;

use components::exercise_card::ExerciseCard;
use services::exercise_loader::load_exercises;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let exercises = load_exercises().expect("Could not load exercises");

    rsx! {

        div {
            style: "
                width:100vw;
                height:100vh;
                background:#111;
                display:flex;
                flex-direction:column;
                align-items:center;
                justify-content:center;
            ",

            h1 {
                "Interlude"
            }


            for exercise in exercises {
                ExerciseCard {
                    exercise
                }
            }
        }
    }
}
