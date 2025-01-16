use std::collections::HashMap;

use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::{channel::Channel, client::Client};

#[derive(Debug)]
pub struct Hub {
    clients: RwLock<HashMap<Uuid, Client>>,
    channels: RwLock<HashMap<String, Channel>>,
    broadcast: broadcast::Sender<String>,
}

impl Hub {
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            channels: RwLock::new(HashMap::new()),
            broadcast: broadcast::channel(64).0,
        }
    }
    pub async fn get_client(&self, id: Uuid) -> Option<Client> {
        self.clients.read().await.get(&id).cloned()
    }

    pub fn dispose(&self) {
        println!("Disposing hub");
    }
}
