use dioxus::{logger::tracing::info, prelude::*}; // sets the apropriate stuff in the scope



#[component]
pub fn DogApp(breed: String) -> Element {
    rsx!(
        div {
            "Bbbbreed: {breed}"
            button {
                class: "bg-red-100",
                onclick: move |_| {
                    info!("clicked");
                },
                "click me"
            }
        }
    )
}