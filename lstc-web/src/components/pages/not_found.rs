use crate::components::Console;
use yew::{function_component, html, Html};

use crate::components::Template;

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    html!(
        <Template>
            <h3><Console text="Not Found" /></h3>
        </Template>
    )
}
