use std::rc::Rc;

use yew::prelude::*;

use crate::services::{
    auth::{AuthAgent, AuthAgentRequest},
    data::{DataAgent, DataAgentRequest, DataAgentResponse},
};

use super::login::AuthState;

pub struct DebugTools {
    link: ComponentLink<Self>,
    data_bridge: Box<dyn Bridge<DataAgent>>,
    auth_bridge: Box<dyn Bridge<AuthAgent>>,
    inspect_next: bool,
}

pub enum Msg {
    DeleteAllData,
    ProbeAuth,
    DataAgentResponse(DataAgentResponse),
    AuthAgentResponse(Rc<AuthState>),
    MakeDebugInventory,
    InspectInventories,
}

impl Component for DebugTools {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            data_bridge: DataAgent::bridge(link.callback(Msg::DataAgentResponse)),
            auth_bridge: AuthAgent::bridge(link.callback(Msg::AuthAgentResponse)),
            inspect_next: false,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteAllData => self.data_bridge.send(DataAgentRequest::DeleteAllData),
            Msg::ProbeAuth => self.auth_bridge.send(AuthAgentRequest::GetAuthStatus),
            Msg::DataAgentResponse(response) => {
                if self.inspect_next {
                    if let DataAgentResponse::Inventories(inventories) = response {
                        self.inspect_next = false;

                        if inventories.is_empty() {
                            log::debug!("No inventories present");
                        }

                        for inventory in inventories {
                            log::debug!("{}\n{:#?}", inventory.name, inventory);
                        }
                    }
                }
            }
            Msg::AuthAgentResponse(_auth_state) => {
                // log::debug!("Received response is {:?}", &_auth_state);
            }
            Msg::MakeDebugInventory => {
                self.data_bridge.send(DataAgentRequest::MakeDebugInventory);
            }
            Msg::InspectInventories => {
                self.inspect_next = true;
                self.data_bridge.send(DataAgentRequest::GetInventories)
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

            <div>
                <button onclick=self.link.callback(|_| Msg::MakeDebugInventory)> { "Make debug inventory" } </button> { " " }
                <button onclick=self.link.callback(|_| Msg::DeleteAllData)> { "Delete everything"} </button> { " " }
                <button onclick=self.link.callback(|_| Msg::ProbeAuth)> { "Probe auth" } </button> { " " }
                <button onclick=self.link.callback(|_| Msg::InspectInventories)> { "Inspect inventories" } </button> { " " }
            </div>

            <br />

            </>
        }
    }
}
