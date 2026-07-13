mod models;
mod services;

use dioxus::prelude::*;

use services::exercise_loader::load_exercises;


fn main() {
    dioxus::launch(App);
}


fn App() -> Element {

    let exercises = load_exercises()
        .expect("Could not load exercises");


    rsx! {
        div {
            style: "
                width: 100vw;
                height: 100vh;
                background: #111;
                color: white;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                font-family: sans-serif;
            ",

            h1 {
                "Interlude"
            }

            p {
                "Loaded {exercises.len()} exercises"
            }

            for exercise in exercises {
                div {
                    style: "
                        margin: 10px;
                        padding: 10px;
                        border: 1px solid #444;
                    ",

                    h2 {
                        "{exercise.name}"
                    }

                    p {
                        "{exercise.value}"
                    }
                }
            }
        }
    }
}
