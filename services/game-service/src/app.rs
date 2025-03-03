use std::cell::{Ref, RefMut};

use common::prelude::{RpcError, RpcErrorCode, RpcRequest, RpcResponse};

use crate::{
    dispatch::Dispatch,
    resources::{Resource, Resources},
    service::{Service, ServiceContext},
};

pub struct App {
    pub resources: Resources,
    pub dispatch: Dispatch,
}

impl App {
    pub fn new() -> Self {
        Self {
            resources: Resources::new(),
            dispatch: Dispatch::new(),
        }
    }

    pub fn add_service<S>(&mut self, name: impl Into<String>, service: S)
    where
        S: Service,
    {
        self.dispatch.add_service(name, service);
    }

    pub fn insert_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.resources.insert(resource);
        self
    }

    pub fn resource<T: Resource>(&self) -> Option<Ref<T>> {
        self.resources.get::<T>()
    }

    pub fn resource_mut<T: Resource>(&mut self) -> Option<RefMut<T>> {
        self.resources.get_mut::<T>()
    }

    pub fn has_resource<T: Resource>(&self) -> bool {
        self.resources.contains::<T>()
    }

    pub fn run(&mut self, request: RpcRequest) -> RpcResponse {
        let method = &request.method;
        let id = request.id;

        match self.dispatch.services.get(method) {
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
