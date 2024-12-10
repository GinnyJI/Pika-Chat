use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::login::Login;
use crate::pages::register::Register;
use crate::pages::dashboard::Dashboard;
use crate::pages::home::Home;
use crate::pages::chatroom::Chatroom;

// Define your app's routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/dashboard")]
    Dashboard,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/chatroom/:room_id")]
    Chatroom { room_id: i64 },
    #[at("/")]
    Home,
}

// Define the route switch function
pub fn switch(route: &Route) -> Html {
    match route {
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Home => html! { <Home /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::Chatroom { room_id } => html! { <Chatroom room_id={*room_id} /> },
    }
}
