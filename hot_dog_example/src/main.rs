use std::collections::HashMap;

use dioxus::{logger::tracing::info, prelude::*};
use serde::Deserialize;

mod components;

use components::BreedPic;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    // use signal rerenders component when value changes
    let mut breed = use_signal(|| "bouvier".to_string());

    // use resource runs inner async function when component mounts
    let get_breed_list = use_resource(move || async move {

        #[derive(Deserialize, Debug, PartialEq, Clone)]
        struct BreedsRes {
            message: HashMap<String, Vec<String>>
        }
        
        let breed_list = reqwest::get("https://dog.ceo/api/breeds/list/all")
        .await
        .unwrap()
        .json::<BreedsRes>()
        .await;

        let Ok(breeds) = breed_list else {
            return rsx!("error, fetching breeds");
        };
        
        info!("breeds: {:#?}", breeds);

        rsx!{
            for cur_breed in breeds.message.keys().take(20).cloned() {
                button {
                    onclick: move |_| breed.set(cur_breed.clone()),
                    "{cur_breed}"
                }
            }
        }
    });

    let Some(breeds) = get_breed_list() else {
        return rsx!("loading... :F");
    };

    rsx! {
        h1 { "select a dog breed : {breed}" },
        BreedPic { breed }
        div { width: "400px", {breeds} }
    }
}


