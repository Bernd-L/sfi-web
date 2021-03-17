use crate::constants;
use sfi_core::{store::Store, Inventory};
use yew::prelude::*;

pub struct Inventories {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    store: Store<'static>,
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
                // { for self.props.store.}

                // {self.view_inventory()}
            </>
        }
    }
}

impl Inventories {
    fn view_inventory(inventory: &Inventory) -> Html {
        html! {
            <div>
                // Or maybe use an h1, or h2?
                <span>{ inventory.name() }</span>
            </div>
        }
    }
}
