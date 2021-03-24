use crate::services::auth::{AuthAgent, AuthAgentRequest};
use sfi_core::users::{UserIdentifier, UserInfo, UserLogin, UserSignup};
use std::rc::Rc;
use yew::{prelude::*, services::fetch::FetchTask};

/// The root component of sfi-web
pub struct LoginComponent {
    link: ComponentLink<Self>,
    state: Rc<AuthState>,
    auth_bridge: Box<dyn Bridge<AuthAgent>>,
    form: LoginForm,
}

struct LoginForm {
    password: String,
    name: String,
}

impl LoginForm {
    fn new() -> Self {
        Self {
            password: String::new(),
            name: String::new(),
        }
    }
}

pub enum Msg {
    StartLogin,
    StartSignup,
    StartLogout,

    NewAuthState(Rc<AuthState>),

    ChangePassword(String),
    ChangeName(String),
}

#[derive(Debug)]
pub enum AuthState {
    Probing(FetchTask),

    // Logged out
    LoggingOut(FetchTask),
    Initial,

    // Login
    LoggingIn(FetchTask),
    LoggedIn(UserInfo),

    // Error
    Error(anyhow::Error),
}

impl Component for LoginComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Initiate a bridge to the auth agent
        let mut auth_bridge = AuthAgent::bridge(link.callback(Msg::NewAuthState));

        // Request the current authentication status
        // auth_bridge.send(AuthAgentRequest::GetAuthStatus);

        Self {
            state: Rc::new(AuthState::Initial),
            link,
            auth_bridge,
            form: LoginForm::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Handle auth requests by the user
            Msg::StartLogin => self.auth_bridge.send(AuthAgentRequest::Login(UserLogin {
                identifier: UserIdentifier::Name(self.form.name.clone()),
                password: self.form.password.clone(),
                totp: None,
            })),
            Msg::StartSignup => self.auth_bridge.send(AuthAgentRequest::Signup(UserSignup {
                password: self.form.password.clone(),
                name: self.form.name.clone(),
            })),
            Msg::StartLogout => self.auth_bridge.send(AuthAgentRequest::Logout),

            // Handle form inputs
            Msg::ChangePassword(password) => self.form.password = password,
            Msg::ChangeName(name) => self.form.name = name,

            // Handle auth agent callbacks
            Msg::NewAuthState(state) => self.state = state,
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

            { self.view_form() }
            { self.view_state() }

            </>
        }
    }
}

impl LoginComponent {
    fn view_form(&self) -> Html {
        let busy = self.is_busy();

        match self.state.as_ref() {
            AuthState::LoggedIn(_) | AuthState::LoggingOut(_) => {
                html! {

                    <button
                        onclick=self.link.callback(|_| Msg::StartLogout)
                        disabled=busy
                    >
                        {"Log out"}
                    </button>

                }
            }
            _ => {
                html! {
                    <>

                    // The name of the new user
                    <input
                        type="text"
                        placeholder="user name"
                        disabled=busy
                        oninput=self.link.callback(|i: InputData| Msg::ChangeName(i.value))
                    />

                    // Space
                    // TODO use CSS instead
                    { " " }

                    // The input fields for new cards
                    <input
                        type="password"
                        placeholder="password"
                        disabled=busy
                        oninput=self.link.callback(|i: InputData| Msg::ChangePassword(i.value))
                    />

                    // // Space
                    // // TODO use CSS instead
                    // { " " }

                    // // The input fields for new cards
                    // <input
                    //     type="text"
                    //     placeholder="UUID"
                    //     oninput=self.link.callback(|i: InputData| Msg::ChangeUuid(i.value))
                    // />

                    // TODO use CSS instead
                    <br />
                    <br />

                    <button
                        onclick=self.link.callback(|_| Msg::StartLogin)
                        disabled=busy
                    >
                        {"Login"}
                    </button>

                    // Space
                    // TODO use CSS instead
                    { " " }

                    <button
                        onclick=self.link.callback(|_| Msg::StartSignup)
                        disabled=busy
                    >
                        {"Sign up"}
                    </button>

                    </>
                }
            }
        }
    }

    fn view_state(&self) -> Html {
        match self.state.as_ref() {
            AuthState::Probing(_) => html! {<p>{ "Fetching auth state..." }</p>},
            AuthState::Initial => {
                html! {<p>{ "Not logged in" }</p>}
            }
            AuthState::LoggingIn(_) => html! {<p>{ "Logging in..." }</p>},
            AuthState::LoggedIn(user) => {
                html! {<p>{ format!("Logged in as {} ({})", user.name, user.uuid) }</p>}
            }
            AuthState::Error(error) => html! {<p>{ "Couldn't log in: " }{ error }</p>},
            AuthState::LoggingOut(_) => html! {<p>{ "Logging out..." }</p>},
        }
    }

    fn is_busy(&self) -> bool {
        match self.state.as_ref() {
            AuthState::LoggingOut(_) | AuthState::LoggingIn(_) | AuthState::Probing(_) => true,
            AuthState::Initial | AuthState::LoggedIn(_) | AuthState::Error(_) => false,
        }
    }
}
