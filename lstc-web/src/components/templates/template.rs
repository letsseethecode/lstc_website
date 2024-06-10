use crate::app::{Link, Route};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TemplateProps {
    pub children: Html,
}

#[function_component(Template)]
pub fn template(props: &TemplateProps) -> Html { 
    html!(
        <div class="template">
            <header class="template__header">
                <div class="template__content">
                    <h1 class="title">
                        {"letsseethecode.com"}
                    </h1>
                    <Link to={Route::Home} >
                        <img class="logo" src="/images/lstc_logo.png" />
                    </Link>
                    <h2 class="vertical subtitle">
                        <span>{"Connect."}</span>
                        <span>{"Collaborate."}</span>
                        <span>{"Code."}</span>
                    </h2>
                </div>
            </header>
            <section class="template__menu">
                <div class="template__content">
                    <ul class="menu">
                        <li class="menu__item">
                            <Link to={Route::Home}>{"Home"}</Link>
                        </li>
                        <li class="menu__item">
                            <Link to={Route::EventList}>{"Events"}</Link>
                        </li>
                    </ul>
                </div>
            </section>
            <main class="template__main">
                <div class="template__content">
                    {props.children.clone()}
                </div>
            </main>
            <footer class="template__footer">
                <div class="template__content">
                    {"Footer | Goes | Here"}
                </div>
            </footer>
            <div class="template__scan-lines" />
        </div>
    )
}
