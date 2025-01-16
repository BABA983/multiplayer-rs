use std::collections::HashMap;

use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::client::Client;

#[derive(Debug, Clone)]
pub struct Channel {
    id: String,
    clients: RwLock<HashMap<Uuid, Client>>,
    broadcast: broadcast::Sender<String>,
}

impl Channel {
    pub fn new(id: String) -> Self {
        Self {
            id,
            clients: RwLock::new(HashMap::new()),
            broadcast: broadcast::channel(64).0,
        }
    }

    pub fn dispose(&self) {
        unimplemented!()
    }
}
