use std::sync::{Arc, Mutex};

use crate::{api_consumer::ApiConsumer, ThreadApiConsumer};

pub struct Consumer {
    pub agent: Arc<Mutex<ApiConsumer>>,
}
impl Consumer {
    pub fn new() -> Self {
        Consumer {
            agent: Arc::new(Mutex::new(ApiConsumer::new().unwrap())),
        }
    }
    pub fn new_agent(&self) -> ThreadApiConsumer {
        Arc::clone(&self.agent)
    }
}