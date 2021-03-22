use serde::{Deserialize, Serialize};
use sfi_core::store::{self, InventoryHandle, Store};
use std::collections::HashSet;
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetInventories,
    MakeDebugInventory,
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
    type Output = Vec<InventoryHandle<'static>>;

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

                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, (&self.store).to_vec())
                }
            }
            Request::MakeDebugInventory => {
                // TODO implement create inventory
                // self.store
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
