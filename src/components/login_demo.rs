use anyhow::Result;
use serde_json::json;
use yew::{
    format::Json,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    web_sys::console,
};

use crate::types::User;

/// The root component of sfi-web
pub struct Login {
    link: ComponentLink<Self>,
    state: State,
}

pub enum Msg {
    MakeRequest,
    LoggedIn(User),
    LoginError(anyhow::Error),
}

pub enum State {
    Initial,
    LoggingIn(FetchTask),
    LoggedIn(User),
    Error(anyhow::Error),
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: State::Initial,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeRequest => self.debug_login(),
            Msg::LoggedIn(user) => {
                console::log_1(&format!("{:?}", &user).into());
                self.state = State::LoggedIn(user);
            }
            Msg::LoginError(error) => {
                console::log_1(&format!("{:?}", &error).into());
                self.state = State::Error(error);
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

            { self.view_button() }
            { self.view_state() }

            </>
        }
    }
}

impl Login {
    fn debug_login(&mut self) {
        let json_payload = json!({
            "uuid": "5a099ef4-9cf1-4e78-93d6-f9cc48a52126",
            "password": "123",
            "totp": null
        });

        let request = Request::post("http://localhost:8080/api/v1/authentication/login")
            .header("Content-Type", "application/json")
            .body(Json(&json_payload))
            .expect("Failed to build request.");

        let task = FetchService::fetch(
            request,
            self.link
                .callback(|response: Response<Json<Result<User>>>| {
                    let Json(data) = response.into_body();

                    match data {
                        Ok(user) => Msg::LoggedIn(user),
                        Err(error) => Msg::LoginError(error),
                    }
                }),
        );

        // Store the task so it isn't canceled immediately
        self.state = match task {
            Ok(fetch_task) => State::LoggingIn(fetch_task),
            Err(error) => State::Error(error),
        };
    }

    fn view_button(&self) -> Html {
        html! {
            <button
                onclick=self.link.callback(|_| Msg::MakeRequest)
                disabled=self.is_logging_in()
            >
                {"debug login"}
            </button>
        }
    }

    fn view_state(&self) -> Html {
        match &self.state {
            State::Initial => html! {<p>{ "Press login to log in" }</p>},
            State::LoggingIn(_) => html! {<p>{ "Logging in..." }</p>},
            State::LoggedIn(user) => html! {<p>{ "Logged in as " }{ user.uuid }</p>},
            State::Error(error) => html! {<p>{ "Couldn't log in: " }{ error }</p>},
        }
    }

    fn is_logging_in(&self) -> bool {
        if let State::LoggingIn(_) = &self.state {
            true
        } else {
            false
        }
    }
}
