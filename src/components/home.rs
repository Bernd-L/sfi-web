use super::{
    app::{AppRoute, AppRouterButton},
    login_demo::LoginComponent,
};
use crate::constants::{self, css::BOXED};
use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::RouteAgentDispatcher};

/// The root component of sfi-web
pub struct Home {
    link: ComponentLink<Self>,
    route_dispatcher: RouteAgentDispatcher,
}

pub enum Msg {
    ShowInventories,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            route_dispatcher: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowInventories => {
                self.route_dispatcher
                    .send(RouteRequest::ChangeRoute(AppRoute::Inventories.into()));
                false
            }
        }
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
