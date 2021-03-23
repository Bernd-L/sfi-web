use serde::{Deserialize, Serialize};
use sfi_core::store::{self, InventoryHandle, Store};
use std::collections::HashSet;
use uuid::Uuid;
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetInventories,
    MakeDebugInventory,
}

#[derive(Debug)]
pub enum Response {
    Inventories(Vec<InventoryHandle<'static>>),
    NewInventoryUuid(Uuid),
}

pub struct DataAgent {
    link: AgentLink<DataAgent>,
    subscribers: HashSet<HandlerId>,

    store: Store<'static>,
}

impl Agent for DataAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
            store: Store::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::GetInventories => {
                log::debug!("Get inventories");

                // TODO remove these clones
                let res = (&self.store).to_vec();

                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, Response::Inventories(res.clone()))
                }
            }
            Request::MakeDebugInventory => {
                log::debug!("Make debug inventory");

                let res = self
                    .store
                    .make_inventory("my inv".to_string(), Uuid::new_v4());

                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, Response::NewInventoryUuid(res))
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
