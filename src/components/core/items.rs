use std::sync::{Arc, RwLock, RwLockReadGuard};

use crate::{
    components::{
        app::{AppRoute, AppRouterButton},
        core::item_card::ItemCard,
    },
    services::data::{DataAgent, DataAgentRequest, DataAgentResponse},
};
use sfi_core::core::{Inventory, Item};
use uuid::Uuid;
use yew::prelude::*;

pub struct Items {
    link: ComponentLink<Self>,
    data_bridge: Box<dyn Bridge<DataAgent>>,
    inventory: Option<Arc<RwLock<Inventory>>>,
    inventory_uuid: Uuid,
}

pub enum Msg {
    AgentResponse(DataAgentResponse),
    RequestNewState,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub inventory_uuid: Uuid,
}

impl Component for Items {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let inventory_uuid = props.inventory_uuid;

        let mut data_bridge = DataAgent::bridge(link.callback(Msg::AgentResponse));
        data_bridge.send(DataAgentRequest::GetInventory(inventory_uuid));

        Self {
            link,
            data_bridge,
            inventory: None,
            inventory_uuid,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestNewState => {
                self.data_bridge
                    .send(DataAgentRequest::GetInventory(self.inventory_uuid));
                false
            }
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
                DataAgentResponse::NewItemUuid(_) => todo!(),
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let inventory = if let Some(inventory) = &self.inventory {
            inventory.read().expect("Cannot read inventory")
        } else {
            return html! { <p>{ "Cannot find this inventory" }</p> };
        };

        html! {
            <>

            <h1>{ "Items of " } {inventory.name.clone()}</h1>

            <button onclick=self.link.callback(|_| Msg::RequestNewState)>
                { "Refresh items" }
            </button>  { " " }

            <AppRouterButton route=AppRoute::Home>{ "Go to home" }</AppRouterButton> { " " }
            <AppRouterButton route=AppRoute::Inventories>{ "Go to inventories" }</AppRouterButton> { " " }

            // Create inventory
            <AppRouterButton route=AppRoute::CreateItem(self.inventory_uuid)>{ "New item" }</AppRouterButton>

            <br /> <br />

            <div class="sfi-cards-container">
                { self.view_items() }
            </div>

            </>
        }
    }
}

impl Items {
    fn view_items(&self) -> Html {
        let items = if let Some(inventory) = &self.inventory {
            inventory
                .read()
                .expect("Cannot read inventory")
                .items
                .to_owned()
        } else {
            return html! {};
        };

        if items.is_empty() {
            html! { <p>{ "This inventory doesn't currently contain any items." }</p> }
        } else {
            items.iter().map(|item| Self::view_item(item)).collect()
        }
    }

    fn view_item(item: &Item) -> Html {
        html! { <ItemCard item=item /> }
    }
}
