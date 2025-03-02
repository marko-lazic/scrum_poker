use crate::{
    resources::Resources,
    service::{Service, ServiceContext},
};
use common::prelude::*;
use std::collections::HashMap;

pub struct Dispatch {
    services: HashMap<String, Box<dyn Service>>,
    resources: Resources,
}

impl Dispatch {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            resources: Resources::new(),
        }
    }

    pub fn add_service<S>(&mut self, name: impl Into<String>, service: S)
    where
        S: Service,
    {
        self.services.insert(name.into(), Box::new(service));
    }

    pub fn run(&mut self, request: RpcRequest) -> RpcResponse {
        let method = &request.method;
        let id = request.id;

        match self.services.get(method) {
            Some(service) => {
                // SAFETY: This is unsafe because we're creating a 'static reference
                // In a real implementation, we would need to ensure the lifetime
                // is properly managed or use a different approach.
                let resources = unsafe {
                    // This is a workaround for the 'static lifetime requirement
                    // In a real Bevy-like solution, we'd use a proper scheduler
                    std::mem::transmute::<&Resources, &'static Resources>(&self.resources)
                };

                let ctx = ServiceContext { resources, request };

                match service.call(ctx) {
                    Ok(response) => response,
                    Err(error) => RpcResponse {
                        result: None,
                        error: Some(error),
                        id,
                    },
                }
            }
            None => {
                // If not found in sync services, it's not found at all
                RpcResponse {
                    result: None,
                    error: Some(RpcError::new(
                        RpcErrorCode::MethodNotFound,
                        format!("Method '{}' not found", method),
                    )),
                    id,
                }
            }
        }
    }
}
