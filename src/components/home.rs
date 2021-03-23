use super::app::{AppRoute, AppRouterButton};
use crate::constants::{self, css::BOXED};
use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::RouteAgentDispatcher};
use yewtil::{Pure, PureComponent};

/// The root component of sfi-web
pub type Home = Pure<PureHome>;

struct PureHome;

impl PureComponent for Home {
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
