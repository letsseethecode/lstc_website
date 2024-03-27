use yew::{function_component, html, Html};

use crate::components::Template;

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    html!(
        <Template>
            {"Not Found"}
        </Template>
    )
}
