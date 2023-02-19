use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let prompt = use_state(|| String::new());
    let message = use_state(|| String::new());

    let onclick = {
        let prompt = prompt.clone();
        let message = message.clone();

        move |_| {
            message.set(prompt.to_string());
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
