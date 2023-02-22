use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlInputElement, Request, Response};
use yew::prelude::*;

async fn ai_response(prompt: String) -> Result<String, JsValue> {
    let url = format!("http://0.0.0.0:1972/api?prompt={}", prompt);
    let mut opts = web_sys::RequestInit::new();

    opts.method("GET");
    opts.mode(web_sys::RequestMode::Cors);

    let request = Request::new_with_str_and_init(url.as_str(), &opts)?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

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
            // <button {onclick}>{ "Send" }</button>
            <button onclick={ onclick }>{ "Send" }</button>
            <p class="italic">{ message.to_string() }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
