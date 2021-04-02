use crate::components::{
    core::{
        create_inventory::CreateInventory, create_item::CreateItem, inventories::Inventories,
        items::Items,
    },
    debug_tools::DebugTools,
    home::Home,
    login::LoginComponent,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::{components::RouterAnchor, prelude::*};

/// The root component of sfi-web
pub struct App;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/login!"]
    Login,

    #[to = "/register!"]
    Register,

    #[to = "/account!"]
    AccountSettings,

    #[to = "/inventories/{inv_uuid}/items/{it_uuid}/units/new!"]
    CreateUnit(Uuid, Uuid),

    #[to = "/inventories/{inv_uuid}/items/{it_uuid}/units/{un_uuid}!"]
    UpdateUnit(Uuid, Uuid, Uuid),

    #[to = "/inventories/{inv_uuid}/items/{it_uuid}/units!"]
    Units(Uuid, Uuid),

    #[to = "/inventories/{inv_uuid}/items/new!"]
    CreateItem(Uuid),

    #[to = "/inventories/{inv_uuid}/items/{it_uuid}!"]
    UpdateItem(Uuid, Uuid),

    #[to = "/inventories/{inv_uuid}/items!"]
    Items(Uuid),

    #[to = "/inventories/new!"]
    CreateInventory,

    #[to = "/inventories/{inv_uuid}!"]
    UpdateInventory(Uuid),

    #[to = "/inventories!"]
    Inventories,

    #[to = "/!"]
    Home,

    #[to = "{*:any}"]
    PageNotFound(String),
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
            // TODO implement some kind of nav bar


            // Include debug tools in debug builds
            { self.view_debug_tools() }

            // The login component
            <LoginComponent />

            // The router outlet
            <AppRouter render=AppRouter::render(Self::handle_route) />

            </>
        }
    }
}

impl App {
    fn handle_route(route: AppRoute) -> Html {
        match route {
            AppRoute::Home => {
                html! { <Home /> }
            }
            AppRoute::Inventories => {
                html! { <Inventories /> }
            }
            AppRoute::CreateInventory => {
                html! { <CreateInventory /> }
            }

            // The 404-like display
            AppRoute::PageNotFound(path) => {
                html! {
                    <>

                    <h1>{ "Page not found" }</h1>

                    <p>
                        { "The path " }
                        { path }
                        { " didn't match any known routes." }
                    </p>

                    <p>
                        { "Try navigating back to the home page using the button below:" }
                    </p>

                    <AppRouterButton route=AppRoute::Home>{ "Go to home" }</AppRouterButton>

                    </>
                }
            }

            // TODO implement the route handlers
            AppRoute::Login => {
                html! { "still need to implement this"}
            }
            AppRoute::Register => {
                html! { "still need to implement this"}
            }
            AppRoute::AccountSettings => {
                html! { "still need to implement this"}
            }
            AppRoute::CreateUnit(inv_uuid, it_uuid) => {
                html! { "still need to implement this"}
            }
            AppRoute::UpdateUnit(inv_uuid, it_uuid, un_uuid) => {
                html! { "still need to implement this"}
            }
            AppRoute::Units(inv_uuid, it_uuid) => {
                html! { "still need to implement this"}
            }
            AppRoute::CreateItem(inv_uuid) => {
                html! { <CreateItem inventory_uuid=inv_uuid /> }
            }
            AppRoute::UpdateItem(inv_uuid, it_uuid) => {
                html! { "still need to implement this"}
            }
            AppRoute::Items(inv_uuid) => {
                html! { <Items inventory_uuid=inv_uuid /> }
            }
            AppRoute::UpdateInventory(inv_uuid) => {
                html! { "still need to implement this"}
            }
        }
    }

    #[allow(unreachable_code)]
    fn view_debug_tools(&self) -> Html {
        // Include debug tools in debug builds
        #[cfg(debug_assertions)]
        return html! { <DebugTools /> };

        // Don't include debug tools in release builds
        html! {}
    }
}
