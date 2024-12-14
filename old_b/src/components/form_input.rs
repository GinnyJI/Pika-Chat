use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormInputProps {
    pub label: String,
    pub placeholder: String,
    pub input_type: String,
    pub value: String,
    pub oninput: Callback<String>,
}

#[function_component(FormInput)]
pub fn form_input(props: &FormInputProps) -> Html {
    html! {
        <div class="login-field">
            <label class="text-sm font-medium text-gray-700">{ &props.label }</label>
            <input
                type={props.input_type.clone()}
                placeholder={props.placeholder.clone()}
                value={props.value.clone()}
                oninput={props.oninput.reform(|e: InputEvent| e.target_unchecked_into::<web_sys::HtmlInputElement>().value())}
                class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
            />
        </div>
    }
}
