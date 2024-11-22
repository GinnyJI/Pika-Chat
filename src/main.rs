use yew::prelude::*;

pub enum Msg {}
pub struct App;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Welcome to Yew Frontend" }</h1>
                <p>{ "This is the starting point of your Yew application." }</p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
