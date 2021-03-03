use crate::constants;
use yew::prelude::*;

/// The root component of sfi-web
pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

            <h1>{constants::meta::NAME}</h1>
            {constants::meta::ABOUT}

            // The main application
            // TODO implement

            <p>
                { "Get the source code "}
                <a href="https://github.com/Bernd-L/sfi-web">{ "here" }</a>
            </p>

            <div style=BOXED>
                <h3>{constants::license::license_notice_title()}</h3>

                <p>
                    {constants::license::LICENSE_SHORT} <br />
                    {"The license: "} <a href=constants::license::LICENSE_URL>{constants::license::LICENSE_URL}</a>
                </p>

                {constants::license::license_notice_body()}
            </div>

            </>
        }
    }
}

const BOXED: &str = "
    max-width: 700px;
";
