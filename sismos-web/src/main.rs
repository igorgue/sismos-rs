use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

pub fn ai_response(prompt: String, ai_cb: Callback<AttrValue>) {
    spawn_local(async move {
        let url = format!("http://0.0.0.0:1972/api?prompt={}", prompt);

        let response = Request::get(url.as_str())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        ai_cb.emit(AttrValue::from(response));
    });
}

pub enum Msg {
    Send,
    Input(String),
    AIResponse(AttrValue),
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Send => {
                ai_response(self.prompt.clone(), ctx.link().callback(Msg::AIResponse));
            }
            Msg::Input(prompt) => {
                self.prompt = prompt;
            }
            Msg::AIResponse(response) => {
                self.message = response.to_string();
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
                <pre>{ self.message.to_string() }</pre>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
