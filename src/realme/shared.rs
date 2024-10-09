use std::sync::{Arc, Mutex};

use super::Realme;

#[derive(Debug)]
pub struct SharedRealme {
    pub inner: Arc<Mutex<Realme>>,
}

impl SharedRealme {
    pub fn new(realme: Realme) -> Self {
        Self {
            inner: Arc::new(Mutex::new(realme)),
        }
    }
}
