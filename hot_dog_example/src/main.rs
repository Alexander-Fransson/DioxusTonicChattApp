use dioxus::prelude::*;

//mod guide_component;
//use guide_component::DogApp;

static CSS: Asset = asset!("assets/main.css");
static DOG: Asset = asset!("assets/dog-3981540_1280.jpg", ImageAssetOptions::new().with_avif()); // this effectivizes the loading of the image, the example was png though

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{href: CSS},
        Title {},
        DogView {}
        
        
    }
}

#[component]
fn Title() -> Element {
    rsx!(
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    )
}

#[component]
fn DogView() -> Element {

    let img_src = use_hook(|| { DOG }); // a hook
    
    //Hook rules:
    //* no hooks in loops
    //* no hooks in conditional
    //* no hooks in closures
    //* only in components or other hooks
    //* hooks must be called in the same order every time

    let skip = move |evt| {};
    let save = move |evt| {};

    rsx!(
        div { id: "dogview",
            img { src: img_src }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    )
}