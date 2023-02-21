use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

#[function_component]
fn App() -> Html {
    let prompt = use_state(|| String::new());
    let message = use_state(|| String::new());
    // let api_endpoint = env!("API_ENDPOINT", "API_ENDPOINT is not set");
    let api_endpoint = "http://0.0.0.0:1972";

    let onclick = {
        let prompt = prompt.clone();
        let message = message.clone();

        move |_| {
            let response = String::new();

            spawn_local(async move {
                response =
                    reqwest_wasm::get(format!("{}/api?prompt={}", api_endpoint, "test").as_str())
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                // message.set(response);
            });

            message.set(response);
        }
    };

    let oninput = {
        let prompt = prompt.clone();

        move |e: InputEvent| {
            let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            prompt.set(input.value());
        }
    };

    html! {
        <div>
            <input type="text" {oninput} />
            <button {onclick}>{ "Send" }</button>
            <p class="italic">{ message.to_string() }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
