use crate::structs::stats::Stats;
use std::sync::{Arc, RwLock};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub struct SharedState {
    pub stats: Arc<RwLock<Stats>>,
    pub semaphore: Arc<Semaphore>,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            stats: Arc::new(RwLock::new(Stats::new())),
            semaphore: Arc::new(Semaphore::new(4)),
        }
    }

    pub fn try_start_processing(&self) -> Option<OwnedSemaphorePermit> {
        self.semaphore.clone().try_acquire_owned().ok()
    }
}
