mod components;
mod models;
mod services;
mod utils;

use dioxus::prelude::*;

use components::countdown::Countdown;
use components::exercise_card::ExerciseCard;
use components::exercise_timer::ExerciseTimer;
use models::interlude_state::{InterludeState, InterludeStatus};
use services::exercise_loader::load_exercises;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let exercises = load_exercises().expect("Could not load exercises");

    let mut state = use_signal(|| InterludeState {
        exercises,
        current_index: 0,
        status: InterludeStatus::Idle,
        exercise_finished: false,
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


            match state().status {

                InterludeStatus::Idle => rsx! {

                    button {

                        onclick: move |_| {
                            state.write().start();
                        },

                        "Start"
                    }

                },


                InterludeStatus::Preparing => rsx! {

                    Countdown {
                        seconds: 3,

                        on_finished: move |_| {
                            state.write().activate();
                        }
                    }

                },


                InterludeStatus::Active => rsx! {

                    if let Some(exercise)=current {

                        ExerciseTimer {
                            duration_seconds: exercise.minimum_seconds,

                            on_finished: move |_| {
                                state.write().finish_timer();
                            }
                        }

                        ExerciseCard {
                            exercise,

                            exercise_finished: state().exercise_finished,

                            on_done: move |_| {
                                state.write().complete();
                            }
                        }

                    }

                },


                InterludeStatus::WaitingForConfirmation => rsx! {

                    button {

                        onclick: move |_| {
                            state.write().next();
                        },

                        "Next"
                    }

                }

            }

        }

    }
}
