mod app;
mod routes;
mod pages;
mod components;
mod services;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
