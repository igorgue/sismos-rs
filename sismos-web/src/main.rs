#[allow(unused_unsafe)]
use gloo::console;
use js_sys::Date;
use yew::{html, Component, Context, Html};

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    value: i64, // This will store the counter value
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                console::log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log!("minus one");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="relative flex min-h-screen flex-col justify-center overflow-hidden bg-gray-50 py-6 sm:py-12">
              <div class="relative bg-white px-6 pt-10 pb-8 shadow-xl ring-1 ring-gray-900/5 sm:mx-auto sm:max-w-lg sm:rounded-lg sm:px-10">
                <div class="mx-auto max-w-md space-x-8">
                  <div class="flex flex-row justify-center space-x-4">
                    <button class="h-10 w-12 rounded-lg bg-gray-100 shadow" onclick={ctx.link().callback(|_| Msg::Increment)}>{ "+1" }</button>

                    <button class="h-10 w-12 rounded-lg bg-gray-100 shadow" onclick={ctx.link().callback(|_| Msg::Decrement)}>{ "-1" }</button>

                    <button class="h-10 w-20 rounded-lg bg-gray-100 shadow" onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>{ "+2" }</button>
                  </div>

                  <div class="flex flex-row justify-center space-x-4 pt-5">
                    <a href="#" class="block max-w-sm rounded-lg border border-gray-200 bg-white p-6 shadow hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700">
                      <h5 class="mb-2 flex flex-row justify-center text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{ self.value }</h5>
                    </a>
                  </div>

                  <div class="w-50 flex flex-row justify-end pt-5 text-base leading-7">
                    <p>
                      { "Rendered: " }
                      { String::from(Date::new_0().to_string()) }
                    </p>
                  </div>
                </div>
              </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
