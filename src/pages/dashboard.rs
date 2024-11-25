use yew::prelude::*;

pub struct Dashboard;

impl Component for Dashboard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-row items-center justify-center min-h-screen bg-gray-100">
                <h1 class="text-3xl font-bold text-gray-800">{ "Dashboard" }</h1>
                <p class="mt-4 text-gray-600">{ "Welcome to the Dashboard page! This is a demo." }</p>
            </div>
        }
    }
}
