use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <div class="content">
            <img class="content__image" src="./images/lstc_logo.png" />
            <h1 class="content__title">{"letsseethecode.com"}</h1>
            <h2 class="content__subtitle">
                <span>{"Connect."}</span>
                <span>{"Collaborate."}</span>
                <span>{"Code."}</span>
            </h2>

            <h3>{"The Code is the Star."}</h3>

            <p>
                {"A new hybrid on-line/in-person meetup where the code is the star.
                See "}<strong>{"live demonstrations"}</strong>{", join "}
                <strong>{"interactive code-reviews"}</strong>{", and discuss with
                attendees as they showcase and explain their code."}
            </p>

            <p>
                {"
                If you have something to show off, or a project you'd like input,
                or are looking to hone your skills in code, then this is the place
                to do that. All in a fun and supportive environment.
                "}
            </p>

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

            <h3>{"How?"}</h3>

            <p>
                {"Check out our github repositories for information "}
                <a href="https://github.com/letsseethecode">{"https://github.com/letsseethecode"}</a>
            </p>

            <h3>{"When?"}</h3>

            <p>
                {"Weekly: Monday @ 6:30pm on "}<strong>{"Twitch"}</strong>
                <img class="content__twitch" src="./images/twitch_PNG28.png" />
                <a href="https://twitch.tv/letsseethecode">{"https://twitch.tv/letsseethecode"}</a>
            </p>
            <p>
                {"Monthly: hybrid "}
                <strong>{"on-line AND in-person"}</strong>
                {" at "}
                <a href="https://github.com/letsseethecode/events">
                    {"The Canal House in Nottingham, UK"}
                </a>{". Next hybrid event is Monday 18th March 2024."}
                {"(Free food and drink!)"}
            </p>
        </div>
    </main>
    }
}
