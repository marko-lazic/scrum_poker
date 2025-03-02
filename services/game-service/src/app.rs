use common::prelude::{RpcRequest, RpcResponse};

use crate::{
    dispatch::Dispatch,
    resources::{Resource, Resources},
    service::Service,
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
        self.dispatch.run(request)
    }
}
