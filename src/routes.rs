use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::login::Login;
use crate::pages::register::Register;
use crate::pages::dashboard::Dashboard;
use crate::pages::home::Home;

// Define your app's routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/dashboard")]
    Dashboard,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
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
    }
}
