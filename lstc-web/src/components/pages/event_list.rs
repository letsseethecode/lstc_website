use yew::{function_component, html, Html};

use crate::app::{Link, Route};
use crate::components::{Console, Template};

#[function_component(EventListPage)]
pub fn event_list() -> Html {
    html!(
        <Template>
            <h3><Console text="Events" /></h3>
            <h4>{"upcoming events"}</h4>
            <table>
                <tr>
                    <td><Link to={Route::EventView { id: "2024-04-01".to_string() }}>{"2024-04-01"}</Link></td>
                    <td>{"online"}</td>
                    <td></td>
                </tr>
                <tr>
                    <td><Link to={Route::EventView { id: "2024-04-08".to_string() }}>{"2024-04-08"}</Link></td>
                    <td>{"online"}</td>
                    <td></td>
                </tr>
                <tr>
                    <td><Link to={Route::EventView { id: "2024-04-15".to_string() }}>{"2024-04-15"}</Link></td>
                    <td>{"online"}</td>
                    <td>{"in-person"}</td>
                </tr>
            </table>
            <h4>{"past events"}</h4>
            <table>
                <tr>
                    <td><Link to={Route::EventView { id: "2024-03-25".to_string() }}>{"2024-03-25"}</Link></td>
                    <td>{"online"}</td>
                    <td></td>
                </tr>
                <tr>
                    <td><Link to={Route::EventView { id: "2024-04-18".to_string() }}>{"2024-04-18"}</Link></td>
                    <td>{"online"}</td>
                    <td>{"in-person"}</td>
                </tr>
            </table>
        </Template>
    )
}
