use sfi_core::Item;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::RouteAgentDispatcher};

use crate::{
    components::app::AppRoute,
    services::data::{DataAgent, DataAgentRequest, DataAgentResponse},
};

pub struct CreateItem {
    link: ComponentLink<Self>,
    name: String,
    ean: Option<String>,
    data_bridge: Box<dyn Bridge<DataAgent>>,
    route_dispatcher: RouteAgentDispatcher,
    is_busy: bool,
}

pub enum Msg {
    UpdateName(String),
    DataAgentResponse(DataAgentResponse),
    Confirm,
    Cancel,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub inventory_uuid: Uuid,
}

impl Component for CreateItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let inventory_uuid = props.inventory_uuid;

        let mut data_bridge = DataAgent::bridge(link.callback(Msg::DataAgentResponse));
        data_bridge.send(DataAgentRequest::GetInventory(inventory_uuid));

        Self {
            data_bridge: DataAgent::bridge(link.callback(Msg::DataAgentResponse)),
            route_dispatcher: RouteAgentDispatcher::new(),
            name: String::new(),
            is_busy: false,
            ean: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // TODO handle EAN
            Msg::UpdateName(name) => {
                self.name = name;
                false
            }
            Msg::Confirm => {
                // Give the new card to the listing component
                self.data_bridge
                    .send(DataAgentRequest::CreateItem(self.name.clone()));

                self.is_busy = true;
                true
            }
            Msg::Cancel => {
                // Cancel the creation of the inventory
                self.route_dispatcher
                    .send(RouteRequest::ChangeRoute(AppRoute::Inventories.into()));

                self.is_busy = true;
                true
            }
            Msg::DataAgentResponse(response) => match response {
                DataAgentResponse::NewInventoryUuid(_uuid) => {
                    self.route_dispatcher
                        .send(RouteRequest::ChangeRoute(AppRoute::Inventories.into()));

                    self.is_busy = false;
                    true
                }
                _ => false,
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
            <div>
                // A heading
                <h2>{ "Create a new item in " } {inventory.name()}</h2>

                // The name input
                <input
                    type="text"
                    placeholder="name"
                    disabled=self.is_busy
                    value={self.name.to_owned()}
                    oninput=self.link.callback(|i: InputData| Msg::UpdateName(i.value))
                /> { " " }

                // Save edits button
                <button
                    onclick=self.link.callback(|_| Msg::Confirm)
                    disabled=self.is_busy
                >
                    { "Save" }
                </button>  { " " }

                // Cancel button
                <button
                    onclick=self.link.callback(|_| Msg::Cancel)
                    disabled=self.is_busy
                >
                    { "Cancel" }
                </button>

            </div>
        }
    }
}
