use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div style="min-height: 100vh; display: flex; flex-direction: column; background-color: #f9fafb;">
            // Header Section
            <header style="background-color: #facc15; padding: 1rem; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); position: static; width: 100%;">
                <nav style="max-width: 1200px; margin: 0 auto; display: flex; justify-content: space-between; align-items: center;">
                    <div style="font-size: 1.5rem; font-weight: bold; color: #1f2937;">
                        <a href="/">{"Pika Chat"}</a>
                    </div>
                    <div style="display: flex; gap: 1rem;">
                        <a href="/register" style="color: #1f2937; text-decoration: none; font-weight: 500; transition: color 0.2s; hover: color: #4b5563;">
                            {"Register"}
                        </a>
                        <a href="/login" style="color: #1f2937; text-decoration: none; font-weight: 500; transition: color 0.2s; hover: color: #4b5563;">
                            {"Login"}
                        </a>
                    </div>
                </nav>
            </header>

            // Main Section
            <main style="flex: 1; padding: 2rem; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center;">
                <h1 style="font-size: 2.5rem; font-weight: bold; color: #1f2937; margin-bottom: 1.5rem;">
                    {"Welcome to Pika Chat"}
                </h1>
                <p style="font-size: 1.125rem; color: #4b5563; margin-bottom: 2rem; max-width: 600px;">
                    {"Connect with your friends, chat in real-time, and enjoy the best social experience online. Join now to explore!"}
                </p>
                <img src="static/pikachu.png" alt="Pikachu image" style="width: 10rem; height: 10rem; margin-bottom: 2rem;" />
                <a href="/" style="background-color: #facc15; padding: 1rem 2rem; border-radius: 0.5rem; font-weight: 600; color: #1f2937; text-decoration: none; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); transition: transform 0.2s; hover: transform: scale(1.05);">
                    {"Get Started"}
                </a>
            </main>

            // Footer Section
            <footer style="background-color: #1f2937; color: #e5e7eb; text-align: center; padding: 1rem; width: 100%;">
                <p style="font-size: 0.875rem;">
                    {"© 2024 Pika Chat. All rights reserved."}
                </p>
            </footer>
        </div>
    }
}
