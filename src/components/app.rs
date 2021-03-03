use yew::prelude::*;

/// The root component of sfi-web
pub struct App;

// #[derive(Switch)]
// enum AppRoute {
//     #[at = "/login"]
//     Login,

//     #[at = "/register"]
//     Register,

//     #[at = "/delete_account"]
//     Delete,

//     #[at = "/posts/{id}"]
//     ViewPost(i32),

//     #[at = "/posts/view"]
//     ViewPosts,

//     #[at = "/"]
//     Home,
// }

impl Component for App {
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
