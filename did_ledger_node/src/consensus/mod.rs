use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use self::types::CommitThresholder;

pub mod protocol;
pub mod types;

pub struct Consensus {
    commit_queue: Arc<Mutex<HashMap<u64, CommitThresholder>>>,
    local_addr: String,
}

impl Consensus {
    pub fn new(local_addr: String) -> Self {
        Consensus {
            commit_queue: Arc::new(Mutex::new(HashMap::<u64, CommitThresholder>::new())),
            local_addr,
        }
    }
}
