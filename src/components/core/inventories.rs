use crate::{
    components::{
        app::{AppRoute, AppRouterButton},
        core::inventory_card::InventoryCard,
    },
    services::data::{DataAgent, DataAgentRequest, DataAgentResponse},
};
use sfi_core::{store::InventoryHandle, Inventory};
use yew::{prelude::*, Bridge};

pub enum Msg {
    // NewState(&'static Vec<InventoryHandle<'static>>),
    // NewState(Vec<InventoryHandle<'static>>),
    AgentResponse(DataAgentResponse),
    RequestNewState,
}

pub struct Inventories {
    link: ComponentLink<Inventories>,
    data_bridge: Box<dyn Bridge<DataAgent>>,
    handles: Option<Vec<InventoryHandle<'static>>>,
}

impl Component for Inventories {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Initiate a bridge to the data agent
        let mut data_bridge = DataAgent::bridge(link.callback(Msg::AgentResponse));

        // Request a list of the currently accessible inventory handles
        data_bridge.send(DataAgentRequest::GetInventories);

        // Create the component
        Self {
            data_bridge,
            link,
            handles: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestNewState => {
                self.data_bridge.send(DataAgentRequest::GetInventories);
                false
            }
            Msg::AgentResponse(response) => match response {
                DataAgentResponse::Inventories(state) => {
                    self.handles = Some(state);
                    true
                }
                DataAgentResponse::NewInventoryUuid(_uuid) => false,

                // Those responses should be ignored
                DataAgentResponse::Inventory(_) | DataAgentResponse::InvalidInventoryUuid => false,
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

            <h1>{ "Inventories" }</h1>

            <button onclick=self.link.callback(|_| Msg::RequestNewState)>
                { "Refresh inventories" }
            </button>  { " " }

            <AppRouterButton route=AppRoute::Home>{ "Go to home" }</AppRouterButton> { " " }

            // Create inventory
            <AppRouterButton route=AppRoute::CreateInventory>{ "New inventory" }</AppRouterButton>

            <br /> <br />

            <div class="sfi-cards-container">
                { self.view_inventories() }
            </div>

            </>
        }
    }
}

impl Inventories {
    fn view_inventories(&self) -> Html {
        if let Some(handles) = &self.handles {
            if handles.is_empty() {
                html! { <p>{ "No accessible inventories found" }</p> }
            } else {
                handles
                    .iter()
                    .map(|inventory| Self::view_inventory(inventory))
                    .collect()
            }
        } else {
            html! { <p>{ "Loading inventories..." }</p> }
        }
    }

    fn view_inventory(inventory: &Inventory) -> Html {
        html! { <InventoryCard inventory=inventory /> }
    }
}
