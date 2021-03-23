use crate::components::login::AuthState;

use super::auth::{AuthAgent, AuthAgentRequest as AuthRequest};
use serde::{Deserialize, Serialize};
use sfi_core::store::{InventoryHandle, Store};
use std::{collections::HashSet, rc::Rc};
use uuid::Uuid;
use yew::{
    format::Json,
    services::{storage::Area, StorageService},
    worker::*,
};

const EVENT_STORE_KEY: &'static str = "sfi.events.store";

#[derive(Serialize, Deserialize, Debug)]
pub enum DataAgentRequest {
    GetInventories,
    MakeDebugInventory,
    CreateInventory(String),
    DeleteAllData,
}

#[derive(Debug)]
pub enum DataAgentResponse {
    Inventories(Vec<InventoryHandle<'static>>),
    NewInventoryUuid(Uuid),
}

pub enum Msg {
    NewAuthState(Rc<AuthState>),
}

pub struct DataAgent {
    link: AgentLink<DataAgent>,
    subscribers: HashSet<HandlerId>,
    local_storage: StorageService,
    auth_state: Rc<AuthState>,

    store: Store<'static>,
    auth_bridge: Box<dyn Bridge<AuthAgent>>,
}

impl Agent for DataAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = DataAgentRequest;
    type Output = DataAgentResponse;

    fn create(link: AgentLink<Self>) -> Self {
        // Get a reference to localStorage
        let local_storage = StorageService::new(Area::Local).expect("Cannot use localStorage");

        // Load the event store from localStorage
        let store = {
            if let Json(Ok(store)) = local_storage.restore(EVENT_STORE_KEY) {
                // Load the event store from localStorage
                store
            } else {
                // If no such entry exists, create a new one
                Store::new()
            }
        };

        Self {
            subscribers: HashSet::new(),
            store,
            local_storage,
            auth_state: Rc::new(AuthState::Initial),
            auth_bridge: AuthAgent::bridge(link.callback(Msg::NewAuthState)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::NewAuthState(auth_state) => self.auth_state = auth_state,
        };
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            DataAgentRequest::GetInventories => {
                // TODO remove these clones
                let res = (&self.store).to_vec();

                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, DataAgentResponse::Inventories(res.clone()))
                }
            }
            DataAgentRequest::MakeDebugInventory => {
                let res = self
                    .store
                    .make_inventory("my inv".to_string(), Uuid::new_v4());

                self.persist_data();

                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, DataAgentResponse::NewInventoryUuid(res))
                }
            }
            DataAgentRequest::CreateInventory(name) => {
                if let AuthState::LoggedIn(user_info) = self.auth_state.as_ref() {
                    let res = self.store.make_inventory(name, user_info.uuid);

                    self.persist_data();

                    for sub in self.subscribers.iter() {
                        self.link
                            .respond(*sub, DataAgentResponse::NewInventoryUuid(res))
                    }
                }
            }
            DataAgentRequest::DeleteAllData => {
                self.store = Store::new();
                self.persist_data();

                let res = (&self.store).to_vec();

                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, DataAgentResponse::Inventories(res.clone()))
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        // FIelD `1` oF STrucT `yeW::AGENT::hANnlERiD` Is PRivATE
        // PRiVATE fIELd rUsTC e0616
        // if id.1 {}
        if format!("{:?}", &id).contains("true") {
            self.subscribers.insert(id);
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl DataAgent {
    fn persist_data(&mut self) -> () {
        self.local_storage.store(EVENT_STORE_KEY, Json(&self.store));
    }
}
