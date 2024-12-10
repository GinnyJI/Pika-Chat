use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub username: Option<String>,
    #[prop_or_default]
    pub avatar_url: Option<String>,
    #[prop_or_default]
    pub on_logout: Option<Callback<()>>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let on_logout = props
        .on_logout
        .as_ref()
        .map(|cb| {
            let cb = cb.clone();
            Callback::from(move |_| {
                gloo::console::log!("Logout button clicked");
                cb.emit(());
            })
        });

    html! {
        <header class="header">
            <nav class="nav">
                <a href="/" class="nav-logo">{"Pika Chat"}</a>
                <div class="nav-links">
                    {
                        match &props.username {
                            Some(username) if !username.is_empty() => html! {
                                <>
                                    <span>{format!("Welcome, {}", username)}</span>
                                    {
                                        if let Some(avatar_url) = &props.avatar_url {
                                            html! {
                                                <img src={avatar_url.clone()} alt="User Avatar" class="nav-avatar" />
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    {
                                        if let Some(on_logout) = on_logout {
                                            html! {
                                                <button class="nav-link" onclick={on_logout}>{"Logout"}</button>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </>
                            },
                            _ => html! {
                                <>
                                    <a href="/register" class="nav-link">{"Register"}</a>
                                    <a href="/login" class="nav-link">{"Login"}</a>
                                </>
                            }
                        }
                    }
                </div>
            </nav>
        </header>
    }
}
