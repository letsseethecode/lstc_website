use crate::app::{Link, Route};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TemplateProps {
    pub children: Html,
}

#[function_component(Template)]
pub fn template(props: &TemplateProps) -> Html {
    html!(
        <main>
            <div class="content">
                <img class="content__image" src="/images/lstc_logo.png" />
                <h1 class="content__title">
                    {"letsseethecode.com"}
                </h1>

                <h2 class="content__subtitle">
                    <span>{"Connect."}</span>
                    <span>{"Collaborate."}</span>
                    <span>{"Code."}</span>
                </h2>

                <ul class="menu">
                    <li class="menu__item">
                        <Link to={Route::Home}>{"Home"}</Link>
                    </li>
                    <li class="menu__item">
                        <Link to={Route::EventList}>{"Events"}</Link>
                    </li>
                </ul>
                {props.children.clone()}
            </div>
        </main>
    )
}
