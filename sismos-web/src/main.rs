use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let prompt = use_state(|| "".to_string());
    let message = use_state(|| "???".to_string());
    let onclick = {
        let message = message.clone();
        move |_| {
            message.set("World".to_string());
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
