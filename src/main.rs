use std::fs;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    for entry in fs::read_dir(".").unwrap() {
        // if entry.file_name
    }

    html! {
        <div>
       </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
