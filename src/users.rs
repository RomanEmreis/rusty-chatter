use tokio::sync::{mpsc::{self, UnboundedSender}, RwLock};
use tungstenite::Message;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub(super) struct Users {
    pub(super) connections: Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>,
    next_id: AtomicUsize 
}

impl Default for Users {
    fn default() -> Self {
        Self { 
            next_id: AtomicUsize::new(1), 
            connections: Default::default()
        }
    }
}

impl Users {
    pub(super) async fn add(&self, tx: UnboundedSender<Message>) -> usize {
        let id = self.new_id();
        tracing::trace!("new user connected: {}", id);
        
        self.connections
            .write()
            .await
            .insert(id, tx);

        id
    }

    pub(super) async fn remove(&self, id: usize) {
        self.connections
            .write()
            .await
            .remove(&id);
    }

    fn new_id(&self) -> usize {
        self.next_id
            .fetch_add(1, Ordering::Relaxed)
    }
}