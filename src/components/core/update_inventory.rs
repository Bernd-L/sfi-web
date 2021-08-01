use std::sync::{Arc, RwLock};

use sfi_core::core::Inventory;
use uuid::Uuid;
use yew::{prelude::*, services::DialogService};
use yew_router::{agent::RouteRequest, prelude::RouteAgentDispatcher};

use crate::{
    components::app::AppRoute,
    services::data::{DataAgent, DataAgentRequest, DataAgentResponse},
};

pub struct UpdateInventory {
    link: ComponentLink<Self>,
    inventory: Option<Arc<RwLock<Inventory>>>,
    old_name: String,
    data_bridge: Box<dyn Bridge<DataAgent>>,
    route_dispatcher: RouteAgentDispatcher,
    is_busy: bool,

    form_data: FormData,
}

pub enum Msg {
    UpdateName(String),
    DataAgentResponse(DataAgentResponse),
    Confirm,
    Cancel,
    Delete,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub inventory_uuid: Uuid,
}

impl Component for UpdateInventory {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let inventory_uuid = props.inventory_uuid;

        let mut data_bridge = DataAgent::bridge(link.callback(Msg::DataAgentResponse));
        data_bridge.send(DataAgentRequest::GetInventory(inventory_uuid));

        Self {
            data_bridge,
            route_dispatcher: RouteAgentDispatcher::new(),
            form_data: FormData::default(),
            is_busy: false,
            link,
            inventory: None,
            old_name: String::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.form_data.name = name;
                true
            }
            Msg::Confirm => {
                // Give the new card to the listing component
                self.data_bridge.send(DataAgentRequest::UpdateInventory {
                    target: self.inventory.clone().expect("Cannot be none"),
                    name: self.form_data.name.clone(),
                    owner: self.form_data.owner.clone(),
                    admins: self.form_data.admins.clone(),
                    writables: self.form_data.writables.clone(),
                    readables: self.form_data.readables.clone(),
                });

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
            Msg::Delete => {
                let should_kaboom = DialogService::confirm(&format!(
                    "Delete inventory \"{}\"?\nThis operation cannot be undone.",
                    self.old_name
                ));

                if should_kaboom {
                    self.data_bridge.send(DataAgentRequest::DeleteInventory(
                        self.inventory.clone().expect("Must be Some"),
                    ))
                }

                should_kaboom
            }
            Msg::DataAgentResponse(res) => match res {
                DataAgentResponse::Inventory(inventory) => {
                    {
                        let inventory = inventory.read().expect("Cannot read inventory");
                        self.old_name = inventory.name.clone();

                        self.form_data = FormData {
                            name: inventory.name.clone(),
                            owner: inventory.owner.clone(),
                            admins: inventory.admins.clone(),
                            writables: inventory.writables.clone(),
                            readables: inventory.readables.clone(),
                        };
                    }
                    self.inventory = Some(inventory);
                    true
                }
                DataAgentResponse::UpdatedInventory(_) => {
                    // Navigate back to the inventories
                    self.route_dispatcher
                        .send(RouteRequest::ChangeRoute(AppRoute::Inventories.into()));

                    self.is_busy = false;
                    true
                }
                DataAgentResponse::DeletedInventory(_) => {
                    self.route_dispatcher
                        .send(RouteRequest::ChangeRoute(AppRoute::Inventories.into()));

                    self.is_busy = true;
                    true
                }

                // These responses should be ignored
                DataAgentResponse::Inventories(_)
                | DataAgentResponse::NewInventoryUuid(_)
                | DataAgentResponse::InvalidInventoryUuid
                | DataAgentResponse::UpdatedItem
                | DataAgentResponse::Item(_)
                | DataAgentResponse::DeletedItem(_)
                | DataAgentResponse::NewItemUuid(_) => false,
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
            <div>
                // A heading
                <h2>{ "Edit inventory " } {inventory.name.clone()}</h2>

                // The name input
                <input
                    type="text"
                    placeholder="name"
                    disabled=self.is_busy
                    value={self.form_data.name.to_owned()}
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
                </button>  { " " }

                // Delete button
                <button
                    onclick=self.link.callback(|_| Msg::Delete)
                    disabled=self.is_busy
                >
                    { "Delete" }
                </button>

                // TODO implement edit options for owner,

            </div>
        }
    }
}

#[derive(Default)]
struct FormData {
    name: String,
    owner: Uuid,
    admins: Vec<Uuid>,
    writables: Vec<Uuid>,
    readables: Vec<Uuid>,
}
