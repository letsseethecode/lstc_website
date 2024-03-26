use yew::{function_component, html, Html, Properties};

use crate::components::Template;

#[derive(Properties, PartialEq)]
pub struct ConsoleProps {
    pub text: String,
    #[prop_or(50)]
    pub delay: usize,
}

#[function_component(Console)]
pub fn console(props: &ConsoleProps) -> Html {
    let s = (props.text.clone() + " ").replace(' ', "\u{00a0}");
    html!(
        <div class="console">
            {for s.chars().enumerate().map(|(ix, c)| html!(
                <span class="console__character" style={format!("animation: reveal {}ms linear", ix*props.delay)}>{c}</span>
            ))}
            <span class="console__cursor" style={format!("animation-duration: {}ms", props.delay*10)}>{"\u{00a0}"}</span>
        </div>
    )
}
