#[allow(unused_unsafe)]
// use js_sys::Date;
use yew::{html, Component, Context, Html};
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    input: String, // This will store the counter value
    output: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::from("Ultimos sismos en Chinandega"),
            output: String::from("Sismos AI: HOLA"),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <input type="text" value={ self.input.as_str() } />
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
