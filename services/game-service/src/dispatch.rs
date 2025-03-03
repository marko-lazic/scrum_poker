use crate::service::Service;
use std::collections::HashMap;

pub struct Dispatch {
    pub services: HashMap<String, Box<dyn Service>>,
}

impl Dispatch {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn add_service<S>(&mut self, name: impl Into<String>, service: S)
    where
        S: Service,
    {
        self.services.insert(name.into(), Box::new(service));
    }
}
