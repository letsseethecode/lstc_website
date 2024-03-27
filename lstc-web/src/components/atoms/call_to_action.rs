use yew::{function_component, html, Html, Properties};

#[derive(Debug, Properties)]
pub struct CallToActionProps {
    children: Html,
}

#[function_component(CallToAction)]
pub fn call_to_action(props: &CallToActionProps) -> Html {
    html!(
        <div class="utc-cta">
            {props.children}
        </div>
    )
}
