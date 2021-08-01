use crate::{
    components::app::{AppRoute, AppRouterButton},
    services::data::{DataAgent, DataAgentRequest, DataAgentResponse},
};
use sfi_core::core::Unit;
use uuid::Uuid;
use yew::prelude::*;
use yew::{prelude::*, Bridge};

pub struct UnitCard {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    OpenItem,
    EditItem,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub unit: Unit,
    pub inventory_uuid: Uuid,
}

impl Component for UnitCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenItem => false,
            Msg::EditItem => {
                // TODO implement item edits
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let update_unit_route = AppRoute::UpdateUnit(
            self.props.inventory_uuid,
            self.props.unit.item_uuid,
            self.props.unit.uuid,
        );

        html! {
            <div class="sfi-card">
                <h3>{ self.props.unit.name.clone() }</h3>
                <span class="sfi-subtitle">{ self.props.unit.uuid }</span>

                <AppRouterButton route=update_unit_route>{ "Edit" }</AppRouterButton> { " " }
            </div>
        }
    }
}
