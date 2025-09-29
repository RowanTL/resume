use gloo_console::log;
use std::error::Error;
use std::fs;
use wasm_bindgen::JsValue;
use yew::prelude::*;

fn count_resumes() -> Result<usize, Box<dyn Error>> {
    let mut resume_count: usize = 0;
    for entry in fs::read_dir(".")? {
        match entry?.file_name().into_string() {
            Ok(converted_str) => {
                if converted_str.contains("resume") {
                    resume_count += 1
                }
            }
            Err(_) => {} // do nothing in this case
        }
    }

    Ok(resume_count)
}

#[function_component]
fn App() -> Html {
    match count_resumes() {
        Ok(count) => {
            html! {
                <div>
                    <p> {count} </p>
               </div>
            }
        }
        Err(err) => {
            let err_str = JsValue::from(format!("{:?}", err));
            log!(err_str);
            html! {
                <div>
                    <p> {"error occured"} </p>
               </div>
            }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
