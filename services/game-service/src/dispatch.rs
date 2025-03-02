use crate::resources::{Resource, Resources};
use common::prelude::*;
use std::collections::HashMap;

// A service context containing resources
pub struct ServiceContext {
    pub resources: &'static Resources,
    pub request: RpcRequest,
}

// Trait for synchronous services
pub trait Service: Send + Sync + 'static {
    fn call(&self, ctx: ServiceContext) -> Result<RpcResponse, RpcError>;
}

// Implement Service for function pointers
impl<F> Service for F
where
    F: Fn(ServiceContext) -> Result<RpcResponse, RpcError> + Send + Sync + 'static,
{
    fn call(&self, ctx: ServiceContext) -> Result<RpcResponse, RpcError> {
        self(ctx)
    }
}

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

    // Bevy-like resource methods
    pub fn insert_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.resources.insert(resource);
        self
    }

    pub fn resource<T: Resource>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    pub fn resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn has_resource<T: Resource>(&self) -> bool {
        self.resources.contains::<T>()
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
