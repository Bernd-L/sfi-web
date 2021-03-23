use yew::prelude::*;

use crate::services::data::{DataAgent, Request, Response};

pub struct DebugTools {
    link: ComponentLink<Self>,
    data_bridge: Box<dyn Bridge<DataAgent>>,
}

pub enum Msg {
    DeleteAllData,
    AgentResponse(Response),
}

impl Component for DebugTools {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let data_bridge = DataAgent::bridge(link.callback(Msg::AgentResponse));
        Self { link, data_bridge }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteAllData => self.data_bridge.send(Request::DeleteAllData),
            Msg::AgentResponse(_) => {}
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
