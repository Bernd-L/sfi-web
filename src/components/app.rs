use crate::components::{core::inventories::Inventories, home::Home, login_demo::LoginComponent};
use sfi_core::store::Store;
use uuid::Uuid;
use yew::{
    prelude::*,
    virtual_dom::{Transformer, VComp},
    web_sys::Url,
};
use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};

/// The root component of sfi-web
pub struct App;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/login"]
    Login,

    #[to = "/register"]
    Register,

    #[to = "/account"]
    AccountSettings,

    #[to = "/inventories/{uuid}"]
    Inventory(Uuid),

    #[to = "/inventories"]
    Inventories,

    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),

    #[to = "/!"]
    Home,
}

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
pub type AppRouterButton = RouterButton<AppRoute>;

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

            <LoginComponent />

                <main>
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        redirect=AppRouter::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route))).into()
                        })
                    />
                </main>

            </>
        }
    }
}

impl App {
    fn switch(switch: AppRoute) -> Html {
        let store = Store::new();

        match switch {
            AppRoute::Home => {
                html! { <Home /> }
            }
            AppRoute::Inventories => {
                html! { <Inventories store=store/> }
            }

            // TODO implement 404
            // AppRoute::PageNotFound(Permissive(route)) => {
            //     html! { <PageNotFound route=route /> }
            // }
            _ => {
                html! {<span>{ "TODO implement this" }</span>}
            }
        }
    }
}
