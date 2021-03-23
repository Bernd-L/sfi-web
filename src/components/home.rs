use super::app::{AppRoute, AppRouterButton};
use crate::constants::{self, css::BOXED};
use yew::prelude::*;
// use yewtil::{Pure, PureComponent};

// Yewtil currently does not compile 🤦🤦🤦

/// The root component of sfi-web
// pub type Home = Pure<PureHome>;
pub type Home = PureHome;

pub struct PureHome;

// impl PureComponent for Home {

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

            <h1>{constants::meta::NAME}</h1>
            {constants::meta::ABOUT}

            <p>
                { "Get the source code "}
                <a href="https://github.com/Bernd-L/sfi-web">{ "here" }</a>
            </p>

            <p>
                { "To navigate to the inventories page, press the button below:"}
            </p>

            <AppRouterButton route=AppRoute::Inventories>{ "Inventories" }</AppRouterButton>

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
