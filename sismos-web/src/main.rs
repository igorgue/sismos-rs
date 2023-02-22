use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlInputElement, Request, Response};
use yew::prelude::*;

async fn _ai_response(prompt: String) -> Result<String, JsValue> {
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

pub enum Msg {
    Send,
    Input(String),
}

pub struct App {
    prompt: String,
    message: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            prompt: String::new(),
            message: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Send => {}
            Msg::Input(prompt) => {
                self.prompt = prompt;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Send);
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            Msg::Input(input.value())
        });

        html! {
            <div>
                <input type="text" {oninput} />
                <button {onclick}>{ "Send" }</button>
                <p class="italic">{ self.message.to_string() }</p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
