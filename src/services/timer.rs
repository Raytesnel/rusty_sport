use dioxus::prelude::*;
use std::time::Duration;

pub fn use_timer(initial_seconds: u32, active: bool) -> Signal<u32> {
    let mut remaining = use_signal(|| initial_seconds);

    use_future(move || async move {
        while remaining() > 0 && active {
            tokio::time::sleep(Duration::from_secs(1)).await;

            if remaining() > 0 {
                remaining.set(remaining() - 1);
            }
        }
    });

    remaining
}
