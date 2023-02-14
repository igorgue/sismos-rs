#[allow(unused_unsafe)]
use js_sys::Date;
use yew::{html, Component, Context, Html};
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex overflow-hidden relative flex-col justify-center py-6 min-h-screen bg-gray-50 sm:py-12">
                <div class="relative px-6 pt-10 pb-8 bg-white ring-1 shadow-xl sm:px-10 sm:mx-auto sm:max-w-lg sm:rounded-lg ring-gray-900/5">
                    <div class="mx-auto space-x-8 max-w-md">
                        <div class="flex flex-row justify-center space-x-4">
                        </div>

                        <div class="container mx-auto text-white bg-red-500">
                            <a href="#"
                                class="block p-6 max-w-sm bg-white rounded-lg border border-gray-200 shadow dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700">
                                <h5
                                    class="flex flex-row justify-center mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
                                    { self.value }</h5>
                            </a>
                        </div>

                        <div class="flex flex-row justify-end pt-5 text-base leading-7 w-50">
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
