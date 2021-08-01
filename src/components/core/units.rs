use std::sync::{Arc, RwLock, RwLockReadGuard};

use crate::{
    components::{
        app::{AppRoute, AppRouterButton},
        core::unit_card::UnitCard,
    },
    services::data::{DataAgent, DataAgentRequest, DataAgentResponse},
};
use sfi_core::core::{Inventory, Item, Unit};
use uuid::Uuid;
use yew::prelude::*;

pub struct Units {
    link: ComponentLink<Self>,
    data_bridge: Box<dyn Bridge<DataAgent>>,
    item: Option<Arc<RwLock<Item>>>,
    props: Props,
}

pub enum Msg {
    AgentResponse(DataAgentResponse),
    RequestNewState,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub inventory_uuid: Uuid,
    pub item_uuid: Uuid,
}

impl Component for Units {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut data_bridge = DataAgent::bridge(link.callback(Msg::AgentResponse));
        data_bridge.send(DataAgentRequest::GetItem(
            props.inventory_uuid,
            props.item_uuid,
        ));

        Self {
            link,
            data_bridge,
            item: None,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestNewState => {
                self.data_bridge.send(DataAgentRequest::GetItem(
                    self.props.inventory_uuid,
                    self.props.item_uuid,
                ));
                false
            }
            Msg::AgentResponse(res) => match res {
                DataAgentResponse::Item(item) => {
                    self.item = Some(item);
                    true
                }
                DataAgentResponse::InvalidInventoryUuid => {
                    self.item = None;
                    true
                }

                // These responses should be ignored
                DataAgentResponse::Inventories(_)
                | DataAgentResponse::NewInventoryUuid(_)
                | DataAgentResponse::Inventory(_)
                | DataAgentResponse::UpdatedItem
                | DataAgentResponse::DeletedInventory(_)
                | DataAgentResponse::NewItemUuid(_)
                | DataAgentResponse::DeletedItem(_)
                | DataAgentResponse::UpdatedInventory(_) => false,
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let item = if let Some(item) = &self.item {
            item.read().expect("Cannot read item")
        } else {
            return html! { <p>{ "Cannot find this item" }</p> };
        };

        html! {
            <>

            <h1>{ "Units of " } {item.name.clone()}</h1>

            <button onclick=self.link.callback(|_| Msg::RequestNewState)>
                { "Refresh units" }
            </button>  { " " }

            <AppRouterButton route=AppRoute::Home>{ "Go to home" }</AppRouterButton> { " " }
            <AppRouterButton route=AppRoute::Inventories>{ "Go to inventories" }</AppRouterButton> { " " }

            // Create inventory
            <AppRouterButton route=AppRoute::CreateUnit(self.props.inventory_uuid, self.props.item_uuid)>{ "New unit" }</AppRouterButton>

            <br /> <br />

            <div class="sfi-cards-container">
                { self.view_units() }
            </div>

            </>
        }
    }
}

impl Units {
    fn view_units(&self) -> Html {
        let items = if let Some(item) = &self.item {
            item.read().expect("Cannot read item").units.to_owned()
        } else {
            return html! {};
        };

        if items.is_empty() {
            html! { <p>{ "This item doesn't currently contain any units." }</p> }
        } else {
            items.iter().map(|item| self.view_unit(item)).collect()
        }
    }

    fn view_unit(&self, unit: &Arc<RwLock<Unit>>) -> Html {
        let unit = unit.read().expect("Cannot read unit").to_owned();
        html! { <UnitCard inventory_uuid=self.props.inventory_uuid unit=unit /> }
    }
}
