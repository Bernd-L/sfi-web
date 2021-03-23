use yew::prelude::*;

use crate::services::{
    auth::AuthAgent,
    data::{DataAgent, DataAgentRequest, DataAgentResponse},
};

pub struct DebugTools {
    link: ComponentLink<Self>,
    data_bridge: Box<dyn Bridge<DataAgent>>,
    // auth_bridge: Box<dyn Bridge<AuthAgent>>,
}

pub enum Msg {
    DeleteAllData,
    DataAgentResponse(DataAgentResponse),
    AuthAgentResponse(DataAgentResponse),
}

impl Component for DebugTools {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            data_bridge: DataAgent::bridge(link.callback(Msg::DataAgentResponse)),
            // auth_bridge: AuthAgent::bridge(link.callback(Msg::AuthAgentResponse)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteAllData => self.data_bridge.send(DataAgentRequest::DeleteAllData),
            Msg::DataAgentResponse(_) => {}
            Msg::AuthAgentResponse(_) => {}
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
                <button
                    onclick=self.link.callback(|_| Msg::DeleteAllData)
                >
                    { "Delete everything"}
                </button>

            </div>

            <br />

            </>
        }
    }
}
