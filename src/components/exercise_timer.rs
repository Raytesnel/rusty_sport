use dioxus::prelude::*;

use crate::services::timer::use_timer;

#[component]
pub fn ExerciseTimer(duration_seconds: u32, on_finished: EventHandler<()>) -> Element {
    let remaining = use_timer(duration_seconds, true);

    use_effect(move || {
        if remaining() == 0 {
            on_finished.call(());
        }
    });

    rsx! {

        div {

            style:"
                color:white;
                font-size:60px;
            ",

            "{remaining()}"

        }

    }
}
