use anyhow::Result;
use sfi_core::users::{UserIdentifier, UserInfo, UserLogin, UserSignup};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchOptions, FetchTask, Request, Response},
        FetchService,
    },
    web_sys::{console, RequestCredentials},
};

/// The root component of sfi-web
pub struct LoginComponent {
    link: ComponentLink<Self>,
    state: AuthState,
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
    LoggedIn(UserInfo),
    LoggedOut,
    LoginError(anyhow::Error),

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
        Self {
            state: Self::probe_state(&link),
            link,
            form: LoginForm::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartLogin => self.login(),
            Msg::StartSignup => self.signup(),
            Msg::StartLogout => self.logout(),
            Msg::LoggedIn(user) => {
                console::log_1(&format!("{:?}", &user).into());
                self.state = AuthState::LoggedIn(user);
            }
            Msg::LoggedOut => {
                self.state = AuthState::Initial;
            }
            Msg::LoginError(error) => {
                console::log_1(&format!("{:?}", &error).into());
                self.state = AuthState::Error(error);
            }
            Msg::ChangePassword(password) => self.form.password = password,
            Msg::ChangeName(name) => self.form.name = name,
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
    fn login(&mut self) {
        let login_info = UserLogin {
            identifier: UserIdentifier::Name(self.form.name.clone()),
            password: self.form.password.clone(),
            totp: None,
        };

        let request = Request::post("http://localhost:8080/api/v1/authentication/login")
            .header("Content-Type", "application/json")
            .body(Json(&login_info))
            .expect("Failed to build request (login).");

        let options = FetchOptions {
            credentials: Some(RequestCredentials::SameOrigin),
            ..FetchOptions::default()
        };

        let callback = self
            .link
            .callback(|response: Response<Json<Result<UserInfo>>>| {
                let Json(data) = response.into_body();

                match data {
                    Ok(user) => Msg::LoggedIn(user),
                    Err(error) => Msg::LoginError(error),
                }
            });

        let task = FetchService::fetch_with_options(request, options, callback);

        // Store the task so it isn't canceled immediately
        self.state = match task {
            Ok(fetch_task) => AuthState::LoggingIn(fetch_task),
            Err(error) => AuthState::Error(error),
        };
    }

    fn signup(&mut self) {
        let signup_info = UserSignup {
            password: self.form.password.clone(),
            name: self.form.name.clone(),
        };

        let request = Request::post("http://localhost:8080/api/v1/authentication/signup")
            .header("Content-Type", "application/json")
            .body(Json(&signup_info))
            .expect("Failed to build request (signup).");

        let options = FetchOptions {
            credentials: Some(RequestCredentials::SameOrigin),
            ..FetchOptions::default()
        };

        let callback = self
            .link
            .callback(|response: Response<Json<Result<UserInfo>>>| {
                let Json(data) = response.into_body();

                match data {
                    Ok(user) => Msg::LoggedIn(user),
                    Err(error) => Msg::LoginError(error),
                }
            });

        let task = FetchService::fetch_with_options(request, options, callback);

        // Store the task so it isn't canceled immediately
        self.state = match task {
            Ok(fetch_task) => AuthState::LoggingIn(fetch_task),
            Err(error) => AuthState::Error(error),
        };
    }

    fn logout(&mut self) {
        let request = Request::get("http://localhost:8080/api/v1/authentication/logout")
            .body(Nothing)
            .expect("Failed to build request (logout).");

        let options = FetchOptions {
            credentials: Some(RequestCredentials::SameOrigin),
            ..FetchOptions::default()
        };

        let callback = self.link.callback(|response: Response<Json<Result<()>>>| {
            let Json(data) = response.into_body();

            match data {
                Ok(_) => Msg::LoggedOut,
                Err(error) => Msg::LoginError(error),
            }
        });

        let task = FetchService::fetch_with_options(request, options, callback);

        // Store the task so it isn't canceled immediately
        self.state = match task {
            Ok(fetch_task) => AuthState::LoggingOut(fetch_task),
            Err(error) => AuthState::Error(error),
        };
    }

    fn probe_state(link: &ComponentLink<Self>) -> AuthState {
        let request = Request::get("http://localhost:8080/api/v1/authentication/status")
            .body(Nothing)
            .expect("Failed to build request (probe).");

        let options = FetchOptions {
            credentials: Some(RequestCredentials::SameOrigin),
            ..FetchOptions::default()
        };

        let callback = link.callback(|response: Response<Json<Result<UserInfo>>>| {
            let Json(data) = response.into_body();

            match data {
                Ok(user) => Msg::LoggedIn(user),
                Err(_) => Msg::LoggedOut,
            }
        });

        let task = FetchService::fetch_with_options(request, options, callback);

        // Store the task so it isn't canceled immediately
        match task {
            Ok(fetch_task) => AuthState::Probing(fetch_task),
            Err(error) => AuthState::Error(error),
        }
    }

    fn view_form(&self) -> Html {
        let busy = self.is_busy();

        match self.state {
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
        match &self.state {
            AuthState::Probing(_) => html! {<p>{ "Fetching state..." }</p>},
            AuthState::Initial => {
                html! {<p>{ "Press login to log in (name only used for sign up)" }</p>}
            }
            AuthState::LoggingIn(_) => html! {<p>{ "Logging in..." }</p>},
            AuthState::LoggedIn(user) => {
                html! {<p>{ format!("Logged in as {} ({})", user.uuid, user.name) }</p>}
            }
            AuthState::Error(error) => html! {<p>{ "Couldn't log in: " }{ error }</p>},
            AuthState::LoggingOut(_) => html! {<p>{ "Logging out..." }</p>},
        }
    }

    fn is_busy(&self) -> bool {
        match &self.state {
            AuthState::LoggingOut(_) | AuthState::LoggingIn(_) | AuthState::Probing(_) => true,
            AuthState::Initial | AuthState::LoggedIn(_) | AuthState::Error(_) => false,
        }
    }
}
