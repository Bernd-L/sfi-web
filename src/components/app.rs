use crate::components::{core::inventories::Inventories, home::Home, login::LoginComponent};
use uuid::Uuid;
use yew::prelude::*;
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
    PageNotFound(String),

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
                            AppRoute::PageNotFound(route.route).into()
                        })
                    />
                </main>

            </>
        }
    }
}

impl App {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Home => {
                html! { <Home /> }
            }
            AppRoute::Inventories => {
                html! { <Inventories /> }
            }

            // TODO implement 404
            // AppRoute::PageNotFound(Permissive(route)) => {
            //     html! { <PageNotFound route=route /> }
            // }
            AppRoute::PageNotFound(data) => {
                html! {
                    <>

                    <h1>{ "Page not found" }</h1>


                    <p>
                        { "The path " }
                        {  data }
                        { " didn't match any known routes." }
                    </p>


                    <p>
                        { "Try navigating back to the home page using the button below:" }
                    </p>

                    <AppRouterButton route=AppRoute::Home>{ "Go to home" }</AppRouterButton>

                    </>
                }
            }

            _ => {
                html! {<span>{ "TODO implement this" }</span>}
            }
        }
    }
}
