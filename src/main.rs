use yew::prelude::*;

pub enum Msg {}

pub struct App;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Welcome to Yew Frontend" }</h1>
                <p>{ "This is the starting point of your Yew application." }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
