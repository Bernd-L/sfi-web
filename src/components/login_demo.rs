use std::str::FromStr;

use anyhow::Result;
use sfi_core::types::{UserInfo, UserLogin, UserLogout, UserSignup};
use uuid::Uuid;
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
    state: State,
    form: LoginForm,
}

struct LoginForm {
    password: String,
    uuid: String,
    name: String,
}

impl LoginForm {
    fn new() -> Self {
        Self {
            password: String::new(),
            uuid: String::new(),
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

    ChangeUuid(String),
    ChangePassword(String),
    ChangeName(String),
}

pub enum State {
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
                self.state = State::LoggedIn(user);
            }
            Msg::LoggedOut => {
                self.state = State::Initial;
            }
            Msg::LoginError(error) => {
                console::log_1(&format!("{:?}", &error).into());
                self.state = State::Error(error);
            }
            Msg::ChangeUuid(uuid) => self.form.uuid = uuid,
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
            uuid: Uuid::from_str(&self.form.uuid).unwrap(),
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
            Ok(fetch_task) => State::LoggingIn(fetch_task),
            Err(error) => State::Error(error),
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
            Ok(fetch_task) => State::LoggingIn(fetch_task),
            Err(error) => State::Error(error),
        };
    }

    fn logout(&mut self) {
        let logout_info = UserLogout {};

        let request = Request::get("http://localhost:8080/api/v1/authentication/logout")
            .body(Json(&logout_info))
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
            Ok(fetch_task) => State::LoggingOut(fetch_task),
            Err(error) => State::Error(error),
        };
    }

    fn view_form(&self) -> Html {
        if let State::LoggedIn(_) = self.state {
            html! {

                <button
                    onclick=self.link.callback(|_| Msg::StartLogout)
                    disabled=self.is_busy()
                >
                    {"Log out"}
                </button>

            }
        } else {
            html! {
                <>

                // The name of the new user
                <input
                    type="text"
                    placeholder="user name"
                    oninput=self.link.callback(|i: InputData| Msg::ChangeName(i.value))
                />

                // Space
                // TODO use CSS instead
                { " " }

                // The input fields for new cards
                <input
                    type="password"
                    placeholder="password"
                    oninput=self.link.callback(|i: InputData| Msg::ChangePassword(i.value))
                />

                // Space
                // TODO use CSS instead
                { " " }

                // The input fields for new cards
                <input
                    type="text"
                    placeholder="UUID"
                    oninput=self.link.callback(|i: InputData| Msg::ChangeUuid(i.value))
                />

                // TODO use CSS instead
                <br />
                <br />

                <button
                    onclick=self.link.callback(|_| Msg::StartLogin)
                    disabled=self.is_busy()
                >
                    {"Login"}
                </button>

                // Space
                // TODO use CSS instead
                { " " }

                <button
                    onclick=self.link.callback(|_| Msg::StartSignup)
                    disabled=self.is_busy()
                >
                    {"Sign up"}
                </button>

                </>
            }
        }
    }

    fn view_state(&self) -> Html {
        match &self.state {
            State::Probing(_) => html! {<p>{ "Fetching state..." }</p>},
            State::Initial => {
                html! {<p>{ "Press login to log in (name only used for sign up)" }</p>}
            }
            State::LoggingIn(_) => html! {<p>{ "Logging in..." }</p>},
            State::LoggedIn(user) => {
                html! {<p>{ format!("Logged in as {} ({})", user.uuid, user.name) }</p>}
            }
            State::Error(error) => html! {<p>{ "Couldn't log in: " }{ error }</p>},
            State::LoggingOut(_) => html! {<p>{ "Logging out..." }</p>},
        }
    }

    fn is_busy(&self) -> bool {
        match &self.state {
            State::LoggingOut(_) | State::LoggingIn(_) | State::Probing(_) => true,
            State::Initial | State::LoggedIn(_) | State::Error(_) => false,
        }
    }

    fn probe_state(link: &ComponentLink<Self>) -> State {
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
            Ok(fetch_task) => State::Probing(fetch_task),
            Err(error) => State::Error(error),
        }
    }
}
