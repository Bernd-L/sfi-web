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
}

pub enum Msg {
    DeleteAllData,
    ProbeAuth,
    DataAgentResponse(DataAgentResponse),
    AuthAgentResponse(Rc<AuthState>),
}

impl Component for DebugTools {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            data_bridge: DataAgent::bridge(link.callback(Msg::DataAgentResponse)),
            auth_bridge: AuthAgent::bridge(link.callback(Msg::AuthAgentResponse)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteAllData => self.data_bridge.send(DataAgentRequest::DeleteAllData),
            Msg::ProbeAuth => self.auth_bridge.send(AuthAgentRequest::GetAuthStatus),
            Msg::DataAgentResponse(_response) => {}
            Msg::AuthAgentResponse(auth_state) => {
                log::debug!("Received response is {:?}", &auth_state);
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
                <button onclick=self.link.callback(|_| Msg::DeleteAllData)>
                    { "Delete everything"}
                </button> { " " }
                <button onclick=self.link.callback(|_| Msg::ProbeAuth)>
                    { "Probe auth" }
                </button>

            </div>

            <br />

            </>
        }
    }
}
