use crate::{
    components::app::{AppRoute, AppRouterButton},
    services::data::{DataAgent, DataAgentRequest, DataAgentResponse},
};
use sfi_core::{store::InventoryHandle, Inventory};
use yew::prelude::*;
use yew::{prelude::*, Bridge};

pub struct InventoryCard {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    OpenInventory,
    EditInventory,
    ExportInventory,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub inventory: Inventory,
}

impl Component for InventoryCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenInventory => false,
            Msg::EditInventory => {
                // TODO implement inventory edits
                true
            }
            Msg::ExportInventory => {
                // TODO implement inventory exports
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="sfi-card">
                <h3>{ self.props.inventory.name() }</h3>
                <span class="sfi-subtitle">{ self.props.inventory.uuid() }</span>

                <AppRouterButton route=AppRoute::Items(self.props.inventory.uuid().clone())>{ "Open inventory" }</AppRouterButton> { " " }
                <AppRouterButton route=AppRoute::UpdateInventory(self.props.inventory.uuid().clone())>{ "Edit" }</AppRouterButton> { " " }
                <button disabled=true onclick=self.link.callback(|_| Msg::ExportInventory)>{ "Export" }</button> { " " }
            </div>
        }
    }
}
