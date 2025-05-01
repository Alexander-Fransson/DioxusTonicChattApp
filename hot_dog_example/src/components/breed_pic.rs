use dioxus::prelude::*;
use serde::Deserialize;

#[component]
pub fn BreedPic(breed: Signal<String>) -> Element {
    
    let mut future = use_resource(move || async move {
        
        #[derive(Deserialize, Debug, PartialEq, Clone)]
        struct DogBreedExampleUrl {
            message: String
        }

        reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
        .await
        .unwrap()
        .json::<DogBreedExampleUrl>()
        .await
    });

    match future.read_unchecked().as_ref() {
        Some(Ok(res)) => rsx!(
            
            div {
                button { onclick: move |_| future.restart(), padding: "5px", background_color: "gray", color: "white", border_radius: "5px", "Click to fetch another doggo" }
                img { max_width: "500px", max_height: "500px", src: "{res.message}" }
            }
        ),
        Some(Err(_)) => rsx!("error, fetching doggo"),
        None => rsx!("loading... :(")
    }
}