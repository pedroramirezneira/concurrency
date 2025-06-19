use crate::structs::stats::Stats;
use std::sync::{Arc, Mutex};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub struct SharedState {
    pub stats: Arc<Mutex<Stats>>,
    pub semaphore: Arc<Semaphore>,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            stats: Arc::new(Mutex::new(Stats::new())),
            semaphore: Arc::new(Semaphore::new(4)),
        }
    }

    pub fn try_start_processing(&self) -> Option<OwnedSemaphorePermit> {
        self.semaphore.clone().try_acquire_owned().ok()
    }
}
