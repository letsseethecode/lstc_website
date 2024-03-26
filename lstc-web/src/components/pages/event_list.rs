use yew::{function_component, html, Html};

use crate::components::Template;

#[function_component(EventListPage)]
pub fn event_list() -> Html {
    html!(
        <Template>
            <h3>{"Events"}</h3>
            <h4>{"upcoming events"}</h4>
            <table>
                <tr>
                <td><a href="/event/2024-04-01">{"2024-04-01"}</a></td>
                <td>{"online"}</td>
                <td></td>
            </tr>
            <tr>
                <td><a href="/event/2024-04-08">{"2024-04-08"}</a></td>
                <td>{"online"}</td>
                <td></td>
            </tr>
            <tr>
                <td><a href="/event/2024-04-15">{"2024-04-15"}</a></td>
                <td>{"online"}</td>
                <td>{"in-person"}</td>
            </tr>
        </table>
            <h4>{"past events"}</h4>
            <table>
                <tr>
                    <td><a href="/event/2024-03-11">{"2024-03-25"}</a></td>
                    <td>{"online"}</td>
                    <td></td>
                </tr>
                <tr>
                    <td><a href="/event/2024-03-18">{"2024-03-18"}</a></td>
                    <td>{"online"}</td>
                    <td>{"in-person"}</td>
                </tr>
            </table>
        </Template>
    )
}
