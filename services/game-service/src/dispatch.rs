use common::prelude::*;
use std::collections::HashMap;

type ServiceFn = fn(RpcRequest) -> Result<RpcResponse, RpcError>;

pub struct Dispatch {
    services: HashMap<String, ServiceFn>,
}

impl Dispatch {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn add_service(&mut self, name: impl Into<String>, service: ServiceFn) {
        self.services.insert(name.into(), service);
    }

    pub fn run(&self, request: RpcRequest) -> RpcResponse {
        let method = &request.method;
        let id = request.id;

        match self.services.get(method) {
            Some(service) => match service(request) {
                Ok(response) => response,
                Err(error) => RpcResponse {
                    result: None,
                    error: Some(error),
                    id,
                },
            },
            None => RpcResponse {
                result: None,
                error: Some(RpcError::new(
                    RpcErrorCode::MethodNotFound,
                    format!("Method '{}' not found", method),
                )),
                id,
            },
        }
    }
}
