use yew::prelude::*;

// Props for the Panel Component
#[derive(Properties, PartialEq)]
pub struct PanelProps {
    pub children: Children,
}

#[function_component(Panel)]
pub fn panel(props: &PanelProps) -> Html {
    let panel_styles = "
        position: fixed; 
        top: 68px;
        left: 0; 
        height: calc(100% - 122px);  
        width: auto; 
        background-color: #f9fafb; 
        border-right: 1px solid #e5e7eb; 
        padding: 1rem; 
        box-shadow: 2px 0 5px rgba(0, 0, 0, 0.1);
    ";

    html! {
        <div style={panel_styles}>
            { for props.children.iter() }
        </div>
    }
}
