use crate::constants;
use anyhow::Result;
use serde_json::json;
use yew::{
    format::Json,
    prelude::*,
    services::{
        fetch::{Request, Response},
        FetchService,
    },
    web_sys::console,
};

/// The root component of sfi-web
pub struct Login {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Do,
    Done(Response<Result<String>>),
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Do => self.debug_login(),
            Msg::Done(response) => console::log_1(&format!("{:?}", response).into()),
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

            <button
                onclick=self.link.callback(|_| Msg::Do)
            >
                {"debug login"}
            </button>

            </>
        }
    }
}

impl Login {
    fn debug_login(&self) {
        let json_payload = json!({
            "uuid": "5a099ef4-9cf1-4e78-93d6-f9cc48a52126",
            "password": "123",
            "totp": null
        });

        let request = Request::post("http://localhost:8080/api/authentication/login")
            .header("Content-Type", "application/json")
            .body(Json(&json_payload))
            .expect("Failed to build request.");

        let task = FetchService::fetch(
            request,
            self.link.callback(|response: Response<Result<String>>| {
                if response.status().is_success() {
                    Msg::Done(response)
                } else {
                    Msg::Done(response)
                }
            }),
        );
    }
}
