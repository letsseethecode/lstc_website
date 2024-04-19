use crate::components::{Console, Template};
use yew::{function_component, html, Html};

#[function_component(HomePage)]
pub fn home() -> Html {
    html! {
        <Template>
            <section>
                <h3><Console text="The Code is the Star." /></h3>

                <p>
                    {"A new hybrid on-line/in-person meetup where the code is the star.
                    See "}<strong>{"live demonstrations"}</strong>{", join "}
                    <strong>{"interactive code-reviews"}</strong>{", and discuss with
                    attendees as they showcase and explain their code."}
                </p>
            </section>

            <section>
                <p>
                    {"
                    If you have something to show off, or a project you'd like input,
                    or are looking to hone your skills in code, then this is the place
                    to do that. All in a fun and supportive environment.
                    "}
                </p>
            </section>

            <section>
                <h3>{"Why?"}</h3>

                <p>{"Our motto is \"Connect. Collaborate. Code.\""}</p>

                <p>
                    {"
                    Meet people who are passionate about the same things you are.
                    Contribute to the multiple projects we're hosting, and see how
                    others solve problems.
                    "}
                </p>

                <p>
                    {"
                    Learn new skills. Hone existing ones. Help others. Code!
                    "}
                </p>
            </section>

                <section>
                <h3>{"How?"}</h3>

                <p>
                    {"Check out our github repositories for information "}
                    <a target="_blank" href="https://github.com/letsseethecode">{"https://github.com/letsseethecode"}</a>
                </p>
            </section>

            <section>
                <h3>{"When?"}</h3>

                <p>
                    {"Weekly: Monday @ 6:30pm on "}<strong>{"Twitch"}</strong>
                    <img class="content__twitch" src="./images/twitch_PNG28.png" width="18" height="18" />
                    <a target="_blank" href="https://twitch.tv/letsseethecode">{"https://twitch.tv/letsseethecode"}</a>
                </p>
                <p>
                    {"Monthly: hybrid "}
                    <strong>{"on-line AND in-person"}</strong>
                    {" at "}
                    <a target="_blank" href="https://github.com/letsseethecode/events">
                        {"The Canal House in Nottingham, UK"}
                    </a>{". Next hybrid event is Monday 18th March 2024."}
                    {"(Free food and drink!)"}
                </p>
            </section>
        </Template>
    }
}
