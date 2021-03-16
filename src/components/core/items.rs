use crate::constants;
use yew::prelude::*;

pub struct Items;

impl Component for Items {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
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

            </>
        }
    }
}

const BOXED: &str = "
    max-width: 700px;
";
