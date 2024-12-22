use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        // Footer Section
        <footer class="footer">
            <p class="footer-text">{"© 2024 Pika Chat. All rights reserved."}</p>
        </footer>
    }
}
