use dioxus::prelude::*;

use crate::models::exercise::Exercise;
use crate::utils::formatters::pretty_label;

#[component]
pub fn ExerciseCard(
    exercise: Exercise,
    exercise_finished: bool,
    on_done: EventHandler<()>,
) -> Element {
    let unit = match exercise.exercise_type {
        crate::models::exercise::ExerciseType::Reps => "reps",
        crate::models::exercise::ExerciseType::Time => "seconds",
    };

    rsx! {
        div {
            style: "
                width: 400px;
                padding: 20px;
                background: #222;
                border-radius: 12px;
                color: white;
            ",

            h1 {
                "{exercise.name}"
            }

            p {
                "{exercise.value} {unit}"
            }

            p {
                "Goals:"
            }

            ul {
                for goal in exercise.goals.iter() {
                    li {
                        "{pretty_label(goal)}"
                    }
                }
            }

            button {
                disabled: !exercise_finished,

                style: if exercise_finished {
                    "
                    padding:10px 20px;
                    cursor:pointer;
                    "
                } else {
                    "
                    padding:10px 20px;
                    opacity:0.4;
                    cursor:not-allowed;
                    "
                },

                onclick: move |_| on_done.call(()),

                "Complete"
            }
        }
    }
}
