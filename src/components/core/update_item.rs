use std::sync::{Arc, RwLock};

use sfi_core::core::{Inventory, Item};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::RouteAgentDispatcher};

use crate::{
    components::app::AppRoute,
    services::data::{DataAgent, DataAgentRequest, DataAgentResponse},
};

pub struct UpdateItem {
    link: ComponentLink<Self>,
    props: Props,
    item: Option<Arc<RwLock<Item>>>,
    old_name: String,
    data_bridge: Box<dyn Bridge<DataAgent>>,
    route_dispatcher: RouteAgentDispatcher,
    is_busy: bool,

    form_data: FormData,
}

pub enum Msg {
    UpdateName(String),
    UpdateEan(String),
    DataAgentResponse(DataAgentResponse),
    Confirm,
    Cancel,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub inventory_uuid: Uuid,
    pub item_uuid: Uuid,
}

impl Component for UpdateItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let inventory_uuid = props.inventory_uuid;
        let item_uuid = props.item_uuid;

        let mut data_bridge = DataAgent::bridge(link.callback(Msg::DataAgentResponse));
        data_bridge.send(DataAgentRequest::GetItem(inventory_uuid, item_uuid));

        Self {
            data_bridge,
            route_dispatcher: RouteAgentDispatcher::new(),
            form_data: FormData::default(),
            is_busy: false,
            link,
            item: None,
            old_name: String::default(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.form_data.name = name;
                true
            }
            Msg::UpdateEan(ean) => {
                self.form_data.ean = if ean.is_empty() { None } else { Some(ean) };
                true
            }
            Msg::Confirm => {
                // Give the new card to the listing component
                self.data_bridge.send(DataAgentRequest::UpdateItem {
                    target: self.item.clone().expect("Cannot be none"),
                    name: self.form_data.name.clone(),
                    ean: self.form_data.ean.clone(),
                });

                self.is_busy = true;
                true
            }
            Msg::Cancel => {
                // Cancel the update of the item
                self.route_dispatcher
                    .send(RouteRequest::ChangeRoute(AppRoute::Inventories.into()));

                self.is_busy = true;
                true
            }
            Msg::DataAgentResponse(res) => match res {
                DataAgentResponse::Item(item) => {
                    {
                        let item = item.read().expect("Cannot read item");
                        self.old_name = item.name.clone();

                        self.form_data = FormData {
                            name: item.name.clone(),
                            ean: item.ean.clone(),
                        };
                    }
                    self.item = Some(item);
                    true
                }
                DataAgentResponse::UpdatedItem => {
                    // Navigate back to the inventories
                    self.route_dispatcher.send(RouteRequest::ChangeRoute(
                        AppRoute::Items(self.props.inventory_uuid).into(),
                    ));

                    self.is_busy = false;
                    true
                }

                // These responses should be ignored
                DataAgentResponse::Inventories(_)
                | DataAgentResponse::NewInventoryUuid(_)
                | DataAgentResponse::InvalidInventoryUuid
                | DataAgentResponse::UpdatedInventory(_)
                | DataAgentResponse::Inventory(_)
                | DataAgentResponse::NewItemUuid(_) => false,
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
            <div>
                // A heading
                <h2>{ "Edit item " } {item.name.clone()}</h2>

                // The name input
                <input
                    type="text"
                    placeholder="name"
                    disabled=self.is_busy
                    value={self.form_data.name.to_owned()}
                    oninput=self.link.callback(|i: InputData| Msg::UpdateName(i.value))
                /> { " " }

                // The EAN input
                <input
                    type="text"
                    placeholder="EAN"
                    disabled=self.is_busy
                    value={self.form_data.ean.clone().unwrap_or(String::default())}
                    oninput=self.link.callback(|i: InputData| Msg::UpdateEan(i.value))
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

                // TODO implement edit options for owner,

            </div>
        }
    }
}

#[derive(Default)]
struct FormData {
    name: String,
    ean: Option<String>,
}
