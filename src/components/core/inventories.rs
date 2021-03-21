use crate::components::app::{AppRoute, AppRouterButton};
use sfi_core::{store::Store, Inventory};
use yew::prelude::*;

pub struct Inventories {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub store: Store<'static>,
}

impl Component for Inventories {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

            <h1>{ "Inventories" }</h1>

            <AppRouterButton route=AppRoute::Home>{ "Go to home" }</AppRouterButton>

            { self.view_inventories() }

            </>
        }
    }
}

impl Inventories {
    fn view_inventories(&self) -> Html {
        self.props
            .store
            .iter()
            .map(|inventory| Self::view_inventory(inventory))
            .collect()
    }

    fn view_inventory(inventory: &Inventory) -> Html {
        html! {

            <div>
                // Or maybe use an h1, or h2?
                <span>{ inventory.name() }</span>
            </div>

        }
    }
}
