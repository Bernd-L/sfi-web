use crate::services::data::{DataAgent, DataAgentRequest, DataAgentResponse};
use sfi_core::{Inventory, Item};
use uuid::Uuid;
use yew::prelude::*;

pub struct Items {
    link: ComponentLink<Self>,
    data_bridge: Box<dyn Bridge<DataAgent>>,
    inventory: Option<Inventory>,
}

pub enum Msg {
    AgentResponse(DataAgentResponse),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub inventory_uuid: Uuid,
}

impl Component for Items {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut data_bridge = DataAgent::bridge(link.callback(Msg::AgentResponse));
        data_bridge.send(DataAgentRequest::GetInventory(props.inventory_uuid));

        Self {
            link,
            data_bridge,
            inventory: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AgentResponse(res) => match res {
                DataAgentResponse::Inventory(inventory) => {
                    self.inventory = Some(inventory);
                    true
                }
                DataAgentResponse::InvalidInventoryUuid => {
                    self.inventory = None;
                    true
                }

                // Those responses should be ignored
                DataAgentResponse::Inventories(_) | DataAgentResponse::NewInventoryUuid(_) => false,
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let inventory = if let Some(inventory) = &self.inventory {
            inventory
        } else {
            return html! { <p>{ "Cannot find this inventory" }</p> };
        };

        html! {
            <>

            <h1>{ "Items of " } {inventory.name()}</h1>

            // <button onclick=self.link.callback(|_| Msg::RequestNewState)>
            //     { "Refresh inventories" }
            // </button>  { " " }

            // <AppRouterButton route=AppRoute::Home>{ "Go to home" }</AppRouterButton> { " " }

            // // Create inventory
            // <AppRouterButton route=AppRoute::CreateInventory>{ "New inventory" }</AppRouterButton>

            // <br /> <br />

            // <div class="sfi-cards-container">
            //     { self.view_inventories() }
            // </div>

            </>
        }
    }
}
