mod app;
mod routes;
mod pages;
mod components;
mod services;
use wasm_logger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("pika_chat"));
    yew::Renderer::<app::App>::new().render();
}
